use std::{collections::BTreeMap, fmt::Write, time::Duration};

use instant::Instant;

use eframe::{
    egui::epaint::{text::LayoutJob, Color32},
    egui::{
        plot::{Bar, BarChart, Plot, VLine},
        Context, DragValue, Key, Layout, Modifiers, RichText, Ui,
    },
};

use {
    doice_roller::{DiceError, Layouter, ProbDist, Roll, RollOut, Rollable, SampleDist},
    doice_utils::ParExecutor,
};

use super::{
    dice_docs::dice_docs,
    dice_history::{DiceHistory, DiceHistoryEntry},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum CurrentPanel {
    Plot,
    History,
    Help,
}

impl CurrentPanel {
    pub fn cycle_left(&mut self) {
        *self = match *self {
            CurrentPanel::Plot => CurrentPanel::Plot,
            CurrentPanel::History => CurrentPanel::Plot,
            CurrentPanel::Help => CurrentPanel::History,
        }
    }

    pub fn cycle_right(&mut self) {
        *self = match *self {
            CurrentPanel::Plot => CurrentPanel::History,
            CurrentPanel::History => CurrentPanel::Help,
            CurrentPanel::Help => CurrentPanel::Help,
        }
    }
}

impl Default for CurrentPanel {
    fn default() -> Self {
        CurrentPanel::Plot
    }
}

#[derive(Default)]
pub struct DiceGrapher<const EXP_UPDATE: u64 = 100> {
    bars: Vec<Bar>,
    roll: Roll,
    roll_txt: String,
    dist_gen: ParExecutor<ProbDist>,
    ctx: Context,
    loading: bool,
    current_dist: ProbDist,
    display_error: Option<String>,
    avg: f64,
    variance: f64,
    cumulative: BTreeMap<isize, f64>,
    res: Option<RollOut>,
    success_chance: f64,
    aspect_rat: f32,
    peak: f64,
    current_panel: CurrentPanel,
    // DC setting
    dc_on: bool,
    dc_val: isize,
    // History
    history: DiceHistory,
    // Experimental result stuff
    exp_bars: Vec<Bar>,
    exp_dist: ProbDist,
    exp_samples: SampleDist,
    exp_exec: ParExecutor<(Box<[isize]>, Roll)>,
    exp_run: bool,
}

impl<const EXP_UPDATE: u64> DiceGrapher<EXP_UPDATE> {
    pub fn new(ctx: Context) -> Self {
        let extra_ctx = ctx.clone();
        let exp_ctx = ctx.clone();
        Self {
            dist_gen: ParExecutor::with_notifyer(move || extra_ctx.request_repaint()),
            exp_exec: ParExecutor::with_notifyer(move || exp_ctx.request_repaint()),
            ctx,
            ..Default::default()
        }
    }

    fn remake_bars(&mut self) {
        self.bars = self
            .current_dist
            .iter()
            .step_by(1 + self.current_dist.len() / 512)
            .map(|(outcome, prob)| Bar::new(*outcome as f64, prob.abs()))
            .collect();
    }

    fn remake_exp_bars(&mut self) {
        let threshold = self.exp_dist.peak().unwrap_or((1, 1.0)).1 / 1000.0;
        self.exp_dist.retain(|_, prob| *prob > threshold);
        self.exp_bars = self
            .exp_dist
            .iter()
            .step_by(1 + self.current_dist.len() / 512)
            .map(|(outcome, prob)| {
                Bar::new(*outcome as f64, prob.abs())
                    .fill(Color32::GOLD)
                    .width(0.4)
            })
            .collect()
    }

    fn recalc_aspect(&mut self) {
        let width = self
            .current_dist
            .keys()
            .max()
            .unwrap_or(&1)
            .abs_diff(*self.current_dist.keys().min().unwrap_or(&-1)) as f32;
        let height = self
            .current_dist
            .values()
            .copied()
            .reduce(f64::max)
            .unwrap_or(1.0f64)
            .clamp(0.0f64, f64::MAX);

        self.peak = height;
        self.aspect_rat = width.max(0.5) / height as f32;
    }

    pub fn set_dc(&mut self, dc: Option<isize>) {
        self.dc_on = dc.is_some();
        if let Some(val) = dc {
            self.dc_val = val;
            self.refresh_dc();
        }
    }

    pub fn display_roll<T: TryInto<Roll, Error: Into<DiceError>>>(&mut self, into_roll: T) {
        match into_roll.try_into() {
            Ok(roll) => {
                self.display_error = None;
                self.roll = roll.clone();
                self.loading = true;
                self.dist_gen.process_into(roll).keep_notifier();
                self.exp_bars.clear();
                self.res = None;
                self.exp_dist.clear();
                self.exp_samples.clear();
                self.exp_exec.clear_tasks();
                if self.exp_run {
                    self.init_experiment();
                }
            }
            Err(err) => {
                // Convert the mystery error into a DiceError, and then into a string
                self.display_error = Some(err.into().into());
            }
        }
    }

    pub fn roll(&mut self) -> RollOut {
        let mut res = self.roll.roll();
        if res.txt.sections.len() > 100 {
            res.txt = Layouter::default();
            res.txt.append("[...]");
        }
        if self.dc_on {
            if self.dc_val <= res.value {
                res.txt.append(" = ");
                res.txt
                    .append_colored(&res.value.to_string(), Color32::GREEN);
            } else {
                res.txt.append(" = ");
                res.txt.append_colored(&res.value.to_string(), Color32::RED);
            }
        } else {
            res.txt.append(&format!(" = {}", res.value));
        }

        self.history.add_entry(DiceHistoryEntry::new(
            self.roll.clone(),
            self.roll_txt.clone(),
            res.clone(),
            self.avg,
            self.variance,
        ));

        self.res = Some(res.clone());
        res
    }

    fn experiment_helper(roll: Roll) -> (Box<[isize]>, Roll) {
        let mut out = Vec::new();
        let ts = Instant::now();
        let max_duration = Duration::from_millis(EXP_UPDATE);
        while ts.elapsed() < max_duration {
            out.push(roll.roll().value);
        }
        (Box::from(out), roll)
    }

    fn init_experiment(&mut self) {
        self.exp_exec
            .process_with(self.roll.clone(), Self::experiment_helper)
            .keep_notifier();
        self.ctx.request_repaint();
    }

    pub fn run_experiment(&mut self, do_run: bool) {
        if do_run && !self.exp_run {
            self.init_experiment();
        }
        self.exp_run = do_run;
    }

    fn show_chart(&mut self, ui: &mut Ui) {
        let chart = if !self.loading {
            BarChart::new(self.bars.clone())
                .color(Color32::LIGHT_BLUE)
                .name("Probability Distribution")
        } else {
            BarChart::new(Vec::new())
                .color(Color32::LIGHT_BLUE)
                .name("Probability Distribution")
        };

        Plot::new("Roll Analyzer")
            .data_aspect(self.aspect_rat)
            .view_aspect(1.0)
            .show(ui, |ui| {
                ui.bar_chart(chart);

                if !self.exp_bars.is_empty() {
                    ui.bar_chart(BarChart::new(self.exp_bars.clone()).name("Experimental results"));
                }

                // If there is a DC, show it
                if self.dc_on {
                    ui.vline(
                        VLine::new(self.dc_val as f64)
                            .name("DC")
                            .color(Color32::DARK_GRAY)
                            .width(2.5),
                    );
                }

                // If there is a roll result, show it too
                if let Some(res) = &self.res {
                    let clr = if self.dc_on {
                        if self.dc_val <= res.value {
                            Color32::GREEN
                        } else {
                            Color32::RED
                        }
                    } else {
                        Color32::GOLD
                    };
                    ui.vline(VLine::new(res.value as f64).name("Roll Result").color(clr));
                }
            });

        ui.vertical_centered(|ui| {
            if self.loading {
                ui.spinner();
            } else {
                ui.label(match &self.display_error {
                    // If there was an error during parsing, display that
                    Some(err) => RichText::new(err).color(Color32::RED),
                    // Otherwise show info about the roll
                    None => RichText::new({
                        let mut info_text = format!(
                            "average = {:.3};\tdeviation = {:.3}",
                            self.avg,
                            self.variance.sqrt()
                        );

                        if self.dc_on {
                            write!(info_text, ";\tsuccess = {:.2}%", self.success_chance).unwrap();
                        }

                        info_text
                    }),
                });
            }
            if let Some(res) = &self.res {
                ui.label(LayoutJob::from(res.txt.clone()));
            }
        });
    }

    fn handle_dist_gen(&mut self) {
        // If new dist is available
        if let Some(dist) = self.dist_gen.try_get_data() {
            // Stop loading and make new plot
            self.loading = false;
            self.current_dist = dist;
            self.recalc_aspect();
            let threshold = self.peak / 1000.0;
            self.current_dist.retain(|_, prob| *prob > threshold);
            self.recalc_aspect();
            self.remake_bars();
            self.avg = self.current_dist.expectation();
            self.cumulative = self.current_dist.get_cumulative_prob();
            self.variance = self.current_dist.var();
            self.refresh_dc();
        }
    }

    fn refresh_dc(&mut self) {
        // Find the last entry that is < dc
        let e = self.cumulative.iter().filter(|e| *e.0 < self.dc_val).last();
        match e {
            // If there are entries that give a chance of failing
            Some(entry) => {
                // Calculate the chance of success from the chance to fail
                self.success_chance = (1.0 - entry.1) * 100.0;
            }
            None => {
                // Otherwise, there can be no failure
                self.success_chance = 100.0;
            }
        }
    }

    fn handle_experiment(&mut self) {
        // If new experimental data is available, process it
        if let Some((samples, roll)) = self.exp_exec.try_get_data() {
            self.exp_samples.add_samples(&samples);
            self.exp_dist.read_samples(&self.exp_samples);
            self.remake_exp_bars();
            // If the experiment should still be running, launch new batch
            if self.exp_run {
                self.exp_exec
                    .process_with(roll, Self::experiment_helper)
                    .keep_notifier();
            }
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        self.handle_dist_gen();
        self.handle_experiment();

        if ui.input_mut().consume_key(Modifiers::NONE, Key::PageUp) {
            self.current_panel.cycle_left();
        }

        if ui.input_mut().consume_key(Modifiers::NONE, Key::PageDown) {
            self.current_panel.cycle_right();
        }

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_panel, CurrentPanel::Plot, "Plot");
            ui.selectable_value(&mut self.current_panel, CurrentPanel::History, "History");
            ui.selectable_value(&mut self.current_panel, CurrentPanel::Help, "Help");

            ui.with_layout(Layout::right_to_left(), |ui| {
                let prev_checked = self.dc_on;
                ui.checkbox(&mut self.dc_on, "");
                //let txt_response = ui.add(TextEdit::singleline(&mut self.dc_text).desired_width(20.0));
                let val_response = ui.add(DragValue::new(&mut self.dc_val));
                ui.label("DC:");

                if self.dc_on && (!prev_checked || val_response.changed()) {
                    self.refresh_dc();
                }
            })
        });

        match self.current_panel {
            CurrentPanel::Plot => self.show_chart(ui),
            CurrentPanel::History => self.history.show(ui),
            CurrentPanel::Help => dice_docs(ui),
        }
    }
}

