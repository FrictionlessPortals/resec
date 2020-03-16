//! # resec
//!
//! A library dedicated to reverse engineering techniques for the [SEC](https://examinations.ie).
//!
//! **Note**: This library can stop working at any time if a website change occurs!

mod consts;
pub mod error;
pub mod parser;
pub mod stages;

/// Convenience re-export of commonly used items.
///
/// # Usage:
///
/// ```
/// use resec::prelude::*;
/// ```
pub mod prelude {
    // SEC Prelude
    pub use crate::{
        consts::{EXAM_PAPER_YEARS, MARKING_SCHEME_YEARS},
        error::SecError,
        parser::*,
        stages::StageBuilder,
    };
}