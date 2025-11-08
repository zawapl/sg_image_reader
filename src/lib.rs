//! A library for reading sg3 files used in some Impressions Games city building games (Cesar 3, Pharaoh, Zeus, Emperor etc.).
//!
//! Documentation of the format can be found at https://github.com/bvschaik/citybuilding-tools/wiki/SG-file-format#image-data.
//!
//! Simple usage:
//! ```rust
//! use sg_image_reader::{SgFile, VecImageBuilderFactory};
//!
//! let path = "path-to-file";
//! let (sg_file, pixel_data): (SgFile, Vec<Vec<u8>>) = SgFile::load_fully(path, &VecImageBuilderFactory)?;
//! ```
//!
//! The basic example provides a vector of raw bytes for all the images.
//! The raw bytes can be used to construct required image structs (with the image library of your choosing).
//! It is also possible to construct the required images directly by implementing the [`ImageBuilderFactory`] trait and passing it instead of the [`VecImageBuilderFactory`].
//!
//! Pixel data can also be loaded for one image at a time, see `viewer` example for an example of that
//! ```rust
//! use std::fs::File;
//! use std::io::BufReader;
//! use sg_image_reader::{SgFile, VecImageBuilderFactory};
//!
//! // Load just the metadata
//! let sg_file = SgFile::load_from_path(path)?;
//!
//! // Select the image we want to load pixel data for
//! let image = &sg_file.images[11];
//!
//! // Get the path of the file where that data is located
//! let path = sg_file.get_555_file_path(image.album_id as usize, image.is_external());
//!
//! // Create a new reader
//! let mut buf_reader = BufReader::new(File::open(path)?);
//!
//! // Load pixel data for that specific image
//! let pixel_data = image.load_image(&mut buf_reader, &VecImageBuilderFactory);
//! ```
pub use error::{Result, SgImageError};
pub use image_builder::*;
pub use sg_album::SgAlbum;
pub use sg_file::SgFile;
pub use sg_image_metadata::SgImageMetadata;
pub(crate) use utils::*;

mod error;
mod image_builder;
mod sg_album;
mod sg_file;
mod sg_image_metadata;
mod utils;
