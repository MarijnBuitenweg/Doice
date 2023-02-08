// Unstable features
#![feature(associated_type_bounds)]
#![feature(is_some_and)]
#![feature(option_get_or_insert_default)]
// Allows
#![allow(dead_code)]

/// A rudimentary application window that can run an activity
#[cfg(feature = "eframe")]
mod activity_host;
/// The main application UI for doiceOS, DEPRECATED
mod application;
/// Some ui components
pub mod components;
mod show_trait;
/// New ui, tailored for ease-of-use
#[cfg(feature = "eframe")]
mod tailored_frames;

// The only things that need to be accessed from the outside
/// Basic application that can run a single activity
#[cfg(feature = "eframe")]
pub use activity_host::ActivityHost;
pub use application::Activity;
pub use application::DCtx;
/// Main application
#[cfg(feature = "eframe")]
pub use application::DoiceApp;
pub use dnd_data;
#[cfg(feature = "eframe")]
pub use eframe;
pub use show_trait::DoiceShow;
