#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod image;
pub use image::image_crop;
pub use image::image_flip_horizontal;
pub use image::image_flip_vertical;
pub use image::image_grayscale;
pub use image::image_info;
pub use image::image_resize;
pub use image::image_rotate;
