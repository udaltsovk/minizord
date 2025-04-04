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
#![allow(clippy::empty_docs)] // TODO: remove this

pub mod common;
pub mod health;
pub mod profile;
pub mod user;
