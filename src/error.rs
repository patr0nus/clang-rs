use std::error::{Error};
use std::fmt;

use clang_sys as ffi;

use libc::{c_longlong};

use utility::{FromError};

//================================================
// Macros
//================================================

macro_rules! error {
    (
        $(#[$meta:meta])*
        pub enum $name:ident: $underlying:ty {
            $(#[$variantdoc:meta] $variant:ident = ($error:pat, $message:expr)), +,
        }
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $(#[$variantdoc] $variant), +
        }

        impl Error for $name {
            fn description(&self) -> &str {
                match *self {
                    $($name::$variant => $message), +
                }
            }
        }

        impl From<$name> for String {
            fn from(error: $name) -> String {
                error.description().into()
            }
        }

        impl FromError<$underlying> for $name {
            fn from_error(error: $underlying) -> Result<(), $name> {
                match error {
                    $($error => Err($name::$variant)), +,
                    _ => Ok(()),
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "{}", self.description())
            }
        }
    };
}

//================================================
// Enums
//================================================

// AlignofError __________________________________

error! {
    /// Indicates the error that prevented determining the alignment of a type.
    pub enum AlignofError: c_longlong {
        /// The type is a dependent type.
        Dependent = (-3, "the type is a dependent type"),
        /// The type is an incomplete type.
        Incomplete = (-2, "the type is an incomplete type"),
    }
}

// OffsetofError _________________________________

error! {
    /// Indicates the error that prevented determining the offset of a field in a record type.
    pub enum OffsetofError: c_longlong {
        /// The record type is a dependent type.
        Dependent = (-3, "the record type is a dependent type"),
        /// The record type is an incomplete type.
        Incomplete = (-2, "the record type is an incomplete type"),
        /// The record type does not contain a field with the supplied name.
        Name = (-5, "the record type does not contain a field with the supplied name"),
        /// The record type has an invalid parent declaration.
        Parent = (-1, "the record type has an invalid parent declaration"),
    }
}

// SaveError _____________________________________

error! {
    /// Indicates the type of error that prevented the saving of a translation unit to an AST file.
    pub enum SaveError: ffi::CXSaveError {
        /// Errors in the translation unit prevented saving.
        Errors = (ffi::CXSaveError::InvalidTU, "errors in the translation unit prevented saving"),
        /// An unknown error occurred.
        Unknown = (ffi::CXSaveError::Unknown, "an unknown error occurred"),
    }
}

// SizeofError ___________________________________

error! {
    /// Indicates the error that prevented determining the size of a type.
    pub enum SizeofError: c_longlong {
        /// The type is a dependent type.
        Dependent = (-3, "the type is a dependent type"),
        /// The type is an incomplete type.
        Incomplete = (-2, "the type is an incomplete type"),
        /// The type is a variable size type.
        VariableSize = (-4, "the type is a variable size type"),
    }
}

// SourceError ___________________________________

error! {
    /// Indicates the type of error that prevented the loading of a translation unit from a source
    /// file.
    pub enum SourceError: ffi::CXErrorCode {
        /// An error occurred while deserializing an AST file.
        AstDeserialization = (ffi::CXErrorCode::ASTReadError, "AST deserialization failed"),
        /// `libclang` crashed.
        Crash = (ffi::CXErrorCode::Crashed, "`libclang` crashed"),
        /// An unknown error occurred.
        Unknown = (ffi::CXErrorCode::Failure, "an unknown error occurred"),
    }
}
