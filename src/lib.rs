//! It is a on-going crate that provides basic matrix manipunations and ports and python's [scipy-sklearn's matrix decomposition](https://scikit-learn.org/stable/modules/decomposition.html) to pure rust
//!
#![allow(missing_docs)]
#![warn(missing_debug_implementations)]

extern crate matrixmultiply;
extern crate num as libnum;

pub mod error;
pub mod matrix;
pub mod norm;
pub mod utils;
pub mod vector;

mod internal_utils;
