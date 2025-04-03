# Glyph Hole Count

## 简介

`Glyph Hole Count` 是一个用于计算图像中洞数的 Rust 程序。该程序可以处理 WebP 格式的图像，并输出每个图像的洞数统计结果。

## 使用说明

### 文件放置

1. **输入文件**: 
   - 请将要处理的 WebP 图像文件放置在 `../webp/` 目录下。文件名应为数字格式，例如 `0.webp`, `1.webp`, `2.webp` 等。

2. **输出文件**:
   - 程序运行后，输出结果将写入到 `../out/` 目录下。具体输出文件为：
     - `0.txt`: 包含每个图像的零维洞数统计。
     - `1.txt`: 包含每个图像的一维洞数统计。

### 获取程序

您可以选择以下两种方式获取程序：

1. **从 Release 页面下载**:
   - 您可以访问 [GitHub Releases](https://github.com/jiyu8iighx/GlyphHoleCount/releases) 页面，下载最新版本的预编译二进制文件。根据您的操作系统选择相应的文件。

2. **从 GitHub Actions 下载**:
   - 在每次提交或发布时，GitHub Actions 会自动构建并上传二进制文件。您可以在 Actions 页面找到最新的构建记录，并下载相应的二进制文件。

### 编译与运行

如果您希望自己编译程序，可以使用以下步骤：

1. **编译程序**:
   - 使用以下命令编译程序：
     ```bash
     cargo build --release
     ```

2. **运行程序**:
   - 编译完成后，运行程序：
     ```bash
     ./target/release/glyph-hole-count
     ```

### 注意事项

- 如果选择自行编译，确保您的系统中已安装 Rust 和 Cargo。
- 输入的图像文件应为有效的 WebP 格式。
- 目前输出文件的内容将被覆盖。
