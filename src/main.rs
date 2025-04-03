use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use image::{DynamicImage, GenericImageView};
use rayon::prelude::*; // 添加 rayon 的并行支持
use rayon::ThreadPoolBuilder;

fn dfs(grid: &mut Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
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

fn compute_betti_0(grid: &mut Vec<Vec<u8>>) -> u8 {
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

fn save_vector_to_file(data: &Vec<u8>, filename: &str) -> io::Result<()> {
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
    let 开始: usize = 0;
    let 数目: usize = 8110; // 假设这是要处理的图像数量

    // 使用并行迭代处理每个图像，并记录索引
    let mut data: Vec<(usize, u8, u8)> = (开始..数目)
        .into_par_iter()
        .map(|序号| {
            let aa = format!("../webp/{}.webp", 序号);
            let 输入 = 读取webp(&aa);
            
            let mut 形状: Vec<Vec<u8>> = vec![vec![0; 输入.len()]; 输入[0].len()];
            let mut 补形状: Vec<Vec<u8>> = vec![vec![0; 输入.len()]; 输入[0].len()];

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
            
            // 计算洞数
            (序号, compute_betti_0(&mut 形状), compute_betti_0(&mut 补形状)) // 返回索引和结果
        })
        .collect::<Vec<(usize, u8, u8)>>(); // 收集为元组向量

    // 根据索引排序
    data.sort_by_key(|(index, _, _)| *index); // 按照索引排序

    let mut zeros = vec![];
    let mut ones = vec![];
    data
        .into_iter()
        .for_each(|(_, zero, one)| {
            zeros.push(zero);
            ones.push(one);
        });

    // 保存结果到文件
    save_vector_to_file(&ones, "../out/1.txt").unwrap();
    save_vector_to_file(&zeros, "../out/0.txt").unwrap();
}

fn main() {
    // 初始化 rayon 的线程池，设置线程数
    let num_threads = 1; // 设置所需的线程数
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();

    // 使用自定义线程池
    pool.install(|| {
        统计洞数(); // 调用处理函数
    });
}
