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

/*!
 * The base module of SFML, providing vector types and time measurement.
 *
 * The C++ version of SFML also provides Unicode and threading functionality.
 * For these features, use the Rust standard library instead.
 */

// Vector support
mod vector2;
mod vector3;
pub use self::vector2::{Vector2, Vector2u, Vector2i, Vector2f};
pub use self::vector3::{Vector3, Vector3f};

// Time, clock, and sleep support
mod time;
mod clock;
pub use self::time::Time;
pub use self::clock::Clock;

// Custom input stream support
mod stream;
pub use self::stream::InputStream;

/// Make the current thread sleep for the given duration.
pub fn sleep(time: Time) {
    use std::thread::sleep_ms;
    sleep_ms(time.as_milliseconds() as u32)
}
