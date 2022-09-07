use doice_legacy::{Roller};

use doice_gui::{Activity, DCtx};
use egui::{
    {Key, Layout, ScrollArea},
    emath::Align,
};

/// Contains all data about a previous roll
#[derive(Clone)]
struct RollHistoryEntry {
    /// Total result of the roll
    total: i32,
    /// The symbolic representation of the roll, that can easily be rerolled.
    /// Currently unused
    _roll: Roller,
    /// Text that was input by the user to get the roll
    roll_txt: String,
    /// Tooltip that is used to display the roll's results in more detail
    tooltip: String,
}

/// Activity that rolls dice using the old TUI dicerolling backend
#[derive(Default, Clone)]
pub struct LegacyDiceRoller {
    /// Buffer for current input
    crnt_command: String,
    /// All previous rolls
    roll_history: Vec<RollHistoryEntry>,
    /// Currently selected entry from the history.
    /// 0 refers to the last entry, 1 to the second-to-last, etc.
    history_i: usize,
    /// Does the activity have focus?
    focus: bool,
}

impl Activity for LegacyDiceRoller {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &mut DCtx) {
        ui.with_layout(Layout::top_down(Align::Min), |ui| {
            // Width of this scrollarea does not fill the entire width of the window for some reason
            ScrollArea::vertical()
                .max_height(300.0)
                .auto_shrink([false, true])
                .stick_to_bottom()
                .show(ui, |ui| {
                    for e in self.roll_history.iter() {
                        ui.horizontal(|ui| {
                            ui.label(&e.roll_txt);
                            ui.label("\t->\t");
                            ui.label(e.total.to_string()).on_hover_text(&e.tooltip);
                        });
                    }
                });

            let command_confirm = ui.text_edit_singleline(&mut self.crnt_command);
            // If focus is lost and enter was just pressed, the user confirmed input
            if command_confirm.lost_focus() && ui.input().key_pressed(Key::Enter) {
                // Reset the history pointer
                self.history_i = 0;
                // Set focus to the command bar again, so the user can immediately continue typing
                command_confirm.request_focus();
                // Try to interpret user input
                match Roller::from_text(&self.crnt_command) {
                    // If input was comprehensible
                    Ok(r) => {
                        // Roll the dice, and store the result
                        let out = r.roll_text(&mut rand::thread_rng());
                        // Add roll to history
                        let entry = RollHistoryEntry {
                            total: out.1,
                            _roll: r,
                            roll_txt: self.crnt_command.clone(),
                            tooltip: out.0,
                        };
                        self.roll_history.push(entry);
                    }
                    // If input could not be used
                    Err(s) => {
                        let entry = RollHistoryEntry {
                            total: 404,
                            _roll: Roller::default(),
                            roll_txt: String::from(s),
                            tooltip: String::from("Result not found."),
                        };
                        self.roll_history.push(entry);
                    }
                }

                self.crnt_command.clear();
            }

            if command_confirm.has_focus() {
                if ui.input().key_pressed(Key::ArrowUp) {
                    let i = self.roll_history.len() - 1 - self.history_i;
                    self.crnt_command = self.roll_history[i].roll_txt.clone();

                    // Increment history pointer if there is yet more to remember
                    if !self.roll_history.is_empty() && self.history_i < self.roll_history.len() - 1
                    {
                        self.history_i += 1;
                    }
                }

                if ui.input().key_pressed(Key::ArrowDown) {
                    //println!("DEC: i: {}, len: {}", self.history_i, self.roll_history.len());
                    let i = self.roll_history.len() - 1 - self.history_i;
                    self.crnt_command = self.roll_history[i].roll_txt.clone();

                    // Decrement history pointer if this does not make it negative
                    if self.history_i > 0 {
                        self.history_i -= 1;
                    } else {
                        self.crnt_command.clear();
                    }
                }
            }

            if self.focus {
                //ui.label("F O C U S");
                command_confirm.request_focus();
            }
        });

        // // Roll history and command bar should be layed out vertically
        // ui.vertical(|ui| {
        //     egui::TopBottomPanel::top("Rollhistorypanel").resizable(true).show_inside(ui, |ui| {
        //         ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
        //             // Width of this scrollarea does not fill the entire width of the window for some reason
        //             ScrollArea::vertical().stick_to_bottom().show(ui, |ui| {
        //                 for e in self.roll_history.iter() {
        //                     ui.horizontal(|ui| {
        //                         ui.label(&e.roll_txt);
        //                         ui.label("\t->\t");
        //                         ui.label(e.total.to_string()).on_hover_text(&e.tooltip);
        //                     });
        //                 }
        //             });
        //         });
        //     });

        //     // Command bar
        //     let command_confirm = ui.text_edit_singleline(&mut self.crnt_command);
        //     // If focus is lost and enter was just pressed, the user confirmed input
        //     if command_confirm.lost_focus() && ui.input().key_pressed(Key::Enter) {
        //         // Reset the history pointer
        //         self.history_i = 0;
        //         // Set focus to the command bar again, so the user can immediately continue typing
        //         command_confirm.request_focus();
        //         // Try to interpret user input
        //         match dice_rolls::Roller::from_text(&self.crnt_command) {
        //             // If input was comprehensible
        //             Ok(r) => {
        //                 // Roll the dice, and store the result
        //                 let out = r.roll_text(&mut rand::thread_rng());
        //                 // Add roll to history
        //                 let entry = RollHistoryEntry {
        //                     total: out.1,
        //                     roll: r,
        //                     roll_txt: self.crnt_command.clone(),
        //                     tooltip: out.0,
        //                 };
        //                 self.roll_history.push(entry);
        //             },
        //             // If input could not be used
        //             Err(s) => {
        //                 let entry = RollHistoryEntry {
        //                     total: 404,
        //                     roll: Roller::default(),
        //                     roll_txt: String::from(s),
        //                     tooltip: String::from("Result not found."),
        //                 };
        //                 self.roll_history.push(entry);
        //             }
        //         }

        //         self.crnt_command.clear();
        //     }

        //     // Cycle through history
        // if command_confirm.has_focus() {
        //     if ui.input().key_pressed(Key::ArrowUp) {
        //         let i = self.roll_history.len() - 1 - self.history_i;
        //         self.crnt_command = self.roll_history[i].roll_txt.clone();

        //         // Increment history pointer if there is yet more to remember
        //         if self.history_i < self.roll_history.len() - 1 {
        //             self.history_i += 1;
        //         }
        //     }

        //     if ui.input().key_pressed(Key::ArrowDown) {
        //         //println!("DEC: i: {}, len: {}", self.history_i, self.roll_history.len());
        //         let i = self.roll_history.len() - 1 - self.history_i;
        //         self.crnt_command = self.roll_history[i].roll_txt.clone();

        //         // Decrement history pointer if this does not make it negative
        //         if self.history_i > 0 {
        //             self.history_i -= 1;
        //         }
        //     }
        // }
        // });
    }

    fn name(&self) -> &'static str {
        "Legacy Dice Roller"
    }

    fn focus(&mut self) {
        self.focus = true;
    }

    fn lose_focus(&mut self) {
        self.focus = false;
    }

    fn resize(&self) -> bool {
        true
    }

    fn category(&self) -> &str {
        "Legacy"
    }
}
