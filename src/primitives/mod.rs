#[allow(non_snake_case)]
mod helpers;
mod iappdomain;
mod iassembly;
mod iclrmetahost;
mod iclrruntimeinfo;
mod iconstructorinfo;
mod icorruntimehost;
mod ienumunknown;
mod imethodinfo;
mod ipropertyinfo;
mod itype;
mod iunknown;
mod types;
extern crate alloc;

pub use helpers::*;
pub use iappdomain::*;
pub use iassembly::*;
pub use iclrmetahost::*;
pub use iclrruntimeinfo::*;
pub use iconstructorinfo::*;
pub use icorruntimehost::*;
pub use ienumunknown::*;
pub use imethodinfo::*;
pub use ipropertyinfo::*;
pub use itype::*;
pub use iunknown::*;
pub use types::*;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use windows_sys::core::BSTR;
use core::ffi::c_void;

/*
pub fn string_to_bstr(s: &str) -> BSTR {
    let wide: Vec<u16> = s.encode_utf16().collect();
    let bstr: BSTR = BSTR::from_raw(wide.as_ptr());
    bstr
}*/

// Import Windows API functions and types
#[link(name = "oleaut32")]
extern "system" {
    fn SysAllocStringLen(psz: *const u16, len: u32) -> *mut u16;
    fn SysFreeString(bstr: *mut u16);
}

/// Converts a `&str` to a `BSTR` in a no_std environment.
fn string_to_bstr(s: &str) -> *mut u16 {
    // Step 1: Convert the Rust string to a UTF-16 wide string
    let utf16: Vec<u16> = s.encode_utf16().collect();
    let len = utf16.len() as u32;

    // Step 2: Allocate a BSTR using SysAllocStringLen
    unsafe {
        let bstr = SysAllocStringLen(utf16.as_ptr(), len);

        // Check allocation success
        if bstr.is_null() {
            panic!("");
        }

        bstr
    }
}

/// Frees a BSTR allocated earlier
fn free_bstr(bstr: *mut u16) {
    unsafe {
        if !bstr.is_null() {
            SysFreeString(bstr);
        }
    }
}

use core::str::{from_utf8, from_utf8_unchecked};

pub fn from_utf8_lossy(bytes: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < bytes.len() {
        match from_utf8(&bytes[i..]) {
            Ok(valid) => {
                result.push_str(valid);
                break;
            }
            Err(e) => {
                let valid_up_to = e.valid_up_to();
                if valid_up_to > 0 {
                    result.push_str(unsafe { from_utf8_unchecked(&bytes[i..i + valid_up_to]) });
                }
                result.push('\u{FFFD}'); // Unicode replacement character
                i += valid_up_to + 1;
            }
        }
    }

    result
}

// Helper function to determine the length of a null-terminated UTF-16 string
pub fn wcslen2(s: *const u16) -> usize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

pub fn from_utf16_lossy2(v: &[u16]) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < v.len() {
        match core::char::decode_utf16(v[i..].iter().cloned()).next() {
            Some(Ok(c)) => {
                result.push(c);
                i += 1;
            }
            Some(Err(_)) => {
                result.push('\u{FFFD}'); // Unicode replacement character
                i += 1;
            }
            None => break,
        }
    }

    result
}


pub trait Interface: Sized {
    const IID: GUID;

    fn vtable(&self) -> *const c_void;
}

pub trait Class: Sized {
    const CLSID: GUID;
}
