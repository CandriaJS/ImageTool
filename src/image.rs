use image::{
  AnimationDecoder,
  DynamicImage::{self, ImageRgba8},
  GenericImage, GenericImageView, ImageDecoder, ImageEncoder, ImageFormat, ImageReader, Rgba,
  RgbaImage,
  codecs::{gif::GifDecoder, png::PngEncoder},
  imageops::FilterType,
};
use napi::{Error, bindgen_prelude::Buffer};
use std::{f32::consts::PI, io::Cursor};

#[napi(object)]
/// 图像的基本信息
pub struct ImageInfo {
  /// 图像宽度
  #[napi(js_name = "width")]
  pub width: u32,

  /// 图像高度
  #[napi(js_name = "height")]
  pub height: u32,

  /// 是否为动图
  #[napi(js_name = "is_multi_frame")]
  pub is_multi_frame: bool,

  /// 动图帧数
  #[napi(js_name = "frame_count")]
  pub frame_count: Option<u32>,

  /// 动图平均帧间隔
  #[napi(js_name = "average_duration")]
  pub average_duration: Option<f64>,
}

/// 获取图片信息
///
/// # 参数
/// * `image_data` - 包含图像数据的Buffer
///
/// # 返回值
/// 返回包含以下字段的 `ImageInfo`：
/// * `width` - 图像宽度
/// * `height` - 图像高度
/// * `is_multi_frame` - 是否为动图
/// * `frame_count` - 动图帧数
/// * `average_duration` - 动图平均帧间隔
///
#[napi(js_name = "image_info")]
pub fn image_info(image_data: Buffer) -> napi::Result<ImageInfo> {
  let cursor = Cursor::new(image_data.as_ref());
  let reader = ImageReader::new(cursor)
    .with_guessed_format()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  match reader.format() {
    // 当图片格式为GIF时
    Some(ImageFormat::Gif) => {
      let cursor = Cursor::new(image_data.as_ref());
      let decoder =
        GifDecoder::new(cursor).map_err(|error| Error::from_reason(error.to_string()))?;

      let (width, height) = decoder.dimensions();
      let frames = decoder
        .into_frames()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| Error::from_reason(error.to_string()))?;

      let frame_count = frames.len() as u32;

      // 计算动图的平均帧率
      let total_delay: f64 = frames
        .iter()
        .map(|frame| {
          let delay = frame.delay().numer_denom_ms();
          (delay.0 as f64 / delay.1 as f64) / 1000.0
        })
        .sum();

      let average_duration = if frame_count > 1 {
        total_delay / (frame_count as f64)
      } else {
        0.0
      };

      Ok(ImageInfo {
        width,
        height,
        is_multi_frame: frame_count > 1,
        frame_count: Some(frame_count),
        average_duration: Some(average_duration),
      })
    }
    // 当图片格式为非Gif时
    _ => {
      let img = reader
        .decode()
        .map_err(|e| Error::from_reason(e.to_string()))?;
      let (width, height) = img.dimensions();

      Ok(ImageInfo {
        width,
        height,
        is_multi_frame: false,
        frame_count: None,
        average_duration: None,
      })
    }
  }
}

/// 裁剪图片
///
/// # 参数
/// - `image_data`: 输入的图像 Buffer
/// - `left`: 裁剪的左上角 X 坐标
/// - `top`: 裁剪的左上角 Y 坐标
/// - `width`: 裁剪的宽度
/// - `height`: 裁剪的高度
///
/// # 返回值
/// - 图像Buffer
///
#[napi(js_name = "image_crop")]
pub fn image_crop(
  image_data: Buffer,
  left: u32,
  top: u32,
  width: u32,
  height: u32,
) -> napi::Result<Buffer> {
  let cursor = Cursor::new(image_data.as_ref());
  let reader = ImageReader::new(cursor)
    .with_guessed_format()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  let img = reader
    .decode()
    .map_err(|error| Error::from_reason(error.to_string()))?;
  let (img_width, img_height) = img.dimensions();

  if left + width > img_width || top + height > img_height {
    return Err(Error::from_reason("裁剪区域超出图像范围".to_string()));
  }

  let cropped_img = img.view(left, top, width, height).to_image();

  let mut output_buffer = Vec::new();
  let encoder = PngEncoder::new(&mut output_buffer);
  let (width, height) = cropped_img.dimensions();
  let raw_pixels = cropped_img.into_raw();

  encoder
    .write_image(&raw_pixels, width, height, image::ColorType::Rgba8.into())
    .map_err(|error| Error::from_reason(error.to_string()))?;

  Ok(Buffer::from(output_buffer))
}

