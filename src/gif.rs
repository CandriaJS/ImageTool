use image::{AnimationDecoder, DynamicImage, ImageFormat, codecs::gif::GifDecoder};
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
  let decoder = GifDecoder::new(Cursor::new(&image_data))
    .map_err(|error| Error::from_reason(format!("GIF 解码器创建失败: {error}")))?;

  let frames: Result<Vec<_>, _> = decoder.into_frames().collect();
  let frames = frames.map_err(|error| Error::from_reason(format!("GIF 帧读取失败: {error}")))?;

  // 判断帧数是否大于 1
  if frames.len() <= 1 {
    return Err(Error::from_reason(
      "当前不是动图或者动图帧数必须小于1".to_string(),
    ));
  }

  let mut buffers = Vec::with_capacity(frames.len());

  for frame in frames {
    let mut buffer = Vec::new();

    DynamicImage::ImageRgba8(frame.buffer().clone())
      .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
      .map_err(|e| Error::from_reason(format!("PNG 编码失败: {e}")))?;

    buffers.push(Buffer::from(buffer));
  }

  Ok(buffers)
}
