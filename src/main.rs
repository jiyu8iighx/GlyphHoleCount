use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use image::{DynamicImage, GenericImageView};

fn dfs(grid: &mut Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    // 检查越界或已访问或非1区域
    if i >= rows || j >= cols || grid[i][j] != 1 || visited[i][j] {
        return;
    }
    visited[i][j] = true; // 标记为已访问
    // 递归访问上下左右四个方向（4邻域）
    dfs(grid, visited, i + 1, j);
    dfs(grid, visited, i.wrapping_sub(1), j);
    dfs(grid, visited, i, j + 1);
    dfs(grid, visited, i, j.wrapping_sub(1));
}

fn compute_betti_0(grid: &mut Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut count = 0; // 统计连通分量数

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 && !visited[i][j] {
                dfs(grid, &mut visited, i, j); // 标记整个连通区域
                count += 1;                     // 发现新连通分量
            }
        }
    }
    count - 1
}

fn save_vector_to_file(data: &Vec<i32>, filename: &str) -> io::Result<()> {
    let mut out_file = File::create(filename)?;
    for &num in data {
        writeln!(out_file, "{}", num)?;
    }
    println!("数据已保存到: {}", filename);
    Ok(())
}

// 读取webp图像并返回像素数据
fn 读取webp(filename: &str) -> Vec<Vec<u8>> {
    let img = image::open(filename).expect("无法打开图像文件").into_luma8(); // 转换为灰度图像
    let (width, height) = img.dimensions();
    let mut pixels = vec![vec![0; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            pixels[y as usize][x as usize] = pixel[0]; // 只取灰度值
        }
    }
    pixels
}

fn 写webp(pixels: &Vec<Vec<u8>>, filename: &str) -> image::ImageResult<()> {
    let height = pixels.len() as u32;
    let width = pixels[0].len() as u32;
    let mut img = image::ImageBuffer::new(width, height);

    for (y, row) in pixels.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            img.put_pixel(x as u32, y as u32, image::Luma([value])); // 使用灰度值
        }
    }

    img.save_with_format(filename, image::ImageFormat::WebP)?;
    Ok(())
}

fn 统计洞数() {
    let 开始 = 0;
    // let 数目 = 1161;
    // let 数目 = 8109;
    let 数目 = 8110;
    let mut 一维洞 = vec![0; 数目];
    let mut 零维洞 = vec![0; 数目];

    for 序号 in 开始..数目 {
        let aa = format!("../webp/{}.webp", 序号);
        let 输入 = 读取webp(&aa);
        // debug(序号); // 假设debug函数已实现
        let mut 形状 = vec![vec![0; 输入.len()]; 输入[0].len()];
        let mut 补形状 = vec![vec![0; 输入.len()]; 输入[0].len()];

        for x in 0..输入.len() {
            for y in 0..输入[0].len() {
                if 输入[x][y] < 186 {
                    形状[x][y] = 1;
                    补形状[x][y] = 0;
                } else {
                    形状[x][y] = 0;
                    补形状[x][y] = 1;
                }
            }
        }
        一维洞[序号] = compute_betti_0(&mut 补形状);
        零维洞[序号] = compute_betti_0(&mut 形状);
        写webp(&输入, &format!("../modified/{}.png", 序号)).unwrap();
    }
    save_vector_to_file(&一维洞, "../out/1.txt").unwrap();
    save_vector_to_file(&零维洞, "../out/0.txt").unwrap();
}

fn main() {
    统计洞数();
}
