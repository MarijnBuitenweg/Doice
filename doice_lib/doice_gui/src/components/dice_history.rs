use instant::{Duration, Instant};

use eframe::egui::epaint::{text::LayoutJob, Color32};
use eframe::egui::{Layout, ScrollArea, TextStyle, Ui};

use doice_roller::{Roll, RollOut};

#[derive(Clone, Default)]
pub struct DiceHistory {
    entries: Vec<DiceHistoryEntry>,
}

impl DiceHistory {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_entry(&mut self, entry: DiceHistoryEntry) {
        self.entries.push(entry);
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.set_height(ui.available_width());
            let row_height = ui.text_style_height(&TextStyle::Body) * 2.1;
            ScrollArea::vertical()
                .always_show_scroll(true)
                .stick_to_bottom()
                .auto_shrink([false; 2])
                .show_rows(ui, row_height, self.entries.len(), |ui, rows| {
                    for entry in self.entries[rows].iter_mut() {
                        ui.group(|ui| entry.show(ui));
                    }
                });
        });
        ui.vertical_centered(|ui| {
            let luck: f64 = if self.entries.is_empty() {
                0.0f64
            } else {
                let weight = 1.0f64 / (self.entries.len() as f64);
                self.entries
                    .iter()
                    .map(|entry| entry.luck() * weight)
                    .filter(|w_luck| w_luck.is_normal())
                    .sum()
            };

            ui.colored_label(
                match luck.total_cmp(&0.0f64) {
                    std::cmp::Ordering::Less => Color32::RED,
                    std::cmp::Ordering::Equal => Color32::GRAY,
                    std::cmp::Ordering::Greater => Color32::GREEN,
                },
                format!("Session Luck: {:.3}", luck),
            );
        });
    }
}

#[derive(Clone)]
pub struct DiceHistoryEntry {
    roll: Roll,
    roll_txt: String,
    result: RollOut,
    avg: f64,
    variance: f64,
    ts: Instant,
}

impl DiceHistoryEntry {
    pub fn new(roll: Roll, roll_txt: String, result: RollOut, avg: f64, variance: f64) -> Self {
        Self {
            roll,
            roll_txt,
            result,
            avg,
            variance,
            ts: Instant::now(),
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.label(&self.roll_txt);
            ui.label(" -> ");
            ui.label(LayoutJob::from(self.result.txt.clone()));

            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.label(format!("{} ago", fmt_duration(&self.ts.elapsed())));
            });
        });
    }

    pub fn src(&self) -> &str {
        &self.roll_txt
    }

    pub fn result(&self) -> &RollOut {
        &self.result
    }

    pub fn avg(&self) -> f64 {
        self.avg
    }

    pub fn variance(&self) -> f64 {
        self.variance
    }

    /// Returns the amount of time since the roll
    pub fn elapsed(&self) -> Duration {
        self.ts.elapsed()
    }

    /// Gives an indication of how lucky/unlucky a roll was.
    /// An output of -1 means the roll was 1 deviation below the mean
    /// An output of 1 means the roll was 1 deviation above the mean
    pub fn luck(&self) -> f64 {
        (self.result.value as f64 - self.avg) / self.variance.sqrt()
    }
}

fn fmt_duration(duration: &Duration) -> String {
    let seconds = duration.as_secs();
    if seconds < 60 {
        format!("{}s", seconds)
    } else {
        let minutes = seconds / 60;
        format!("{} min", minutes)
    }
}
