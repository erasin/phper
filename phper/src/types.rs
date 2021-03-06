//! Apis relate to PHP types.

use crate::sys::*;
use derive_more::From;
use std::{ffi::CStr, os::raw::c_int};

/// Wrapper of PHP type.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Type {
    t: u32,
}

impl Type {
    pub const fn from_raw(t: u32) -> Self {
        Self { t }
    }

    pub const fn into_raw(self) -> u32 {
        self.t
    }

    pub const fn undef() -> Type {
        Self::from_raw(IS_UNDEF)
    }

    pub const fn null() -> Type {
        Self::from_raw(IS_NULL)
    }

    pub const fn bool(b: bool) -> Type {
        Self::from_raw(if b { IS_TRUE } else { IS_FALSE })
    }

    pub const fn long() -> Type {
        Self::from_raw(IS_LONG)
    }

    pub const fn double() -> Type {
        Self::from_raw(IS_DOUBLE)
    }

    pub const fn array() -> Type {
        Self::from_raw(IS_ARRAY)
    }

    pub const fn array_ex() -> Type {
        Self::from_raw(IS_ARRAY_EX)
    }

    pub const fn object() -> Type {
        Self::from_raw(IS_OBJECT)
    }

    pub const fn object_ex() -> Type {
        Self::from_raw(IS_OBJECT_EX)
    }

    pub const fn is_undef(self) -> bool {
        self.t == IS_UNDEF
    }

    pub const fn is_null(self) -> bool {
        self.t == IS_NULL
    }

    pub const fn is_bool(self) -> bool {
        self.is_true() || self.is_false()
    }

    pub const fn is_true(self) -> bool {
        get_base_type_by_raw(self.t) == IS_TRUE
    }

    pub const fn is_false(self) -> bool {
        get_base_type_by_raw(self.t) == IS_FALSE
    }

    pub const fn is_long(self) -> bool {
        get_base_type_by_raw(self.t) == IS_LONG
    }

    pub const fn is_double(self) -> bool {
        get_base_type_by_raw(self.t) == IS_DOUBLE
    }

    pub const fn is_string(self) -> bool {
        get_base_type_by_raw(self.t) == IS_STRING
    }

    pub const fn is_array(self) -> bool {
        get_base_type_by_raw(self.t) == IS_ARRAY
    }

    pub const fn is_object(self) -> bool {
        get_base_type_by_raw(self.t) == IS_OBJECT
    }

    pub const fn is_indirect(self) -> bool {
        self.t == IS_INDIRECT
    }

    pub const fn get_base_type(self) -> Type {
        Self::from_raw(get_base_type_by_raw(self.t))
    }

    pub fn get_base_type_name(self) -> crate::Result<String> {
        get_type_by_const(self.t)
    }
}

impl From<u32> for Type {
    fn from(n: u32) -> Self {
        Self::from_raw(n)
    }
}

fn get_type_by_const(mut t: u32) -> crate::Result<String> {
    unsafe {
        t = get_base_type_by_raw(t);
        let s = zend_get_type_by_const(t as c_int);
        let mut s = CStr::from_ptr(s).to_str()?.to_string();

        // Compact with PHP7.
        if s == "boolean" {
            s = "bool".to_string();
        } else if s == "integer" {
            s = "int".to_string();
        }

        Ok(s)
    }
}

const fn get_base_type_by_raw(t: u32) -> u32 {
    t & !(!0 << Z_TYPE_FLAGS_SHIFT)
}

#[derive(From)]
pub enum Scalar {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
}

impl From<i32> for Scalar {
    fn from(i: i32) -> Self {
        Self::I64(i.into())
    }
}

impl From<&str> for Scalar {
    fn from(s: &str) -> Self {
        Self::String(s.to_owned())
    }
}

impl From<&[u8]> for Scalar {
    fn from(b: &[u8]) -> Self {
        Self::Bytes(b.to_owned())
    }
}
