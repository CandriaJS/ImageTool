use gif::{Decoder, Encoder, Frame, Repeat};
use image::{DynamicImage, GenericImageView, ImageFormat, RgbaImage, load_from_memory};
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
pub fn gif_split(image_data: Buffer) -> napi::Result<Vec<Buffer>> {
  let mut decoder = Decoder::new(Cursor::new(image_data.as_ref()))
    .map_err(|error| Error::from_reason(format!("GIF解码失败: {error}")))?;

  let mut frames = Vec::new();
  while let Some(frame) = decoder
    .read_next_frame()
    .map_err(|error| Error::from_reason(format!("GIF帧读取失败: {error}")))?
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

  let global_palette = decoder.global_palette().map(|p| p.to_vec());

  for frame in frames {
    let mut buffer = Vec::new();
    let mut rgba_data = Vec::with_capacity(frame.width as usize * frame.height as usize * 4);
    let palette = if let Some(frame_palette) = frame.palette {
      frame_palette
    } else if let Some(ref global_pal) = global_palette {
      global_pal.clone()
    } else {
      return Err(Error::from_reason("获取图像调色板"));
    };

    for &pixel_idx in frame.buffer.iter() {
      let color_idx = pixel_idx as usize * 3;
      if color_idx + 2 < palette.len() {
        // 检查是否是透明像素，如果有则设置为透明，否则则为不透明
        let alpha = if let Some(transparent_idx) = frame.transparent {
          if pixel_idx == transparent_idx { 0 } else { 255 }
        } else {
          255
        };

        rgba_data.push(palette[color_idx]);
        rgba_data.push(palette[color_idx + 1]);
        rgba_data.push(palette[color_idx + 2]);
        rgba_data.push(alpha);
      }
    }

    let rgba = RgbaImage::from_raw(frame.width as u32, frame.height as u32, rgba_data)
      .ok_or_else(|| Error::from_reason("图像创建失败"))?;

    DynamicImage::ImageRgba8(rgba)
      .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
      .map_err(|error| Error::from_reason(format!("图像编码失败: {error}")))?;

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
pub fn gif_merge(images: Vec<Buffer>, duration: Option<f64>) -> napi::Result<Buffer> {
  if images.is_empty() {
    return Err(Error::from_reason("输入图片数组不能为空"));
  }
  let first_img = load_from_memory(&images[0])
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
    let img = load_from_memory(&img_buffer)
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

/// gif倒放
///
/// # 参数
/// * `image_data` - 包含图像数据的Buffer
///
/// # 返回值
/// - 图像Buffer数组
///
#[napi(js_name = "gif_reverse")]
pub fn gif_reverse(image_data: Buffer) -> napi::Result<Buffer> {
  let cursor = Cursor::new(image_data.as_ref());
  let mut decoder = Decoder::new(cursor).map_err(|error| Error::from_reason(error.to_string()))?;
  let global_palette = decoder.global_palette().map(|p| p.to_vec());

  let (width, height) = (decoder.width() as u16, decoder.height() as u16);
  let mut frames = Vec::new();

  while let Some(frame) = decoder
    .read_next_frame()
    .map_err(|error| Error::from_reason(error.to_string()))?
  {
    frames.push(frame.to_owned());
  }

  if frames.len() <= 1 {
    return Err(Error::from_reason(
      "当前不是动图或者动图帧数必须小于1".to_string(),
    ));
  }

  let mut output_buffer = Vec::new();
  {
    let mut encoder = Encoder::new(
      &mut output_buffer,
      width,
      height,
      global_palette.as_deref().unwrap_or(&[]),
    )
    .map_err(|error| Error::from_reason(error.to_string()))?;

    encoder
      .set_repeat(Repeat::Infinite)
      .map_err(|error| Error::from_reason(error.to_string()))?;

    for frame in frames.iter().rev() {
      let mut new_frame = frame.clone();
      if let Some(palette) = frame.palette.as_ref() {
        new_frame.palette = Some(palette.clone());
      }
      encoder
        .write_frame(&new_frame)
        .map_err(|error| Error::from_reason(error.to_string()))?;
    }
  }

  Ok(Buffer::from(output_buffer))
}

/// gif变速
///
/// # 参数
/// - `image_data`: 输入的 GIF 图像 Buffer
/// - `duration`: 每帧的间隔时间（秒），如果为 0 则保持原速
///
/// # 返回值
/// - 图像Buffer
///
#[napi(js_name = "gif_change_duration")]
pub fn gif_change_duration(image_data: Buffer, duration: f64) -> napi::Result<Buffer> {
  let cursor = Cursor::new(image_data.as_ref());
  let mut decoder = Decoder::new(cursor).map_err(|error| Error::from_reason(error.to_string()))?;
  let global_palette = decoder.global_palette().map(|p| p.to_vec());

  let (width, height) = (decoder.width() as u16, decoder.height() as u16);
  let mut frames = Vec::new();

  while let Some(frame) = decoder
    .read_next_frame()
    .map_err(|error| Error::from_reason(error.to_string()))?
  {
    frames.push(frame.to_owned());
  }

  if frames.len() <= 1 {
    return Err(Error::from_reason(
      "当前不是动图或者动图帧数必须小于1".to_string(),
    ));
  }

  let mut output_buffer = Vec::new();
  {
    let mut encoder = Encoder::new(
      &mut output_buffer,
      width,
      height,
      global_palette.as_deref().unwrap_or(&[]),
    )
    .map_err(|error| Error::from_reason(error.to_string()))?;

    encoder
      .set_repeat(Repeat::Infinite)
      .map_err(|error| Error::from_reason(error.to_string()))?;
    for frame in frames.iter() {
      let mut new_frame = frame.clone();
      if let Some(palette) = frame.palette.as_ref() {
        new_frame.palette = Some(palette.clone());
      }

      if duration > 0.0 {
        new_frame.delay = (duration * 100.0).round() as u16;
      }

      encoder
        .write_frame(&new_frame)
        .map_err(|error| Error::from_reason(error.to_string()))?;
    }
  }

  Ok(Buffer::from(output_buffer))
}
