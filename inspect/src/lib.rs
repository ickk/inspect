#![doc = include_str!("../../README.md")]

pub mod type_info;
pub use self::type_info::TypeInfo;
#[cfg(feature = "derive")]
pub use ::inspect_derive::TypeInfo;
