use std::sync::Arc;

use doice_gui::AppData;

use crate::activities::{CharacterManager, GlobalAnalyzer, Notes};

pub struct TailoredUI {
    /// Global application data
    data: Arc<AppData>,
    // Components
    roller: GlobalAnalyzer,
    manager: CharacterManager,
    notes: Notes,
}