/// 调整图片大小
///
/// # 参数
/// - `buffer`: 输入的图像 Buffer
/// - `width`: 目标宽度
/// - `height`: 目标高度
///
/// # 返回值
/// - 图像Buffer
///
#[napi(js_name = "image_resize")]
pub fn image_resize(
  buffer: Buffer,
  width: Option<u32>,
  height: Option<u32>,
) -> napi::Result<Buffer> {
  let width = width.unwrap();
  let height = height.unwrap();
  let cursor = Cursor::new(buffer.as_ref());
  let decoder = ImageReader::new(cursor)
    .with_guessed_format()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  let image = decoder
    .decode()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  // 缩放图像
  let resized_image = image.resize_exact(width, height, FilterType::Triangle);
  let mut output_buffer = Vec::new();
  let encoder = PngEncoder::new(&mut output_buffer);
  let (width, height) = resized_image.dimensions();
  let raw_pixels = resized_image.into_rgba8().into_raw();

  encoder
    .write_image(&raw_pixels, width, height, image::ColorType::Rgba8.into())
    .map_err(|error| Error::from_reason(error.to_string()))?;

  Ok(Buffer::from(output_buffer))
}

/// 旋转图片
///
/// # 参数
/// - `image_data`: 输入的图像 Buffer
/// - `degrees`: 旋转的角度, 可选参数, 默认为 90.0
///
/// # 返回值
/// - 图像Buffer
///
#[napi(js_name = "image_rotate")]
pub fn image_rotate(image_data: Buffer, degrees: Option<f64>) -> napi::Result<Buffer> {
  let degrees = degrees.unwrap_or(90.0);
  let cursor = Cursor::new(image_data.as_ref());
  let decoder = ImageReader::new(cursor)
    .with_guessed_format()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  let image = decoder
    .decode()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  let (width, height) = image.dimensions();

  // 计算旋转后需要的画布大小
  let radians = (degrees as f32) * PI as f32 / 180.0;
  let sin_rad = radians.sin().abs();
  let cos_rad = radians.cos().abs();

  // 计算旋转后的新尺寸
  let new_width = (width as f32 * cos_rad + height as f32 * sin_rad).ceil() as u32;
  let new_height = (width as f32 * sin_rad + height as f32 * cos_rad).ceil() as u32;

  let white_pixel = Rgba([255, 255, 255, 255]);
  let mut rotated = ImageRgba8(RgbaImage::from_pixel(new_width, new_height, white_pixel));

  let cos_rad = radians.cos();
  let sin_rad = radians.sin();

  let half_width = width as f32 * 0.5;
  let half_height = height as f32 * 0.5;
  let half_new_width = new_width as f32 * 0.5;
  let half_new_height = new_height as f32 * 0.5;

  let width_i32 = width as i32;
  let height_i32 = height as i32;

  for y in 0..new_height {
    let dy = y as f32 - half_new_height;

    for x in 0..new_width {
      let dx = x as f32 - half_new_width;

      // 计算原始坐标
      let old_x = (dx * cos_rad + dy * sin_rad + half_width) as i32;
      let old_y = (-dx * sin_rad + dy * cos_rad + half_height) as i32;

      // 检查坐标是否在图像范围内
      if (old_x | old_y) >= 0 && old_x < width_i32 && old_y < height_i32 {
        let pixel = image.get_pixel(old_x as u32, old_y as u32);
        rotated.put_pixel(x, y, pixel);
      }
    }
  }

  let mut output_buffer = Vec::new();
  let encoder = PngEncoder::new(&mut output_buffer);

  let (width, height) = rotated.dimensions();
  let raw_pixels = rotated.into_rgba8().into_raw();

  encoder
    .write_image(&raw_pixels, width, height, image::ColorType::Rgba8.into())
    .map_err(|error| Error::from_reason(error.to_string()))?;

  Ok(Buffer::from(output_buffer))
}

