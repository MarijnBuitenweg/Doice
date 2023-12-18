use eframe::{
    egui::{DragValue, Id, Key, Layout, TextEdit, Ui},
    emath::Align,
};
use egui_dnd::{utils::shift_vec, DragDropItem, DragDropUi};

#[derive(Debug, Default, Clone)]
struct Item {
    id: usize,
    name: String,
    initiative: isize,
    note: String,
    remove: bool,
}

impl Item {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.add(DragValue::new(&mut self.initiative));
                    if ui.small_button("-").clicked() {
                        self.remove = true;
                    }
                    ui.label(&self.name);
                    ui.with_layout(Layout::right_to_left(Align::Center), |_ui| {
                        //ui.small_button(">")
                    });

                    // ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    //     if !self.expandero {
                    //         if ui.small_button("\\/").clicked() {
                    //             self.expandero = true;
                    //         }
                    //     } else if ui.small_button("/\\").clicked() {
                    //         self.expandero = false;
                    //     }
                    //     // let id = ui.auto_id_with(self.id);
                    //     // CollapsingState::load_with_default_open(ui.ctx(), id, false)
                    //     //     .show_header(ui, |ui| ui.label(""))
                    //     //     .body_unindented(|ui| {
                    //     //         ui.with_layout(Layout::top_down(Align::Min), |ui| {
                    //     //             ui.label("");
                    //     //             ui.group(|ui| {
                    //     //                 ui.label("Uno");
                    //     //                 ui.label("Duo");
                    //     //             })
                    //     //         })
                    //     //     });
                    // });
                });
                // if self.expandero {
                //     ui.separator();
                //     ui.label("NOTEN");
                // }
            });
        });
    }

    pub fn reset(&mut self) {
        *self = Item {
            id: self.id,
            ..Default::default()
        };
    }
}

impl DragDropItem for Item {
    fn id(&self) -> Id {
        Id::new(self.id)
    }
}

#[derive(Default, Clone)]
pub struct Initiator {
    pre_item: Item,
    innit_text: String,
    list: Vec<Item>,
    dnd: DragDropUi,
    current: Option<isize>,
    clear_confirm: bool,
}

impl Initiator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show_flex(&mut self, ui: &mut Ui) {
        // Entry adder
        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.small_button("+").clicked() {
                    self.list.push(self.pre_item.clone());
                    self.pre_item.id += 1;
                    self.innit_text.clear();
                    self.pre_item.reset();
                }

                if !self.innit_text.is_empty() {
                    self.innit_text = self.pre_item.initiative.to_string();
                }
                let innit = ui.add(
                    TextEdit::singleline(&mut self.innit_text)
                        .char_limit(3)
                        .desired_width(20.0),
                );
                if let Ok(num) = self.innit_text.trim().parse() {
                    self.pre_item.initiative = num;
                }
                ui.label("name: ");
                let name = ui.text_edit_singleline(&mut self.pre_item.name);

                if (innit.lost_focus() || name.lost_focus())
                    && ui.input(|i| i.key_pressed(Key::Enter))
                {
                    self.list.push(self.pre_item.clone());
                    self.pre_item.id += 1;
                    self.pre_item.reset();
                    self.innit_text.clear();
                    innit.request_focus();
                }

                // if innit.gained_focus() || name.gained_focus() || note.gained_focus() {
                //     ui.input_mut().events.push(Event::Key {
                //         key: Key::A,
                //         pressed: true,
                //         modifiers: Modifiers::CTRL,
                //     });
                // }
            });
        });

        // List control
        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Sort").clicked() {
                    self.list.sort_by_key(|item| -item.initiative);
                    //self.list.reverse();
                }
                if ui.button("Next").clicked() {
                    self.current = if let Some(i) = self.current {
                        self.list
                            .iter()
                            .filter(|e| e.initiative < i)
                            .max_by_key(|e| e.initiative)
                            .map(|e| e.initiative)
                    } else {
                        self.list
                            .iter()
                            .max_by_key(|e| e.initiative)
                            .map(|e| e.initiative)
                    };
                }
                if self.clear_confirm {
                    let sure = ui.button("Sure?");
                    if sure.clicked() {
                        self.list.clear();
                        self.clear_confirm = false;
                    } else if sure.clicked_elsewhere() {
                        self.clear_confirm = false;
                    }
                } else {
                    self.clear_confirm = ui.button("Clear").clicked();
                }
                if self.current.is_some() {
                    ui.label("current initiative: ");
                    ui.add(DragValue::new(self.current.get_or_insert_default()));
                }
            });
        });

        // The list
        let response = self
            .dnd
            .ui::<Item>(ui, self.list.iter_mut(), |item, ui, handle| {
                ui.horizontal(|ui| {
                    handle.ui(ui, item, |ui| {
                        ui.label("::");
                    });
                    item.show(ui);
                });
            });

        if let Some(response) = response.completed {
            shift_vec(response.from, response.to, &mut self.list);
        }

        self.list.retain(|e| !e.remove);
    }

    pub fn show(&mut self, ui: &mut Ui) {
        // Entry adder
        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.small_button("+").clicked() {
                    self.list.push(self.pre_item.clone());
                    self.pre_item.id += 1;
                    self.pre_item.reset();
                }

                let innit = ui.add(DragValue::new(&mut self.pre_item.initiative));
                ui.label("name: ");
                let name = ui.text_edit_singleline(&mut self.pre_item.name);
                ui.label("note: ");
                let note = ui.text_edit_singleline(&mut self.pre_item.note);

                if (innit.lost_focus() || name.lost_focus() || note.lost_focus())
                    && ui.input(|i| i.key_pressed(Key::Enter))
                {
                    self.list.push(self.pre_item.clone());
                    self.pre_item.id += 1;
                    self.pre_item.reset();
                    innit.request_focus();
                }

                // if innit.gained_focus() || name.gained_focus() || note.gained_focus() {
                //     ui.input_mut().events.push(Event::Key {
                //         key: Key::A,
                //         pressed: true,
                //         modifiers: Modifiers::CTRL,
                //     });
                // }
            });
        });

        // List control
        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Sort").clicked() {
                    self.list.sort_by_key(|item| -item.initiative);
                    //self.list.reverse();
                }
                if ui.button("Next").clicked() {
                    self.current = if let Some(i) = self.current {
                        self.list
                            .iter()
                            .filter(|e| e.initiative < i)
                            .max_by_key(|e| e.initiative)
                            .map(|e| e.initiative)
                    } else {
                        self.list
                            .iter()
                            .max_by_key(|e| e.initiative)
                            .map(|e| e.initiative)
                    };
                }
                if self.clear_confirm {
                    let sure = ui.button("Sure?");
                    if sure.clicked() {
                        self.list.clear();
                        self.clear_confirm = false;
                    } else if sure.clicked_elsewhere() {
                        self.clear_confirm = false;
                    }
                } else {
                    self.clear_confirm = ui.button("Clear").clicked();
                }
                if self.current.is_some() {
                    ui.label("current initiative: ");
                    ui.add(DragValue::new(self.current.get_or_insert_default()));
                }
            });
        });

        // The list
        let response = self
            .dnd
            .ui::<Item>(ui, self.list.iter_mut(), |item, ui, handle| {
                ui.horizontal(|ui| {
                    handle.ui(ui, item, |ui| {
                        ui.label("::");
                    });
                    item.show(ui);
                });
            });

        if let Some(response) = response.completed {
            shift_vec(response.from, response.to, &mut self.list);
        }

        self.list.retain(|e| !e.remove);
    }
}
