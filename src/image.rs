use image::{
  AnimationDecoder, DynamicImage, GenericImageView, ImageDecoder, ImageFormat, ImageReader,
  codecs::gif::GifDecoder, codecs::jpeg::JpegEncoder, imageops::FilterType,
};
use napi::{Error, bindgen_prelude::Buffer};
use std::io::Cursor;

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

  /// 动图总帧数
  #[napi(js_name = "frame_count")]
  pub frame_count: Option<u32>,

  /// 动图平均帧间隔时间
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
/// * `frame_count` - 帧数（静态图为1）
/// * `average_duration` - 平均帧延迟（毫秒）
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

/// 裁剪图像并返回裁剪后的 Buffer
///
/// # 参数
/// - `image_data`: 输入的图像 Buffer
/// - `left`: 裁剪的左上角 X 坐标
/// - `top`: 裁剪的左上角 Y 坐标
/// - `width`: 裁剪的宽度
/// - `height`: 裁剪的高度
///
/// # 返回值
/// - 成功时返回裁剪后的图像Buffer
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

  // 转换为RGB8格式
  let rgb_img = DynamicImage::ImageRgba8(cropped_img).into_rgb8();

  let mut output_buffer = Vec::new();
  let mut encoder = JpegEncoder::new_with_quality(&mut output_buffer, 100);

  encoder
    .encode(
      rgb_img.as_raw(),
      width,
      height,
      image::ExtendedColorType::Rgb8,
    )
    .map_err(|e| Error::from_reason(e.to_string()))?;

  Ok(Buffer::from(output_buffer))
}

/// 调整图像尺寸并返回缩放后的 Buffer
///
/// # 参数
/// - `buffer`: 输入的图像 Buffer
/// - [width](file://d:\project\Rust\image-tool\dist\index.d.ts#L8-L8): 目标宽度
/// - [height](file://d:\project\Rust\image-tool\dist\index.d.ts#L10-L10): 目标高度
///
/// # 返回值
/// - 成功时返回缩放后的图像 Buffer（JPEG 格式）
///
#[napi(js_name = "image_resize")]
pub fn image_resize(buffer: Buffer, width: u32, height: u32) -> napi::Result<Buffer> {
  let cursor = Cursor::new(buffer.as_ref());
  let decoder = ImageReader::new(cursor)
    .with_guessed_format()
    .map_err(|e| Error::from_reason(e.to_string()))?;

  let image = decoder
    .decode()
    .map_err(|error| Error::from_reason(error.to_string()))?;

  // 缩放图像
  let resized_image = image.resize_exact(width, height, FilterType::Triangle);

  // 将图像编码为 JPEG
  let mut output_buffer = Vec::new();
  let mut encoder = JpegEncoder::new_with_quality(&mut output_buffer, 100);

  encoder
    .encode_image(&resized_image)
    .map_err(|e| Error::from_reason(e.to_string()))?;

  Ok(Buffer::from(output_buffer))
}
