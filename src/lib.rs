// SPDX-License-Identifier: MIT OR Apache-2.0+

//! A collection of procedural macros compatible with stable Rust 2018 that simplify
//! some common operations not covered by `serde_derive`.
//!
//! Whilst this crate is named `serde_json_helpers`, `serde_json` is not a requirement
//! to use it; it's just what the proc macros were intended to be helpful for.
//!
//! # Examples
//!
//! ## `serde_enum_string`
//!
//! ```
//! use serde_json_helpers::serde_enum_string;
//!
//! #[serde_enum_string(transform = "snake_case")]
//! #[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
//! enum ExampleEnum {
//!     Option1,
//!     Option2,
//!     Option3,
//!     AnotherOption,
//! }
//!
//! fn main() {
//!     let out1 =
//!         serde_json::to_string(&ExampleEnum::Option1).expect("Unable to serialize ExampleEnum");
//!     let out2 =
//!         serde_json::to_string(&ExampleEnum::Option2).expect("Unable to serialize ExampleEnum");
//!     let out3 =
//!         serde_json::to_string(&ExampleEnum::Option3).expect("Unable to serialize ExampleEnum");
//!     let out4 = serde_json::to_string(&ExampleEnum::AnotherOption)
//!         .expect("Unable to serialize ExampleEnum");
//!
//!     println!("Serialized ExampleEnum::Option1: {}", out1);
//!     println!("Serialized ExampleEnum::Option2: {}", out2);
//!     println!("Serialized ExampleEnum::Option3: {}", out3);
//!     println!("Serialized ExampleEnum::AnotherOption: {}", out4);
//!
//!     let in1 = serde_json::from_str::<ExampleEnum>(&*out1).expect("Unable to deserialize");
//!     let in2 = serde_json::from_str::<ExampleEnum>(&*out2).expect("Unable to deserialize");
//!     let in3 = serde_json::from_str::<ExampleEnum>(&*out3).expect("Unable to deserialize");
//!     let in4 = serde_json::from_str::<ExampleEnum>(&*out4).expect("Unable to deserialize");
//!
//!     println!("Deserialized {}: {:?}", out1, in1);
//!     println!("Deserialized {}: {:?}", out2, in2);
//!     println!("Deserialized {}: {:?}", out3, in3);
//!     println!("Deserialized {}: {:?}", out4, in4);
//!
//!     println!(
//!         "Bad deserialized value {}: {:?}",
//!         "\"bad_value\"",
//!         serde_json::from_str::<ExampleEnum>("\"bad_value\"")
//!     );
//! }
//! ```

#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;

mod enum_string;
mod helpers;
mod string_transform;

/// Allows a C-style `enum` to be serialized as a string, useful for human-readable
/// JSON.
///
/// Takes two optional attributes:
///
/// * `transform = "<type>"` - transform the `enum` variants by name into the same
///   formats as supported by the serde attribute `rename_all`. This includes:
///     * `lowercase` - makes variant names _lowercase_
///     * `uppercase` - makes variant names _UPPERCASE_
///     * `pascal_case` - makes variant names _PascalCase_
///     * `camel_case` - makes variant names _camelCase_
///     * `snake_case` - makes variant names _snake\_case_
///     * `screaming_snake_case` - makes variant names _SCREAMING\_SNAKE\_CASE_
///     * `kebab_case` - makes variant names _snake\_case_
///     * `screaming_snake_case` - makes variant names _SCREAMING\_SNAKE\_CASE_
///
///   If unspecified, the `enum` variant names will be passed through unmodified.
///
/// * `prepend_enum_name` - Add the name of the `enum` to the values for each variant. This will
///   be prepended to the variant name before running the transform described above.
///
/// Note that this macro is incompatible with existing `Serialize` and `Deserialize` `impl`s.
/// If a Serialize or Deserialize derive is detected, this macro will panic, but if you `impl` them
/// directly you will just get normal compiler issues which you're on your own to figure out.
#[proc_macro_attribute]
pub fn serde_enum_string(attr: TokenStream, item: TokenStream) -> TokenStream {
    enum_string::serde_enum_string_impl(attr, item)
}
