extern crate alloc;
use windows_sys::{
    core::BSTR,
    Win32::{
        Foundation::VARIANT_BOOL,
        System::{
            Com::{
                SAFEARRAY, SAFEARRAYBOUND,
            },
            Ole::{
                SafeArrayAccessData, SafeArrayCreate, SafeArrayCreateVector, SafeArrayGetElement,
                SafeArrayGetLBound, SafeArrayGetUBound, SafeArrayPutElement, SafeArrayUnaccessData,
            },
        },
    },
};
//use windows_sys::Win32::System::{Variant,VARIANT_0, VARIANT_0_0, VARIANT_0_0_0};
use windows_sys::Win32::System::Variant::{VARIANT,VARIANT_0, VARIANT_0_0, VARIANT_0_0_0,
    VT_ARRAY, VT_BOOL, VT_BSTR, VT_EMPTY, VT_I8, VT_UI1, VT_UNKNOWN, VT_VARIANT,VARENUM,
};

use crate::primitives::{
    string_to_bstr
};

use core::mem;
use core::ptr;
use core::ffi::c_void;
use core::mem::ManuallyDrop;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;


pub fn prepare_assembly(bytes: &[u8]) -> Result<*mut SAFEARRAY, String> {
    let mut bounds = SAFEARRAYBOUND {
        cElements: bytes.len() as _,
        lLbound: 0,
    };

    let safe_array_ptr: *mut SAFEARRAY = unsafe { SafeArrayCreate(VT_UI1, 1, &mut bounds) };
    let mut pv_data: *mut c_void = ptr::null_mut();

    let hr = unsafe { SafeArrayAccessData(safe_array_ptr, &mut pv_data) };
    if hr != 0 {
        if cfg!(feature = "verbose")
        {
            return Err(format!("Could not prepare assembly: {:?}", hr));
        }
        else
        {
            return Err("".to_string());
        }

    }

    unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), pv_data.cast(), bytes.len()) };

    let hr = unsafe { SafeArrayUnaccessData(safe_array_ptr) };
    if hr != 0 {
        if cfg!(feature = "verbose")
        {
            return Err(format!("Could not prepare assembly: {:?}", hr));
        }
        else
        {
            return Err("".to_string());
        }
    }
    

    Ok(safe_array_ptr)
}

pub fn get_array_length(array_ptr: *mut SAFEARRAY) -> i32 {
    let mut upper: i32 = 0;
    unsafe { SafeArrayGetUBound(array_ptr, 1, &mut upper) };
    let mut lower: i32 = 0;
    unsafe { SafeArrayGetLBound(array_ptr, 1, &mut lower) };

    match upper - lower {
        0 => 0,
        delta => delta + 1,
    }
}

pub fn empty_array() -> *mut SAFEARRAY {
    unsafe { SafeArrayCreateVector(VT_EMPTY, 0, 0) }
}

pub fn empty_variant_array() -> *mut SAFEARRAY {
    unsafe { SafeArrayCreateVector(VT_VARIANT, 0, 0) }
}

pub fn wrap_unknown_ptr_in_variant(unknown_ptr: *mut c_void) -> VARIANT {
    let unknown: *mut c_void = unknown_ptr;

    VARIANT {
        Anonymous: VARIANT_0 {
            Anonymous: *ManuallyDrop::new(VARIANT_0_0 {
                vt: VT_UNKNOWN,
                wReserved1: 0,
                wReserved2: 0,
                wReserved3: 0,
                Anonymous: VARIANT_0_0_0 {
                    punkVal: *ManuallyDrop::new(unknown),
                },
            }),
        },
    }
}

pub fn wrap_bool_in_variant(value: bool) -> VARIANT {
    VARIANT {
        Anonymous: VARIANT_0 {
            Anonymous: *ManuallyDrop::new(VARIANT_0_0 {
                vt: VT_BOOL,
                wReserved1: 0,
                wReserved2: 0,
                wReserved3: 0,
                Anonymous: VARIANT_0_0_0 {
                    boolVal: VARIANT_BOOL::from(value),
                },
            }),
        },
    }
}

