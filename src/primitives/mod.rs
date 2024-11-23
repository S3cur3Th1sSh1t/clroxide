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

use core::ffi::c_void;
use alloc::string::String;

pub trait Interface: Sized {
    const IID: GUID;

    fn vtable(&self) -> *const c_void;
}

pub trait Class: Sized {
    const CLSID: GUID;
}
