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

//! Sounds, streaming (music or custom sources), recording, and spatialization.

pub use audio::sound_buffer::SoundBuffer;
pub use audio::sound_status::SoundStatus;
pub use audio::music::Music;
pub use audio::sound::Sound;
pub use audio::sound_source::{SoundSource, PlayableSound};
pub use audio::sound_stream::{SoundStream, SoundStreamImpl};
pub use audio::sound_recorder::{SoundRecorder, SoundRecorderImpl};
pub use audio::sound_buffer_recorder::SoundBufferRecorder;

/// Sound implementation using reference counting to manage shared resources
/*pub mod rc {
    pub use audio::sound::rc::Sound;
}*/

mod sound_buffer;
pub mod listener;
mod sound_status;
mod music;
mod sound;
mod sound_source;
mod sound_stream;
mod sound_recorder;
mod sound_buffer_recorder;
