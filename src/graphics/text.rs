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

use libc::{c_float, c_uint, size_t};

use graphics::{RenderTarget, Font, FloatRect, Drawable, Transformable,
               Color, Transform, RenderStates, TextStyle};
use system::Vector2f;

use ffi::Foreign;
use ffi::graphics as ffi;

/// Graphical text that can be drawn to a render target.
///
/// Text implements all the properties of `Transformable`, and adds
/// text-specific properties such as the font to use, the character size, the
/// font style (bold, italic, underline, strikethrough), the global color, and
/// the string to display. It also provides convenience functions to calculate
/// the graphical size of the text, or to get the global position of a given
/// character.
///
/// `Text` works in combination with the `Font` class, which loads and provides
/// the glyphs (visual characters) of a given font. The separation of `Font` and
/// `Text` allows more flexibility and better performances: indeed a `Font` is a
/// heavy resource, and any operation on it is slow (often too slow for
/// real-time applications). On the other side, a `Text` is a lightweight object
/// which can combine the glyphs data and metrics of a `Font` to display any
/// text on a render target.
///
/// See also the note on coordinated and undistorted rendering in
/// `Transformable`.
pub struct Text<'s> {
    text: Foreign<ffi::sfText>,
    font: Option<&'s Font>
}

impl<'s> Text<'s> {
	/// Create a new empty text.
    ///
    /// Returns Some(Text) or None on failure.
    pub fn new() -> Option<Text<'s>> {
        unsafe {
			Foreign::new(ffi::sfText_create())
		}.map(|text| Text {
			text: text,
			font: None
		})
    }

    /// Create a new text from a string, font, and size.
    ///
	/// Note that if the used font is a bitmap font, it is not scalable, thus
	/// not all requested sizes will be available to use. This needs to be taken
	/// into consideration when setting the character size. If you need to
	/// display text of a certain size, make sure the corresponding bitmap font
	/// that supports that size is used.
	///
	/// The default value for `character_size` is 30 pixels.
	///
    /// Returns Some(Text) or None on failure.
    pub fn new_init(string: &str,
                    font: &'s Font,
                    character_size: u32) -> Option<Text<'s>> {
		Text::new().map(|mut text| {
			text.set_string(string);
			text.set_font(font);
			text.set_character_size(character_size);
			text
		})
    }
	
	fn raw(&self) -> &ffi::sfText { self.text.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfText { self.text.as_mut() }
	#[doc(hidden)]
    pub unsafe fn unwrap(&self) -> &ffi::sfText { self.raw() }

    /// Copy an existing Text.
    ///
    /// Returns Some(Text) or None on failure.
    pub fn clone_opt(&self) -> Option<Text<'s>> {
        unsafe {
			Foreign::new(ffi::sfText_copy(self.raw()))
		}.map(|text| Text {
			text: text,
			font: self.font
		})
    }

	/// Set the text's string.
    ///
    /// A text's string is empty by default.
    pub fn set_string(&mut self, string: &str) {
		let vec = ::ffi::to_utf32(string);
        unsafe {
            ffi::sfText_setUnicodeString(self.raw_mut(), vec.as_ptr())
        }
    }

    /// Get the text's string.
    pub fn get_string(&self) -> String {
		unsafe {
			let pointer = ffi::sfText_getUnicodeString(self.raw());
			let mut len = 0;
			while *pointer.offset(len) != 0 {
				len += 1;
			}
			::std::slice::from_raw_parts(pointer, len as usize)
		}.iter().filter_map(|&ch| ::std::char::from_u32(ch)).collect()
    }

	/// Get the character size, in pixels.
    pub fn get_character_size(&self) -> u32 {
        unsafe { ffi::sfText_getCharacterSize(self.raw()) as u32 }
    }

    /// Set the text's font.
    pub fn set_font(&mut self, font: &'s Font) {
        self.font = Some(font);
        unsafe { ffi::sfText_setFont(self.raw_mut(), font.unwrap()) }
    }

    /// Set the text's style.
    ///
    /// You can pass a combination of one or more styles, for
    /// example `BOLD | ITALIC`. The default style is `REGULAR`.
    pub fn set_style(&mut self, style: TextStyle) {
        unsafe { ffi::sfText_setStyle(self.raw_mut(), style.bits()) }
    }

    /// Set the character size in pixels.
    ///
    /// The default size is 30.
    ///
    /// Note that if the used font is a bitmap font, it is not scalable, thus
	/// not all requested sizes will be available to use. This needs to be taken
	/// into consideration when setting the character size. If you need to
	/// display text of a certain size, make sure the corresponding bitmap font
	/// that supports that size is used.
    pub fn set_character_size(&mut self, size: u32) {
        unsafe { ffi::sfText_setCharacterSize(self.raw_mut(), size as c_uint) }
    }

    /// Get the text's style.
    pub fn get_style(&self) -> TextStyle {
		unsafe { TextStyle::from_bits_truncate(ffi::sfText_getStyle(self.raw())) }
    }

    /// Get the text's font.
	///
    /// If the text has no font attached, None is returned. The returned
	/// reference is immutable, which means that you can't modify the font when
	/// you retrieve it with this function.
    pub fn get_font(&self) -> Option<&'s Font> {
        self.font
    }

    /// Set the global color of the text.
    ///
    /// By default, the text's color is opaque white.
    pub fn set_color(&mut self, color: Color) {
        unsafe { ffi::sfText_setColor(self.raw_mut(), color) }
    }

    /// Get the global color of the text.
    pub fn get_color(&self) -> Color {
        unsafe { ffi::sfText_getColor(self.raw()) }
    }

    /// Return the position of the `index`-th character.
    ///
    /// This function computes the visual position of a character
    /// from its index in the string. The returned position is
    /// in global coordinates (translation, rotation, scale and
    /// origin are applied).
    /// If `index` is out of range, the position of the end of
    /// the string is returned.
    pub fn find_character_pos(&self, index: u64) -> Vector2f {
        unsafe { ffi::sfText_findCharacterPos(self.raw(), index as size_t) }
    }

    /// Get the local bounding rectangle of the text.
    ///
    /// The returned rectangle is in local coordinates, which means
    /// that it ignores the transformations (translation, rotation,
    /// scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// entity in the entity's coordinate system.
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfText_getLocalBounds(self.raw()) }
    }

    /// Get the global bounding rectangle of the text.
    ///
    /// The returned rectangle is in global coordinates, which means
    /// that it takes in account the transformations (translation,
    /// rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// text in the global 2D world's coordinate system.
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfText_getGlobalBounds(self.raw()) }
    }
}

