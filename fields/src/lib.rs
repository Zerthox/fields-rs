//! Interface and macros for dynamically accessing struct fields at runtime.
//!
//! # Usage
//! ```no_run
//! use fields::Fields;
//!
//! #[derive(Default, Fields)]
//! struct MyStruct {
//!     valid: bool,
//!     id: u32,
//!     name: String,
//! }
//!
//! let mut my_struct = MyStruct::default();
//! my_struct.set(MyStructField::Valid(true));
//! ```
//!
//! The name and derives of the generated fields enum can be customized:
//! ```ignore
//! # use fields::Fields;
//! #[derive(Default, Fields)]
//! #[fields(name = "MyField")]
//! #[fields(derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize))]
//! struct MyStruct {
//!     valid: bool,
//!     id: u32,
//!     name: String,
//! }
//!
//! let mut my_struct = MyStruct::default();
//! my_struct.set(MyField::Valid(true));
//! ```
//!
//! Generated field variants can be filtered based on visibility:
//! ```no_run
//! # use fields::Fields;
//! #[derive(Fields)]
//! #[fields(visibility(pub, pub(crate)))]
//! struct Restricted {
//!     pub valid: bool,
//!     pub(crate) id: u32,
//!     name: String, // not in fields enum
//! }
//!
//! #[derive(Fields)]
//! #[fields(visibility(priv))]
//! struct Private {
//!     pub valid: bool, // not in fields enum
//!     pub(crate) id: u32, // not in fields enum
//!     name: String,
//! }
//! ```

pub use fields_macros::*;

/// Concise access to a struct field type.
pub type Field<T> = <T as Fields>::Field;

/// A trait for dynamically accessing struct fields at runtime.
pub trait Fields {
    /// Type representing individual fields with their values.
    type Field;

    /// Sets the value of the individual given field.
    fn set(&mut self, field: Self::Field);

    /// Sets the values for all given fields.
    #[inline]
    fn set_all(&mut self, fields: impl IntoIterator<Item = Self::Field>) {
        for field in fields {
            self.set(field);
        }
    }
}

/// A trait for dynamically accessing all struct fields at runtime.
pub trait AllFields: Fields
where
    Self::Field: Clone,
{
    /// Returns the current values of all fields.
    fn all(&self) -> impl Iterator<Item = Self::Field> + 'static;
}
