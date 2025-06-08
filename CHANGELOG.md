# 变更日志

## [0.3.3](https://github.com/CandriaJS/image-tool/compare/v0.3.2...v0.3.3) (2025-06-08)


### 🐛 错误修复

* **gif:** 修复 GIF 图像解码和帧处理的问题 ([acbf22d](https://github.com/CandriaJS/image-tool/commit/acbf22d1c844f7feec59e7202f2b57c54b88d874))

## [0.3.2](https://github.com/CandriaJS/image-tool/compare/v0.3.1...v0.3.2) (2025-06-07)


### 🔧 其他更新

* 在发布工作流中添加导入npm包文件的步骤 ([f57d750](https://github.com/CandriaJS/image-tool/commit/f57d750c0e2051aaba2ce93ce8b2169c5b5b37b9))

## [0.3.1](https://github.com/CandriaJS/image-tool/compare/v0.3.0...v0.3.1) (2025-06-07)


### 🐛 错误修复

* 修正gif变速描述 ([52539f4](https://github.com/CandriaJS/image-tool/commit/52539f4f09c1bed93ba425ce92c51025bb73472e))

## [0.3.0](https://github.com/CandriaJS/image-tool/compare/v0.2.0...v0.3.0) (2025-06-07)


### ✨ 新功能

* **gif:** 添加 gif_merge 函数实现 GIF 图像合成 ([67dbe57](https://github.com/CandriaJS/image-tool/commit/67dbe570be24135ea7961acb3311ed643b28dd0b))
* **gif:** 添加gif倒放功能 ([5ce0045](https://github.com/CandriaJS/image-tool/commit/5ce00451e8deff70f2042bf839f43163e3f27f7e))
* **gif:** 添加修GIF 变速的功能 ([654035e](https://github.com/CandriaJS/image-tool/commit/654035ec64fe2960da56b04b8d6867a0c81eaba5))
* **image:** 添加 gif_split 函数实现 GIF 图像分解 ([de719c5](https://github.com/CandriaJS/image-tool/commit/de719c5029d0d974d68d0a9b8ba1cd26ad20a271))


### 📝 文档更新

* **gif:** 更新 README.md，添加 GIF 操作相关功能说明 ([9da703c](https://github.com/CandriaJS/image-tool/commit/9da703c717fdcea1dbadf263c3333c613fa9817e))


### 🎡 持续集成

* 同步子包版本 ([4b2d945](https://github.com/CandriaJS/image-tool/commit/4b2d945fa115c80d398a81fbe29ebec544575dc5))

## [0.2.0](https://github.com/CandriaJS/image-tool/compare/v0.1.11...v0.2.0) (2025-06-07)


### ✨ 新功能

* **image:** 旋转图片功能 ([be4071b](https://github.com/CandriaJS/image-tool/commit/be4071bebedefc2e5917b7c4065cef7635188943))
* **image:** 添加图像信息获取和裁剪功能 ([32bff8f](https://github.com/CandriaJS/image-tool/commit/32bff8f41ef0c7ed1d1022ec5ed04e5850f25ead))
* **image:** 添加图像灰度化功能 ([6ff5cb3](https://github.com/CandriaJS/image-tool/commit/6ff5cb3e73d39bc0e2c5d96c7923ed85d28f7e06))
* **image:** 添加图像缩放功能并优化错误处理 ([2d050dc](https://github.com/CandriaJS/image-tool/commit/2d050dce6b5d6a9295fe4b934cf003767387f2ef))
* **image:** 添加图像翻转功能 ([c4ff102](https://github.com/CandriaJS/image-tool/commit/c4ff1025b118af0bbc0d65f8fc757d4da3687132))
* **image:** 添加图像颜色滤镜功能 ([363076e](https://github.com/CandriaJS/image-tool/commit/363076ea2a5e61537f60ed1016dd9f5245412787))
* **image:** 添加图片反色处理功能 ([12b5cbd](https://github.com/CandriaJS/image-tool/commit/12b5cbd7b0be7b30b9baa54a405d795cc7c996c8))
* **image:** 添加图片拼接功能 ([218dc5b](https://github.com/CandriaJS/image-tool/commit/218dc5bce74c605a27f4215f3970789c11bd52c1))


### ♻️ 代码重构

* **image:** 重构图像处理功能并支持 PNG 编码 ([63073d2](https://github.com/CandriaJS/image-tool/commit/63073d2cf30d287d7f2470dd7a2a751fc3fa2e98))
* **image:** 重构图像裁剪功能并优化 JPEG 编码 ([6fba15d](https://github.com/CandriaJS/image-tool/commit/6fba15d76f4aa540379b43f470e8bcf8a8b2f0c8))

## [0.1.11](https://github.com/CandriaJS/image-tool/compare/v0.1.10...v0.1.11) (2025-06-07)


### 🎡 持续集成

* **release:** 为 Android 平台添加 aarch64 架构的构建产物 ([cefe0c4](https://github.com/CandriaJS/image-tool/commit/cefe0c4aa95aba99db8fac6041f682189e40467a))

## [0.1.10](https://github.com/CandriaJS/image-tool/compare/v0.1.9...v0.1.10) (2025-06-06)

### ✅ 测试相关

* 测试napi-rs ([20d82da](https://github.com/CandriaJS/image-tool/commit/20d82da9651ec848c8f31c7045478ee64a3c1a16))


### 📦️ 构建系统

* 优化构建流程并添加平台特定构建产物 ([4f94a90](https://github.com/CandriaJS/image-tool/commit/4f94a90f80651f1a23dbdbc43c1ac27f67ee3913))
* 更新版本号并移除 Windows 二进制文件 ([5cba3d0](https://github.com/CandriaJS/image-tool/commit/5cba3d09a8829baa352d73752638f80af415f455))
* 更新版本号并移除可选依赖 ([bb28d61](https://github.com/CandriaJS/image-tool/commit/bb28d617e82cbed9cdab9f69c1219821d5696cf2))


### 🎡 持续集成

* **release:** 优化构建流程并确保上传文件存在性 ([5d8c57f](https://github.com/CandriaJS/image-tool/commit/5d8c57f8bc6140db31cdc1f72d64659202150059))
* **release:** 使用 pnpm 处理构建产物 ([0fc4b19](https://github.com/CandriaJS/image-tool/commit/0fc4b19b337057322504ef7e058a61883a00dc0a))
* **release:** 更新发布配置并处理发布失败情况 ([b4a545e](https://github.com/CandriaJS/image-tool/commit/b4a545e7f74b4ed6d60199e937a5a99ac2b03624))
* **release:** 替换 GitHub Actions 发布到 npm 注册表的方法 ([c71b3d5](https://github.com/CandriaJS/image-tool/commit/c71b3d53f00dcdfbb46ac6e710d46330e5857389))
* 更新缓存键并移除 Cargo 配置 ([7da641a](https://github.com/CandriaJS/image-tool/commit/7da641a2731b837800adb70353096a361b26c438))