pub fn wrap_i64_in_variant(value: i64) -> VARIANT {
    VARIANT {
        Anonymous: VARIANT_0 {
            Anonymous: *ManuallyDrop::new(VARIANT_0_0 {
                vt: VT_I8,
                wReserved1: 0,
                wReserved2: 0,
                wReserved3: 0,
                Anonymous: VARIANT_0_0_0 { llVal: value },
            }),
        },
    }
}

pub fn wrap_string_in_variant(string: &str) -> VARIANT {
    let inner = string_to_bstr(string);

    VARIANT {
        Anonymous: VARIANT_0 {
            Anonymous: *ManuallyDrop::new(VARIANT_0_0 {
                vt: VT_BSTR,
                wReserved1: 0,
                wReserved2: 0,
                wReserved3: 0,
                Anonymous: VARIANT_0_0_0 {
                    bstrVal: *ManuallyDrop::new(inner),
                },
            }),
        },
    }
}

pub fn wrap_strings_in_array(strings: &[String]) -> Result<VARIANT, String> {
    let mut inner = vec![];

    for string in strings.iter() {
        inner.push(string_to_bstr(string))
    }

    let safe_array_ptr: *mut SAFEARRAY =
        unsafe { SafeArrayCreateVector(VT_BSTR, 0, inner.len() as u32) };

    for i in 0..inner.len() {
        let indices: [i32; 1] = [i as _];
        let v_ref = &inner[i];
        let hr = unsafe { SafeArrayPutElement(safe_array_ptr, indices.as_ptr(), *v_ref as *const _) };
        if hr != 0 {
            if cfg!(feature = "verbose")
            {
                return Err(format!("Could not create an array of strings: {:?}", hr));
            }
            else
            {
                return Err("".to_string());
            }
        }
    }

    Ok(VARIANT {
        Anonymous: VARIANT_0 {
            Anonymous: *ManuallyDrop::new(VARIANT_0_0 {
                vt: (VT_BSTR | VT_ARRAY) as u16,
                wReserved1: 0,
                wReserved2: 0,
                wReserved3: 0,
                Anonymous: VARIANT_0_0_0 {
                    parray: safe_array_ptr,
                },
            }),
        },
    })
}

pub fn wrap_method_arguments(arguments: Vec<VARIANT>) -> Result<*mut SAFEARRAY, String> {
    let variant_array_ptr: *mut SAFEARRAY =
        unsafe { SafeArrayCreateVector(VT_VARIANT, 0, arguments.len() as u32) };

    for i in 0..arguments.len() {
        let indices: [i32; 1] = [i as _];
        let v_ref: *const _ = &arguments[i];
        let hr = unsafe { SafeArrayPutElement(variant_array_ptr, indices.as_ptr(), v_ref as *const _) };
        if hr != 0 {
            if cfg!(feature = "verbose")
            {
                return Err(format!("Could not create an array of arguments: {:?}", hr));
            }
            else
            {
                return Err("".to_string());
            }
        }
    }

    Ok(variant_array_ptr)
}

pub fn unpack_byte_array(safe_array_ptr: *mut SAFEARRAY) -> Result<Vec<u8>, String> {
    let mut ubound: i32 = 0;
    unsafe { SafeArrayGetUBound(safe_array_ptr, 1, &mut ubound) };
    let mut results: Vec<u8> = vec![];

    for i in 0..ubound {
        let indices: [i32; 1] = [i as _];
        let mut variant: u8 = 0;
        let pv = &mut variant as *mut _ as *mut c_void;

        let hr = unsafe { SafeArrayGetElement(safe_array_ptr, indices.as_ptr(), pv) };
        if hr != 0 {
            if cfg!(feature = "verbose")
            {
                return Err(format!("Could not access safe array: {:?}", hr));
            }
            else
            {
                return Err("".to_string());
            }
        }

        if !pv.is_null() {
            results.push(variant);
        }
    }

    Ok(results)
}
