#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod image;
pub use image::image_crop;
pub use image::image_info;
pub use image::image_resize;
