use gif::{Decoder, Encoder, Frame, Repeat};
use image::{DynamicImage, GenericImageView, ImageFormat};
use napi::{Error, bindgen_prelude::Buffer};
use std::io::Cursor;

/// gif分解
///
/// # 参数
/// * `image_data` - 包含图像数据的Buffer
///
/// # 返回值
/// - 图像Buffer数组
///
#[napi(js_name = "gif_split")]
pub fn gif_split(image_data: Buffer) -> Result<Vec<Buffer>, Error> {
  let mut decoder = Decoder::new(Cursor::new(&image_data))
    .map_err(|error| Error::from_reason(format!("GIF 解码器创建失败: {error}")))?;

  // 收集所有帧
  let mut frames = Vec::new();
  while let Some(frame) = decoder
    .read_next_frame()
    .map_err(|error| Error::from_reason(format!("GIF 帧读取失败: {error}")))?
  {
    frames.push(frame.to_owned());
  }

  // 判断帧数是否大于 1
  if frames.len() <= 1 {
    return Err(Error::from_reason(
      "当前不是动图或者动图帧数必须小于1".to_string(),
    ));
  }

  let mut buffers = Vec::with_capacity(frames.len());

  for frame in frames {
    let mut buffer = Vec::new();

    let rgba = image::RgbaImage::from_raw(
      frame.width as u32,
      frame.height as u32,
      frame.buffer.to_vec(),
    )
    .ok_or_else(|| Error::from_reason("图像转换失败"))?;

    DynamicImage::ImageRgba8(rgba)
      .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
      .map_err(|e| Error::from_reason(format!("PNG 编码失败: {e}")))?;

    buffers.push(Buffer::from(buffer));
  }

  Ok(buffers)
}

/// gif合成
///
/// # 参数
/// * `images` - 包含图像数据的Buffer数组
///
/// # 返回值
/// - 图像Buffer
///

#[napi(js_name = "gif_merge")]
pub fn gif_merge(images: Vec<Buffer>, duration: Option<f64>) -> Result<Buffer, Error> {
  if images.is_empty() {
    return Err(Error::from_reason("输入图片数组不能为空"));
  }
  let first_img = image::load_from_memory(&images[0])
    .map_err(|e| Error::from_reason(format!("首帧图片加载失败: {e}")))?;
  let (width, height) = first_img.dimensions();

  let mut output = Vec::new();
  let mut encoder = Encoder::new(&mut output, width as u16, height as u16, &[])
    .map_err(|e| Error::from_reason(format!("GIF 编码器创建失败: {e}")))?;
  encoder
    .set_repeat(Repeat::Infinite)
    .map_err(|e| Error::from_reason(format!("设置 GIF 循环失败: {e}")))?;
  let delay = ((duration.unwrap_or(0.05) * 100.0) as u16).max(1);

  for img_buffer in images {
    let img = image::load_from_memory(&img_buffer)
      .map_err(|e| Error::from_reason(format!("图片加载失败: {e}")))?;
    let resized = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let rgba = resized.into_rgba8();
    let mut frame = Frame::from_rgba_speed(width as u16, height as u16, &mut rgba.into_raw(), 10);
    frame.delay = delay;
    encoder
      .write_frame(&frame)
      .map_err(|e| Error::from_reason(format!("帧写入失败: {e}")))?;
  }

  drop(encoder);
  Ok(Buffer::from(output))
}