impl Clone for DiceGrapher {
    fn clone(&self) -> Self {
        // Thank god Context is cheap to clone
        let extra_ctx = self.ctx.clone();
        let exp_ctx = self.ctx.clone();
        Self {
            bars: self.bars.clone(),
            roll: self.roll.clone(),
            roll_txt: self.roll_txt.clone(),
            dist_gen: ParExecutor::with_notifyer(move || extra_ctx.request_repaint()),
            ctx: self.ctx.clone(),
            loading: self.loading,
            current_dist: self.current_dist.clone(),
            display_error: self.display_error.clone(),
            avg: self.avg,
            variance: self.variance,
            cumulative: self.cumulative.clone(),
            res: self.res.clone(),
            dc_on: self.dc_on,
            success_chance: self.success_chance,
            aspect_rat: self.aspect_rat,
            peak: self.peak,
            current_panel: self.current_panel,
            // DC setting
            dc_val: self.dc_val,
            // History
            history: self.history.clone(),
            // Experiment stuff
            exp_dist: self.exp_dist.clone(),
            exp_samples: self.exp_samples.clone(),
            exp_bars: self.exp_bars.clone(),
            exp_exec: ParExecutor::with_notifyer(move || exp_ctx.request_repaint()),
            exp_run: self.exp_run,
        }
    }
}
