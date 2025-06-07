# <h1 align="center">image-tool</h1>
<div align="center">

<img src="https://socialify.git.ci/CandriaJS/image-tool/image?description=1&forks=1&issues=1&language=1&name=1&owner=1&pulls=1&stargazers=1&theme=Light" alt="image-tool" width="640" height="320" />

</div>


## 介绍

`image-tool`是一个图像处理库，提供一些图像处理，如图像旋转，gif变速等

## 安装
```npm
npm install @candriajs/image-tool
```

## 快速使用
```ts
import * as imageTool from '@candriajs/image-tool';
const buffer = await fs.readFile('loli.png');
imageTool.rotate(buffer, 96.0); // 旋转图片，返回Buffer
await fs.writeFile('loli2.png', buffer);
```

## 使用

<details>
  <summary>图片操作：旋转、缩放、裁剪等</summary>

### 📊 查看图片信息
```ts
imageTool.image_info(buffer: Buffer): ImageInfo
```
- `buffer`: 输入的图像二进制数据（如 PNG/JPG/GIF）
返回值：包含以下字段的对象：
- `width`: 图像宽度（单位像素）
- `height`: 图像高度（单位像素）
- `is_multi_frame`: 是否为动图（如 GIF 多帧动画）
- `frameCount`: 动图帧数（仅当 `is_multi_frame` === true 时存在）
- `average_duration`: 动图平均帧间隔时间（单位秒，仅当 `is_multi_frame` === true 时存在）


### 🔁 旋转图片
```ts
imageTool.image_rotate(buffer: Buffer, angle?: number): Buffer
```
- `buffer`: 输入的图像二进制数据（如 PNG/JPG/GIF）
- `angle`: 旋转角度（单位为度），支持任意角度旋转（例如 90.0, 45.0），默认为 90.0
- 返回值：旋转后的图像 Buffer

### 📏 调整大小
```ts
imageTool.image_resize(buffer: Buffer, width?: number, height?: number): Buffer
```
- `buffer`: 输入的图像二进制数据
- `width`: 目标宽度（可选）
- `height`: 目标高度（可选）
- 说明：可指定宽度或高度，支持等比缩放；若仅提供一个参数，则另一个按比例计算

### ✂️ 裁剪图片
```ts
imageTool.image_crop(buffer: Buffer, left: number, top: number, width: number, height: number): Buffer
```
- `buffer`: 输入的图像二进制数据
- `left`: 裁剪区域左上角 X 坐标
- `top`: 裁剪区域左上角 Y 坐标
- `width`: 裁剪区域宽度
- `height`: 裁剪区域高度

### 📷 图像翻转
```ts
// 水平翻转
imageTool.image_flip_horizontal(buffer: Buffer): Buffer

// 垂直翻转
imageTool.image_flip_vertical(buffer: Buffer): Buffer
```

### 🎨 图像效果
```ts
// 灰度化
imageTool.image_grayscale(buffer: Buffer): Buffer

// 反色
imageTool.image_invert(buffer: Buffer): Buffer
```

### 🧩 图像拼接
```ts
// 水平拼接
imageTool.image_merge_horizontal(images: Buffer[]): Buffer

// 垂直拼接
imageTool.image_merge_vertical(images: Buffer[]): Buffer
```
</details>

## 贡献者 👨‍💻👩‍💻

<a href="https://github.com/CandriaJS/image-tool/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=CandriaJS/image-tool" />
</a>

![Alt](https://repobeats.axiom.co/api/embed/04d06e4e2d0cdfb7ef436a681dee7a2c83f199a6.svg "Repobeats analytics image")

# 资源 📚
