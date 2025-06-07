#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod image;
pub use image::crop_image;
pub use image::image_info;
