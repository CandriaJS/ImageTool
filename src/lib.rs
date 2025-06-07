#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod image;
pub use image::image_color_mask;
pub use image::image_crop;
pub use image::image_flip_horizontal;
pub use image::image_flip_vertical;
pub use image::image_grayscale;
pub use image::image_info;
pub use image::image_invert;
pub use image::image_merge_horizontal;
pub use image::image_merge_vertical;
pub use image::image_resize;
pub use image::image_rotate;

mod gif;
pub use gif::gif_merge;
pub use gif::gif_reverse;
pub use gif::gif_split;
