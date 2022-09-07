#![feature(test)]
#![feature(associated_type_bounds)]

mod par_exec;
pub use par_exec::ParExecutor;

mod search;
pub use search::{Named, Search};
pub use search::ParSearch;
mod tuple_swap;
pub use tuple_swap::tup_swap;
