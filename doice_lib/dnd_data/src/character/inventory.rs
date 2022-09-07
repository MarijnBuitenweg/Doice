use crate::item::Item;

pub struct InventoryItem<'a> {
    base_item: Option<&'a Item>,
    name: String,
    count: u32,
    unit_weight: Option<f32>,
    extra_info: String,
}
