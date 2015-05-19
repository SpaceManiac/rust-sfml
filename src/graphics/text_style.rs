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

//! Available text styles.

// Manual #[doc] tags are to work around apparent bitflags! bug where the three
// slashes are included in the resulting docs.

bitflags! {
	#[doc="Available text styles."]
	#[derive(Debug)]
	#[repr(C)]
	flags TextStyle: u32 {
		#[doc="Regular characters, no style."]
		const REGULAR = 0,
		#[doc="Bold characters."]
		const BOLD = 1 << 0,
		#[doc="Italic characters."]
		const ITALIC = 1 << 1,
		#[doc="Underlined characters."]
		const UNDERLINED = 1 << 2,
		#[doc="Stuck-through characters."]
		const STRIKETHROUGH = 1 << 3
	}
}
