// Unstable features
#![feature(associated_type_bounds)]
#![feature(is_some_with)]
// Allows
#![allow(dead_code)]

/// A rudimentary application window that can run an activity
#[cfg(feature = "eframe")]
mod activity_host;
/// The main application UI
mod application;
/// Some ui components
pub mod components;
mod show_trait;

// The only things that need to be accessed from the outside
/// Basic application that can run a single activity
#[cfg(feature = "eframe")]
pub use activity_host::ActivityHost;
pub use application::Activity;
pub use application::DCtx;
pub use show_trait::DoiceShow;
/// Main application
#[cfg(feature = "eframe")]
pub use application::DoiceApp;
#[cfg(feature = "eframe")]
pub use eframe;
pub use dnd_data;