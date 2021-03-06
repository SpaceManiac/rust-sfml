/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

#![allow(dead_code, non_camel_case_types)]

mod sf_bool;
mod foreign_ptr;
pub use self::sf_bool::SfBool;
pub use self::foreign_ptr::{Foreign, ForeignHolder, Ref};

macro_rules! foreign_type {
	($($name:ident, $destroy:ident;)*) => (
		$(
			#[repr(C)]
			pub struct $name (());
			impl $crate::ffi::foreign_ptr::ForeignType for $name {
				unsafe fn destroy(ptr: *mut $name) {
					$destroy(ptr);
				}
			}
		)*
	)
}

pub mod window;
pub mod graphics;
pub mod audio;

/// Encode a string in UTF-32 for passing into SFML.
pub fn to_utf32(string: &str) -> Vec<u32> {
	let mut vec: Vec<u32> = string.chars().map(|ch| ch as u32).collect();
	vec.push(0);
	vec
}

/// Perform best-effort decoding of a C string (assumed UTF-8).
pub unsafe fn from_c_str(ptr: *const ::libc::c_char) -> String {
	if ptr.is_null() {
		"".to_owned()
	} else {
		String::from_utf8_lossy(::std::ffi::CStr::from_ptr(ptr).to_bytes()).into_owned()
	}
}
