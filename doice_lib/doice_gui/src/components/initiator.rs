use eframe::{
    egui::{self, DragValue, Id, Key, Layout, Ui},
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
            ui.horizontal_wrapped(|ui| {
                ui.add(DragValue::new(&mut self.initiative));
                if ui.small_button("-").clicked() {
                    self.remove = true;
                }
                ui.label(&self.name);

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.text_edit_singleline(&mut self.note);
                    ui.label("note: ");
                });
            })
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
    list: Vec<Item>,
    dnd: DragDropUi,
    current: Option<isize>,
    clear_confirm: bool,
}

impl Initiator {
    pub fn new() -> Self {
        Self::default()
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

    pub fn show_flex(&mut self, ui: &mut Ui) {
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
