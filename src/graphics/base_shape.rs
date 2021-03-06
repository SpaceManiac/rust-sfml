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

use libc::{c_void, c_float, c_uint};
use std::{ptr, mem};

use graphics::{RenderTarget, RenderStates, Texture, Color, Transformable,
               Transform, IntRect, FloatRect, Drawable, ShapeImpl, Shape};
use system::Vector2f;

use ffi::{SfBool, Foreign};
use ffi::graphics as ffi;

/// Custom textured shape with outline.
///
/// Implements most of the `Shape` trait and delegates `get_point_count()` and
/// `get_point()` to an implementor of `ShapeImpl`.
pub struct BaseShape<'s> {
    shape: Foreign<ffi::sfShape>,
    texture: Option<&'s Texture>
}

extern fn get_point_count_callback<T: ShapeImpl>(obj: *mut c_void) -> u32 {
	let shape: &T = unsafe { mem::transmute(obj) };
	shape.get_point_count()
}

extern fn get_point_callback<T: ShapeImpl>(point: u32, obj: *mut c_void) -> Vector2f {
	let shape: &T = unsafe { mem::transmute(obj) };
	shape.get_point(point)
}

impl<'s> BaseShape<'s> {
    /// Create a new Shape with the given implementation.
    ///
    /// Returns Some(Shape) or None on failure.
    pub fn new<T: ShapeImpl>(shape_impl: &'s T) -> Option<BaseShape<'s>> {
		unsafe {
			Foreign::new(ffi::sfShape_create(
				get_point_count_callback::<T>,
				get_point_callback::<T>,
				mem::transmute(shape_impl)
			))
		}.map(|sp| {
			let mut shape = BaseShape {
				shape: sp,
				texture: None
			};
			shape.update();
			shape
		})
    }

    /// Create a new Shape with the given implementation and texture.
	///
    /// Returns Some(Shape) or None on failure.
    pub fn new_with_texture<T: ShapeImpl>(shape_impl: &'s T, texture: &'s Texture) -> Option<BaseShape<'s>> {
		BaseShape::new(shape_impl).map(|mut shape| {
			shape.set_texture(texture, true);
			shape
		})
    }

	fn raw(&self) -> &ffi::sfShape { self.shape.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfShape { self.shape.as_mut() }
    #[doc(hidden)]
    pub fn unwrap(&self) -> &ffi::sfShape { self.raw() }

    /// Change the source texture of the shape.
    ///
    /// If reset_rect is true, the TextureRect property of
    /// the shape is automatically adjusted to the size of the new
    /// texture. If it is false, the texture rect is left unchanged.
    pub fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        self.texture = Some(texture);
        unsafe {
            ffi::sfShape_setTexture(self.raw_mut(), texture.unwrap(), SfBool::from_bool(reset_rect))
        }
    }

    /// Disable the source texture of this shape and reset the texture rect.
    pub fn disable_texture(&mut self) {
        self.texture = None;
        unsafe {
            ffi::sfShape_setTexture(self.raw_mut(), ptr::null_mut(), SfBool::SFTRUE)
        }
    }

    /// Get the source texture of the shape.
    ///
    /// You can't modify the texture when you retrieve it with this function.
    pub fn get_texture(&self) -> Option<&'s Texture> {
        self.texture
    }

    /// Set the sub-rectangle of the texture that the shape will display.
	///
	/// The texture rect is useful when you don't want to display the whole
	/// texture, but rather a part of it. By default, the texture rect covers
	/// the entire texture.
    pub fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe { ffi::sfShape_setTextureRect(self.raw_mut(), rect) }
    }

    /// Get the sub-rectangle of the texture displayed by the shape.
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe { ffi::sfShape_getTextureRect(self.raw()) }
    }

    /// Get the local bounding rectangle of the sprite.
    ///
    /// The returned rectangle is in local coordinates, which means
    /// that it ignores the transformations (translation, rotation,
    /// scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// entity in the entity's coordinate system.
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfShape_getLocalBounds(self.raw()) }
    }

    /// Get the global bounding rectangle of the sprite.
    ///
    /// The returned rectangle is in global coordinates, which means
    /// that it takes in account the transformations (translation,
    /// rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// sprite in the global 2D world's coordinate system.
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfShape_getGlobalBounds(self.raw()) }
    }

    /// Recompute the internal geometry of a shape.
    ///
    /// This function must be called by specialized shape objects
    /// everytime their points change (i.e. the result of either
    /// the `get_point_count()` or `get_point()` callbacks is different).
    pub fn update(&mut self) {
        unsafe { ffi::sfShape_update(self.raw_mut()) }
    }
}

impl<'s> Shape for BaseShape<'s> {
    fn set_fill_color(&mut self, color: Color) {
        unsafe {
            ffi::sfShape_setFillColor(self.raw_mut(), color)
        }
    }

    fn set_outline_color(&mut self, color: Color) {
        unsafe {
            ffi::sfShape_setOutlineColor(self.raw_mut(), color)
        }
    }

    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe {
            ffi::sfShape_setOutlineThickness(self.raw_mut(), thickness as c_float)
        }
    }

    fn get_fill_color(&self) -> Color {
        unsafe {
            ffi::sfShape_getFillColor(self.raw())
        }
    }

    fn get_outline_color(&self) -> Color {
        unsafe {
            ffi::sfShape_getOutlineColor(self.raw())
        }
    }

    fn get_outline_thickness(&self) -> f32 {
        unsafe {
            ffi::sfShape_getOutlineThickness(self.raw()) as f32
        }
    }

    fn get_point_count(&self) -> u32 {
        unsafe {
            ffi::sfShape_getPointCount(self.raw()) as u32
        }
    }

    fn get_point(&self, index: u32) -> Vector2f {
        unsafe {
            ffi::sfShape_getPoint(self.raw(), index as c_uint)
        }
    }
}

impl<'s> Transformable for BaseShape<'s> {
    fn set_position(&mut self, position: Vector2f) {
        unsafe {
            ffi::sfShape_setPosition(self.raw_mut(), position)
        }
    }

    fn set_rotation(&mut self, angle: f32) {
        unsafe {
            ffi::sfShape_setRotation(self.raw_mut(), angle as c_float)
        }
    }

    fn set_scale(&mut self, scale: Vector2f) {
        unsafe {
            ffi::sfShape_setScale(self.raw_mut(), scale)
        }
    }

    fn set_origin(&mut self, origin: Vector2f) {
        unsafe {
            ffi::sfShape_setOrigin(self.raw_mut(), origin)
        }
    }

    fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfShape_getPosition(self.raw())
        }
    }

    fn get_rotation(&self) -> f32 {
        unsafe {
            ffi::sfShape_getRotation(self.raw()) as f32
        }
    }

    fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfShape_getScale(self.raw())
        }
    }

    fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfShape_getOrigin(self.raw())
        }
    }

    fn move_(&mut self, offset: Vector2f) {
        unsafe {
            ffi::sfShape_move(self.raw_mut(), offset)
        }
    }

    fn rotate(&mut self, angle: f32) {
        unsafe {
            ffi::sfShape_rotate(self.raw_mut(), angle as c_float)
        }
    }

    fn scale(&mut self, factors: Vector2f) {
        unsafe {
            ffi::sfShape_scale(self.raw_mut(), factors)
        }
    }

    fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfShape_getTransform(self.raw())
        }
    }

    fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfShape_getInverseTransform(self.raw())
        }
    }
}

impl<'s> Drawable for BaseShape<'s> {
    fn draw(&self, target: &mut RenderTarget, states: &RenderStates) {
		target.draw_shape_rs(self, states)
    }
}
