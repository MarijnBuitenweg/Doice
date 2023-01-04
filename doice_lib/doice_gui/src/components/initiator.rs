use eframe::{
    egui::{DragValue, Id, Layout, Ui},
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
                ui.label(&self.name);
                if ui.small_button("-").clicked() {
                    self.remove = true;
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.text_edit_singleline(&mut self.note);
                    ui.label("note: ");
                });
            })
        });
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
}

impl Initiator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&mut self, ui: &mut Ui) {
        // Add stuff
        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.small_button("+").clicked() {
                    self.list.push(self.pre_item.clone());
                    self.pre_item.id += 1;
                }

                ui.add(DragValue::new(&mut self.pre_item.initiative));
                ui.label("name: ");
                ui.text_edit_singleline(&mut self.pre_item.name);
                ui.label("note: ");
                ui.text_edit_singleline(&mut self.pre_item.note);
            });
        });
        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Sort").clicked() {
                    self.list.sort_by_key(|item| item.initiative);
                    self.list.reverse();
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
