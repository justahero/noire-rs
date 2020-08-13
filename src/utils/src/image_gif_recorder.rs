//! ImageSetRecorder

use image::{Delay, RgbImage, ImageFormat};
use std::{fmt, path::Path};

#[derive(Debug)]
pub enum RecordError {
    /// An invalid animation frame was given
    ImageError(image::ImageError),
    /// Raised when too many frames are stored
    FramesLimitReached(u32),
}

impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RecordError::ImageError(err) => format!("Image Error: {}", err),
            RecordError::FramesLimitReached(frames) => format!("Limit of frames {} reached", frames),
        };
        write!(f, "{}", s)
    }
}

pub type RecordResult<T> = Result<T, RecordError>;

/// A recorder to assemble a number of images to an animated GIF image.
///
/// It requires that all images have the same dimensions, the frame rate and number of frames
/// should not be too high or the resulting GIF image will become large. The main purpose
/// of the recorder is to generate a short GIF, preferably something in a loop that can be
/// easily shared or posted.
///
pub struct ImageSetRecorder<P: AsRef<Path>> {
    /// The path to store all images into
    pub path: P,
    /// The number of animation frames to expect. This is basically a safe guard to keep size in limit
    pub num_frames: u32,
    /// Number of animated frames written to output
    frames_written: u32,
}

impl<P: AsRef<Path>> ImageSetRecorder<P> {
    /// Creates a new instance of the Image Recorder
    ///
    /// # Properties
    /// * `path` - The path reference to store all images into
    /// * `num_frames` - The number of frames to store 
    pub fn new(path: P, num_frames: u32) -> Self {
        Self {
            path,
            num_frames,
            frames_written: 0,
        }
    }

    /// Returns the number of frames written to output
    pub fn frames_written(&self) -> u32 {
        self.frames_written
    }

    /// Returns true if the number of frames written is the same as expected to be written
    pub fn complete(&self) -> bool {
        self.frames_written == self.num_frames
    }

    /// Saves the image data to the output folder, all images are stored as PNG files.
    ///
    /// Add a new animation frame to the GIF.
    /// The given frame contains information about frame rate.
    pub fn save_image(&mut self, image: RgbImage) -> RecordResult<()> {
        if self.frames_written >= self.num_frames {
            return Err(RecordError::FramesLimitReached(self.num_frames));
        }

        let path = Path::join(self.path.as_ref(), &format!("image-{:04}.png", self.frames_written));
        image.save_with_format(path, ImageFormat::Png).map_err(|e| RecordError::ImageError(e))?;

        self.frames_written += 1;

        Ok(())
    }
}
