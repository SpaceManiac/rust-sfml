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

/// Generic two-dimensional vector.
#[repr(C)]
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Copy, Default)]
pub struct Vector2<T> {
    /// X coordinate of the vector.
    pub x: T,
    /// Y coordinate of the vector.
    pub y: T
}

/// Two-dimensional integer vector.
pub type Vector2i = Vector2<i32>;
/// Two-dimensional unsigned integer vector.
pub type Vector2u = Vector2<u32>;
/// Two-dimensional float vector.
pub type Vector2f = Vector2<f32>;

impl<T> Vector2<T> {
    /// Create a new `Vector2` with the given values.
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 {
            x: x,
            y: y
        }
    }

	/// Convert this `Vector2` into a 2-tuple.
	pub fn to_tup(self) -> (T, T) {
		(self.x, self.y)
	}
}

impl Vector2<i32> {
	/// Convert this vector to a Vector2u.
	pub fn to_vector2u(self) -> Vector2u {
		Vector2u::new(self.x as u32, self.y as u32)
	}
	/// Convert this vector to a Vector2f.
	pub fn to_vector2f(self) -> Vector2f {
		Vector2f::new(self.x as f32, self.y as f32)
	}
}

impl Vector2<u32> {
	/// Convert this vector to a Vector2i.
	pub fn to_vector2i(self) -> Vector2i {
		Vector2i::new(self.x as i32, self.y as i32)
	}
	/// Convert this vector to a Vector2f.
	pub fn to_vector2f(self) -> Vector2f {
		Vector2f::new(self.x as f32, self.y as f32)
	}
}

impl Vector2<f32> {
	/// Convert this vector to a Vector2u.
	pub fn to_vector2u(self) -> Vector2u {
		Vector2u::new(self.x as u32, self.y as u32)
	}
	/// Convert this vector to a Vector2i.
	pub fn to_vector2i(self) -> Vector2i {
		Vector2i::new(self.x as i32, self.y as i32)
	}
}

impl<T> From<(T, T)> for Vector2<T> {
	fn from((x, y): (T, T)) -> Vector2<T> {
		Vector2 { x: x, y: y }
	}
}

impl<T: Add> Add for Vector2<T> {
    type Output = Vector2<T::Output>;

	/// Component-wise addition.
    fn add(self, other: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T: Sub> Sub for Vector2<T> {
    type Output = Vector2<T::Output>;

	/// Component-wise subtraction.
    fn sub(self, other: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<T: Copy + Mul> Mul<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

	/// Scalar multiplication.
    fn mul(self, rhs: T) -> Vector2<T::Output> {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl<T: Mul> Mul for Vector2<T> {
    type Output = Vector2<T::Output>;

	/// Component-wise multiplication.
    fn mul(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T: Copy + Div> Div<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

	/// Scalar division.
    fn div(self, rhs: T) -> Vector2<T::Output> {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl<T: Div> Div for Vector2<T> {
	type Output = Vector2<T::Output>;

	/// Component-wise division.
	fn div(self, rhs: Vector2<T>) -> Vector2<T::Output> {
		Vector2 {
			x: self.x / rhs.x,
			y: self.y / rhs.y
		}
	}
}
