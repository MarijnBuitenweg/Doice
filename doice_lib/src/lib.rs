/// Contains all activities
pub mod activities;
/// Contains UI setup tailored for ease of use.
/// It is here instead of in doice_gui, because it is not component agnostic like the other ui layouts (and as such depends on the specific ui components).
mod tailored_frames;
pub use tailored_frames::TailoredUI;
