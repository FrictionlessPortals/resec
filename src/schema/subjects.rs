//! The subjects schema for ``resec``.
//!
//! **NOTE**: All values in this schema have been reverse
//! engineered. They could be subject to change in the future!

use serde::{Serialize, Deserialize};
use strum_macros::{EnumIter, EnumProperty};
use resec_macros::make_schema;

// Form the schema using a macro.
make_schema!("schema/schema.json");