/// 水平翻转图片
///
/// # 参数
/// - `image_data`: 输入的图像 Buffer
///
/// # 返回值
/// - 图像Buffer
///
#[napi(js_name = "image_flip_horizontal")]
pub fn image_flip_horizontal(image_data: Buffer) -> napi::Result<Buffer> {
  let cursor = Cursor::new(image_data.as_ref());
  let decoder = ImageReader::new(cursor)
    .with_guessed_format()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  let image = decoder
    .decode()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  // 水平翻转图像
  let flipped_image = image.fliph();

  let mut output_buffer = Vec::new();
  let encoder = PngEncoder::new(&mut output_buffer);

  let (width, height) = flipped_image.dimensions();
  let raw_pixels = flipped_image.into_rgba8().into_raw();

  encoder
    .write_image(&raw_pixels, width, height, image::ColorType::Rgba8.into())
    .map_err(|error| Error::from_reason(error.to_string()))?;

  Ok(Buffer::from(output_buffer))
}

/// 垂直翻转图片
///
/// # 参数
/// - `image_data`: 输入的图像 Buffer
///
/// # 返回值
/// - 图像Buffer
///
#[napi(js_name = "image_flip_vertical")]
pub fn image_flip_vertical(image_data: Buffer) -> napi::Result<Buffer> {
  let cursor = Cursor::new(image_data.as_ref());
  let decoder = ImageReader::new(cursor)
    .with_guessed_format()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  let image = decoder
    .decode()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  // 垂直翻转图像
  let flipped_image = image.flipv();

  let mut output_buffer = Vec::new();
  let encoder = PngEncoder::new(&mut output_buffer);

  let (width, height) = flipped_image.dimensions();
  let raw_pixels = flipped_image.into_rgba8().into_raw();

  encoder
    .write_image(&raw_pixels, width, height, image::ColorType::Rgba8.into())
    .map_err(|error| Error::from_reason(error.to_string()))?;

  Ok(Buffer::from(output_buffer))
}

/// 灰度化图片
///
/// # 参数
/// - `image_data`: 输入的图像 Buffer
///
/// # 返回值
/// - 图像Buffer
///
#[napi(js_name = "image_grayscale")]
pub fn image_grayscale(image_data: Buffer) -> napi::Result<Buffer> {
  let cursor = Cursor::new(image_data.as_ref());
  let decoder = ImageReader::new(cursor)
    .with_guessed_format()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  // 灰度化
  let image = decoder
    .decode()
    .map_err(|error| Error::from_reason(error.to_string()))?
    .grayscale()
    .into_rgba8();

  let (width, height) = image.dimensions();
  let raw_pixels = image.into_raw();

  let mut output_buffer = Vec::new();
  let encoder = PngEncoder::new(&mut output_buffer);

  encoder
    .write_image(&raw_pixels, width, height, image::ColorType::Rgba8.into())
    .map_err(|error| Error::from_reason(error.to_string()))?;

  Ok(Buffer::from(output_buffer))
}

/// 反色图片
///
/// # 参数
/// - `image_data`: 输入的图像 Buffer
///
/// # 返回值
/// - 图像Buffer
///
#[napi(js_name = "image_invert")]
pub fn image_invert(image_data: Buffer) -> napi::Result<Buffer> {
  let cursor = Cursor::new(image_data.as_ref());
  let decoder = ImageReader::new(cursor)
    .with_guessed_format()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  let mut image = decoder
    .decode()
    .map_err(|error| Error::from_reason(error.to_string()))?
    .into_rgba8();

  // 反色像素
  for pixel in image.pixels_mut() {
    let [r, g, b, a] = pixel.0;
    pixel.0 = [255 - r, 255 - g, 255 - b, a];
  }

  let rgba_image = DynamicImage::ImageRgba8(image);
  let mut output_buffer = Vec::new();
  let encoder = PngEncoder::new(&mut output_buffer);
  let (width, height) = rgba_image.dimensions();
  let raw_pixels = rgba_image.into_rgba8().into_raw();

  encoder
    .write_image(&raw_pixels, width, height, image::ColorType::Rgba8.into())
    .map_err(|error| Error::from_reason(error.to_string()))?;

  Ok(Buffer::from(output_buffer))
}