impl<'s> Transformable for Text<'s> {
    fn scale(&mut self, factors: Vector2f) {
        unsafe { ffi::sfText_scale(self.raw_mut(), factors) }
    }
    fn set_scale(&mut self, scale: Vector2f) {
        unsafe { ffi::sfText_setScale(self.raw_mut(), scale) }
    }
    fn move_(&mut self, offset: Vector2f) {
        unsafe { ffi::sfText_move(self.raw_mut(), offset) }
    }
    fn set_position(&mut self, position: Vector2f) {
        unsafe { ffi::sfText_setPosition(self.raw_mut(), position) }
    }
    fn set_origin(&mut self, origin: Vector2f) {
        unsafe { ffi::sfText_setOrigin(self.raw_mut(), origin) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { ffi::sfText_getScale(self.raw()) }
    }
    fn get_origin(&self) -> Vector2f {
        unsafe { ffi::sfText_getOrigin(self.raw()) }
    }
    fn get_position(&self) -> Vector2f {
        unsafe { ffi::sfText_getPosition(self.raw()) }
    }
    fn get_transform(&self) -> Transform {
        unsafe { ffi::sfText_getTransform(self.raw()) }
    }
    fn get_inverse_transform(&self) -> Transform {
        unsafe { ffi::sfText_getInverseTransform(self.raw()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfText_setRotation(self.raw_mut(), angle as c_float) }
    }
    fn get_rotation(&self) -> f32 {
        unsafe { ffi::sfText_getRotation(self.raw()) as f32 }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfText_rotate(self.raw_mut(), angle as c_float) }
    }
}

impl<'s> Clone for Text<'s> {
    fn clone(&self) -> Text<'s> {
		self.clone_opt().expect("Failed to clone Text")
    }
}

impl<'s> Drawable for Text<'s> {
    fn draw(&self, target: &mut RenderTarget, states: &RenderStates) {
        target.draw_text_rs(self, states)
    }
}
