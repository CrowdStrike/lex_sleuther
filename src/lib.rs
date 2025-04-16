// in case the caller wants to use the APIs that allow you to provide your own weights and classifications
pub use ndarray;

// reexport all the model code, including the baked model if that feature is enabled
mod model;
pub use model::*;