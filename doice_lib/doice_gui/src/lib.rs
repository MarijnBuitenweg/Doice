// Unstable features
#![feature(associated_type_bounds)]
#![feature(option_get_or_insert_default)]
// Allows
#![allow(dead_code)]

/// The main application UI for doiceOS, DEPRECATED
mod application;
/// Some ui components
pub mod components;
mod show_trait;

// The only things that need to be accessed from the outside
/// Basic application that can run a single activity
#[cfg(feature = "eframe")]
pub use application::Activity;
pub use application::AppData;
pub use application::DCtx;
/// Main application
#[cfg(feature = "eframe")]
pub use application::DoiceApp;
pub use dnd_data;
#[cfg(feature = "eframe")]
pub use eframe;
pub use show_trait::DoiceShow;
