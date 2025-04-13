// Arithmetic
#![deny(arithmetic_overflow)] // Prevent operations that would cause integer overflow
#![deny(clippy::checked_conversions)] // Suggest using checked conversions between numeric types
#![deny(clippy::cast_possible_truncation)] // Detect when casting might truncate a value
#![deny(clippy::cast_sign_loss)] // Detect when casting might lose sign information
#![deny(clippy::cast_possible_wrap)] // Detect when casting might cause value to wrap around
#![deny(clippy::cast_precision_loss)] // Detect when casting might lose precision
#![deny(clippy::integer_division)] // Highlight potential bugs from integer division truncation
#![deny(clippy::arithmetic_side_effects)] // Detect arithmetic operations with potential side effects
#![deny(clippy::unchecked_duration_subtraction)] // Ensure duration subtraction won't cause underflow

// Unwraps
#![deny(clippy::unwrap_used)] // Discourage using .unwrap() which can cause panics
#![deny(clippy::panicking_unwrap)] // Prevent unwrap on values known to cause panics
#![deny(clippy::option_env_unwrap)]
// Prevent unwrapping environment variables which might be absent

// Array indexing
#![deny(clippy::indexing_slicing)]
// Avoid direct array indexing and use safer methods like .get()

// Path handling
#![deny(clippy::join_absolute_paths)]
// Prevent issues when joining paths with absolute paths

// Serialization issues
#![deny(clippy::serde_api_misuse)]
// Prevent incorrect usage of Serde's serialization/deserialization API

// Unbounded input
#![deny(clippy::uninit_vec)]
// Prevent creating uninitialized vectors which is unsafe

// Unsafe code detection
#![deny(clippy::transmute_int_to_char)] // Prevent unsafe transmutation from integers to characters
#![deny(clippy::transmute_int_to_float)] // Prevent unsafe transmutation from integers to floats
#![deny(clippy::transmute_ptr_to_ref)] // Prevent unsafe transmutation from pointers to references
#![deny(clippy::transmute_undefined_repr)] // Detect transmutes with potentially undefined representations
#![allow(clippy::empty_docs)]

use garde::Validate;
use serde::Deserialize;
use utoipa::IntoParams;

pub mod auth;
pub mod image;
pub mod profile;
pub mod review;
pub mod team;
pub mod tour;
pub mod user;

///
#[derive(Deserialize, Validate, IntoParams, Debug)]
#[into_params(style = Form, parameter_in = Query)]
pub struct Pagination {
    ///
    #[param(format = UInt16, minimum = 0, maximum = 57, default = 7)]
    #[garde(range(min = 0))]
    limit: Option<i64>,

    ///
    #[param(format = UInt64, minimum = 0, default = 0)]
    #[garde(range(min = 0))]
    offset: Option<i64>,
}
impl From<Pagination> for (u16, u64) {
    fn from(
        Pagination {
            limit,
            offset,
        }: Pagination,
    ) -> Self {
        (
            limit
                .map(|limit| limit.min(57))
                .map(u16::try_from)
                .and_then(Result::ok)
                .unwrap_or(7),
            offset.map(u64::try_from).and_then(Result::ok).unwrap_or(0),
        )
    }
}
