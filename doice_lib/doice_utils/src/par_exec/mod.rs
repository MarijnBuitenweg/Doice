mod proc_config;
#[cfg(feature = "rayon")]
mod actually_par;
#[cfg(feature = "rayon")]
pub use actually_par::ParExecutor;
#[cfg(not(feature = "rayon"))]
mod ser_exec;
#[cfg(not(feature = "rayon"))]
pub use ser_exec::ParExecutor;

type Notifier = Box<dyn FnMut() + Send>;

