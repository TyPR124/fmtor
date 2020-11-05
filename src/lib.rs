#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
//! # fmtor
//!
//! See [`fmtor::FmtOr`]
//!
//! # Examples
//!
//! ```rust
//! use fmtor::FmtOr;
//!
//! let maybe_ref: Option<&()> = None;
//!
//! assert_eq!(
//!     "Null",
//!     format!("{:p}", maybe_ref.fmt_or("Null"))
//! );
//! ```
//!
//! ```rust
//! use fmtor::FmtOr;
//!
//! struct NullMarker;
//! impl std::fmt::Display for NullMarker {
//!     fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
//!         out.write_str("Null")
//!     }    
//! }
//! let maybe_ref: Option<&u32> = None;
//!
//! assert_eq!(
//!     "Null",
//!     format!("{:x}", maybe_ref.fmt_or(NullMarker))
//! );
//! ```
//!

#[cfg(test)]
mod tests;

use core::fmt::{
    Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Pointer, Result, UpperExp,
    UpperHex,
};
/// The type returned from [`FmtOr::fmt_or_empty`]
#[derive(Eq, PartialEq)]
pub struct MaybeFormat<'t, T>(&'t Option<T>);
/// The type returned from [`FmtOr::fmt_or`]
pub struct MaybeFormatOr<'t, T, U>(&'t Option<T>, U);
/// The type returned from [`FmtOr::fmt_or_else`]
pub struct MaybeFormatOrElse<'t, T, F>(&'t Option<T>, F);

impl<'t, T> Copy for MaybeFormat<'t, T> {}
impl<'t, T> Clone for MaybeFormat<'t, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'t, T, U: Copy> Copy for MaybeFormatOr<'t, T, U> {}
impl<'t, T, U: Clone> Clone for MaybeFormatOr<'t, T, U> {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}

impl<'t, T, F: Copy> Copy for MaybeFormatOrElse<'t, T, F> {}
impl<'t, T, F: Clone> Clone for MaybeFormatOrElse<'t, T, F> {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}

