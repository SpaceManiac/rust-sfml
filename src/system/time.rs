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

use std::ops::{Add, Sub, Mul, Div};
use libc::c_longlong;
use std::mem;

/// Represents a time duration in a flexible way.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    microseconds: i64
}

// Verify at compile time that i64 == c_longlong
unsafe fn _assert_sizes_match(x: i64) -> c_longlong {
	mem::transmute(x)
}

impl Time {
    /// Construct a time value from a number of seconds
    pub fn with_seconds(seconds: f32) -> Time {
        Time {
            microseconds: (seconds * 1_000_000.) as i64
        }
    }

    /// Construct a time value from a number of milliseconds
    pub fn with_milliseconds(milliseconds: i32) -> Time {
        Time {
            microseconds: (milliseconds * 1_000) as i64
        }
    }

    /// Construct a time value from a number of microseconds
    pub fn with_microseconds(microseconds: i64) -> Time {
        Time {
            microseconds: microseconds
        }
    }

    /// Return a time value as a number of seconds
    pub fn as_seconds(&self) -> f32 {
        ((self.microseconds as f64) / 1_000_000.) as f32
    }

    /// Return a time value as a number of milliseconds
    pub fn as_milliseconds(&self) -> i32 {
        ((self.microseconds as f64) / 1_000.) as i32
    }

    /// Return a time value as a number of microseconds
    pub fn as_microseconds(&self) -> i64 {
        self.microseconds
    }
}

impl Add for Time {
	type Output = Time;
	fn add(self, other: Time) -> Time {
		Time { microseconds: self.microseconds + other.microseconds }
	}
}

impl Sub for Time {
	type Output = Time;
	fn sub(self, other: Time) -> Time {
		Time { microseconds: self.microseconds - other.microseconds }
	}
}

impl Mul<i64> for Time {
	type Output = Time;
	fn mul(self, rhs: i64) -> Time {
		Time { microseconds: self.microseconds * rhs }
	}
}

impl Mul<f64> for Time {
	type Output = Time;
	fn mul(self, rhs: f64) -> Time {
		Time { microseconds: (self.microseconds as f64 * rhs) as i64 }
	}
}

impl Div for Time {
	type Output = f32;
	fn div(self, rhs: Time) -> f32 {
		self.microseconds as f32 / rhs.microseconds as f32
	}
}

impl Div<i64> for Time {
	type Output = Time;
	fn div(self, rhs: i64) -> Time {
		Time { microseconds: self.microseconds / rhs }
	}
}

impl Div<f64> for Time {
	type Output = Time;
	fn div(self, rhs: f64) -> Time {
		Time { microseconds: (self.microseconds as f64 / rhs) as i64}
	}
}

#[test]
fn time_tests() {
	assert_eq!(Time::with_seconds(1.5), Time::with_milliseconds(1500));
	assert_eq!(Time::with_seconds(8.) / Time::with_seconds(2.), 4.);
	assert_eq!(Time::with_seconds(8.) / 2., Time::with_seconds(4.));
}