/// An extension trait for [`Option<T>`]. The methods on this trait are the inteded way to use this crate.
///
/// # TLDR
///
/// The methods on this trait allow a missing [`Option<T>`] value to be formatted as if it
/// were a `T`, however [`None`] is replaced by some other value.
///
/// # Details
///
/// The methods on this trait return types which implement all the same format traits as `T`.
/// They borrow the [`Option<T>`] for the duration of the formatting operation.
///
/// When the value is [`None`], the formatting is replaced with another, whose type and value are determined
/// by the user. The replacement type, `U`, need only implement [`Display`]. Regardless of the desired format
/// of T, the format of U will *always* be [`Display`]. This makes it simple to replace non-[`Display`] formats with
/// the anticipated values.
///
/// ```rust
/// fn fallable_box_maker() -> Option<Box<()>> { None }
///
/// use fmtor::FmtOr;
///
/// let maybe_box = fallable_box_maker();
/// println!("Got a fallable box at {:p}", maybe_box.fmt_or("Null"));
/// ```
///
/// # More formatting logic
///
/// Here's a real-ish example with surrounding formatting logic. Formatting is applied to log messages
/// so that they include the optional file name and line number when either the file or both are present.
/// Having just a line number by itself wouldn't be all that helpful.
///
/// ```rust no_run
/// // Be warned: this is a hypothetical code snippet that may not compile on current versions of amethyst.
/// // The code is not compiled by doctest, and therefore should not be relied upon as working.
///
/// use fmtor::FmtOr;
///
/// // Generates log messeges like
/// // [INFO] src\game\states\init.rs:56 - Init starting
/// # #[cfg(never)]
/// amethyst::Logger::from_config_formatter(config, |out, msg, record| {
///     out.finish(format_args!(
///         "[{level}] {file}{colon}{line}{spacer}{message}",
///         level = record.level(),
///         file = record.file().unwrap_or(""), // <-- This could be .fmt_or(""), but is not necessary
///         colon = if record.file().is_some() { ":" } else { "" },
///         line = record.file().and(record.line()).fmt_or(""), // <-- This could not be .unwrap_or("") due to type error
///         spacer = if record.file().is_some() { " - " } else { "" },
///         message = msg
///     ))
/// })
/// .start();
/// ```
///
#[allow(clippy::needless_lifetimes)] // They're nice to see in docs
pub trait FmtOr<T> {
    /// Format the value, if there is one, or display an empty string instead.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtor::FmtOr;
    ///
    /// let foo = Some(0x42);
    /// let bar: Option<u32> = None;
    ///
    /// assert_eq!(
    ///     "0x42",
    ///     format!("{:#x}", foo.fmt_or_empty())
    /// );
    /// assert_eq!(
    ///     "",
    ///     format!("{:#x}", bar.fmt_or_empty())
    /// );
    /// ```
    fn fmt_or_empty<'t>(&'t self) -> MaybeFormat<'t, T>;
    /// Format the value, if there is one, or display the given value instead.
    ///
    /// The given value must implement [`Display`]
    /// regardless of which formatting is used on the original value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtor::FmtOr;
    ///
    /// let foo = Some(Box::new(42));
    /// let bar: Option<Box<u32>> = None;
    ///
    /// assert_eq!(
    ///     "foo: 42",
    ///     format!("foo: {}", foo.fmt_or("Null"))
    /// );
    /// assert_eq!(
    ///     "bar: Null",
    ///     format!("bar: {}", bar.fmt_or("Null"))
    /// );
    /// ```
    fn fmt_or<'t, U>(&'t self, u: U) -> MaybeFormatOr<'t, T, U>
    where
        U: Display;
    /// Format the value, if there is one, or run the closure to get a value to display instead.
    ///
    /// The returned value must implement [`Display`]
    /// regardless of which formatting is used on the original value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtor::FmtOr;
    ///
    /// let foo = Some(42);
    /// let bar: Option<u32> = None;
    /// let missing_msg = "Missing";
    /// let missing_item = "bar";
    ///
    /// assert_eq!(
    ///     "42",
    ///     format!("{}", foo.fmt_or_else(||format!("{} {}", missing_msg, missing_item)))
    /// );
    /// assert_eq!(
    ///     "Missing bar",
    ///     format!("{}", bar.fmt_or_else(||format!("{} {}", missing_msg, missing_item)))
    /// );
    /// ```
    fn fmt_or_else<'t, U, F>(&'t self, f: F) -> MaybeFormatOrElse<'t, T, F>
    where
        U: Display,
        F: Fn() -> U;
}

impl<T> FmtOr<T> for Option<T> {
    #[inline]
    fn fmt_or_empty(&self) -> MaybeFormat<T> {
        MaybeFormat(self)
    }
    #[inline]
    fn fmt_or<U>(&self, u: U) -> MaybeFormatOr<T, U>
    where
        U: Display,
    {
        MaybeFormatOr(self, u)
    }
    #[inline]
    fn fmt_or_else<U, F>(&self, f: F) -> MaybeFormatOrElse<T, F>
    where
        U: Display,
        F: Fn() -> U,
    {
        MaybeFormatOrElse(self, f)
    }
}

macro_rules! impl_fmt_traits {
    ($($Trait:ident),*$(,)?) => {$(

impl<'t, T> $Trait for MaybeFormat<'t, T>
where
    T: $Trait,
{
    #[inline]
    fn fmt(&self, out: &mut Formatter<'_>) -> Result {
        $Trait::fmt(&self.0.fmt_or(""), out)
    }
}

impl<'t, T, U> $Trait for MaybeFormatOr<'t, T, U>
where
    T: $Trait,
    U: Display,
{
    #[inline]
    fn fmt(&self, out: &mut Formatter<'_>) -> Result {
        $Trait::fmt(&self.0.fmt_or_else(||&self.1), out)
    }
}

impl<'t, T, F, U> $Trait for MaybeFormatOrElse<'t, T, F>
where
    T: $Trait,
    F: Fn() -> U,
    U: Display,
{
    #[inline]
    fn fmt(&self, out: &mut Formatter<'_>) -> Result {
        if let Some(t) = self.0 {
            <T as $Trait>::fmt(t, out)
        } else {
            Display::fmt(&self.1(), out)
        }
    }
}

    )*}
} // macro_rules! impl_fmt_traits

impl_fmt_traits!(Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex);
