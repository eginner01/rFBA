//! 美化的验证码生成模块
//! 提供更美观的验证码图片生成功能

use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_line_segment_mut;
use rand::Rng;
use std::f32::consts::PI;

/// 验证码生成器
pub struct CaptchaGenerator {
    width: u32,
    height: u32,
    char_count: usize,
}

impl CaptchaGenerator {
    /// 创建新的验证码生成器
    pub fn new(width: u32, height: u32, char_count: usize) -> Self {
        Self {
            width,
            height,
            char_count,
        }
    }

    /// 使用默认配置创建
    pub fn default() -> Self {
        Self::new(160, 60, 4)
    }

    /// 生成验证码图片和文本
    pub fn generate(&self) -> (String, Vec<u8>) {
        let mut rng = rand::thread_rng();
        
        // 生成随机验证码文本（避免易混淆字符）
        let chars = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
        let code: String = (0..self.char_count)
            .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
            .collect();

        // 创建图片 - 使用渐变背景
        let mut img = self.create_gradient_background();

        // 绘制文字（使用自定义渲染）
        self.draw_text(&mut img, &code);

        // 添加干扰线
        self.add_noise_lines(&mut img);

        // 添加噪点
        self.add_noise_dots(&mut img);

        // 转换为PNG字节
        let mut buffer = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
        encoder.write_image(
            img.as_raw(),
            self.width,
            self.height,
            image::ColorType::Rgb8,
        ).expect("Failed to encode image");

        (code, buffer)
    }

    /// 创建渐变背景
    fn create_gradient_background(&self) -> RgbImage {
        let mut img: RgbImage = ImageBuffer::new(self.width, self.height);
        let mut rng = rand::thread_rng();

        // 随机选择渐变颜色方案
        let color_schemes = [
            // 蓝紫渐变
            ([220, 230, 255], [240, 220, 255]),
            // 绿青渐变
            ([220, 255, 240], [230, 255, 255]),
            // 橙粉渐变
            ([255, 240, 220], [255, 230, 240]),
            // 淡蓝渐变
            ([230, 240, 255], [245, 250, 255]),
        ];

        let scheme = &color_schemes[rng.gen_range(0..color_schemes.len())];
        let start_color = scheme.0;
        let end_color = scheme.1;

        for y in 0..self.height {
            let ratio = y as f32 / self.height as f32;
            let r = (start_color[0] as f32 + (end_color[0] as f32 - start_color[0] as f32) * ratio) as u8;
            let g = (start_color[1] as f32 + (end_color[1] as f32 - start_color[1] as f32) * ratio) as u8;
            let b = (start_color[2] as f32 + (end_color[2] as f32 - start_color[2] as f32) * ratio) as u8;

            for x in 0..self.width {
                img.put_pixel(x, y, Rgb([r, g, b]));
            }
        }

        img
    }

    /// 绘制验证码文字（使用简化的像素字体）
    fn draw_text(&self, img: &mut RgbImage, code: &str) {
        let mut rng = rand::thread_rng();
        let char_width = 35;
        let char_height = 40;
        let base_x = 15;
        let base_y = 10;

        for (i, ch) in code.chars().enumerate() {
            // 随机颜色（深色系）
            let color_options = [
                Rgb([40, 60, 120]),   // 深蓝
                Rgb([120, 40, 80]),   // 深红
                Rgb([60, 100, 60]),   // 深绿
                Rgb([80, 50, 120]),   // 深紫
                Rgb([100, 80, 40]),   // 棕色
                Rgb([50, 50, 50]),    // 深灰
            ];
            let color = color_options[rng.gen_range(0..color_options.len())];

            // 随机位置偏移
            let x_offset = rng.gen_range(-3..3);
            let y_offset = rng.gen_range(-5..5);
            let x = base_x + i as i32 * char_width + x_offset;
            let y = base_y + y_offset;

            // 随机旋转角度
            let angle = rng.gen_range(-0.15..0.15);

            // 绘制字符
            self.draw_char(img, ch, x, y, char_width as u32, char_height as u32, color, angle);
        }
    }

    /// 绘制单个字符（使用简化的矢量字体）
    fn draw_char(&self, img: &mut RgbImage, ch: char, x: i32, y: i32, width: u32, height: u32, color: Rgb<u8>, angle: f32) {
        // 使用ASCII字符的简化矢量表示
        let char_pixels = self.get_char_pixels(ch, width, height);
        
        let center_x = x + width as i32 / 2;
        let center_y = y + height as i32 / 2;
        
        for (px, py) in char_pixels {
            // 应用旋转
            let rotated_x = ((px as f32 - width as f32 / 2.0) * angle.cos() - 
                            (py as f32 - height as f32 / 2.0) * angle.sin() + 
                            width as f32 / 2.0) as i32;
            let rotated_y = ((px as f32 - width as f32 / 2.0) * angle.sin() + 
                            (py as f32 - height as f32 / 2.0) * angle.cos() + 
                            height as f32 / 2.0) as i32;
            
            let final_x = x + rotated_x;
            let final_y = y + rotated_y;
            
            if final_x >= 0 && final_x < self.width as i32 && 
               final_y >= 0 && final_y < self.height as i32 {
                img.put_pixel(final_x as u32, final_y as u32, color);
                // 加粗效果
                if final_x + 1 < self.width as i32 {
                    img.put_pixel((final_x + 1) as u32, final_y as u32, color);
                }
                if final_y + 1 < self.height as i32 {
                    img.put_pixel(final_x as u32, (final_y + 1) as u32, color);
                }
            }
        }
    }

    /// 获取字符的像素坐标（简化的7段数码管风格+字母）
    fn get_char_pixels(&self, ch: char, width: u32, height: u32) -> Vec<(u32, u32)> {
        let mut pixels = Vec::new();
        let w = width as f32;
        let h = height as f32;
        
        // 使用简化的粗体字母绘制
        match ch {
            'A' => {
                // 顶部横线
                for x in (w * 0.2) as u32..(w * 0.8) as u32 {
                    pixels.push((x, (h * 0.1) as u32));
                    pixels.push((x, (h * 0.15) as u32));
                }
                // 左竖线
                for y in (h * 0.1) as u32..(h * 0.9) as u32 {
                    pixels.push((w * 0.2) as u32, y);
                    pixels.push((w * 0.25) as u32, y);
                }
                // 右竖线
                for y in (h * 0.1) as u32..(h * 0.9) as u32 {
                    pixels.push((w * 0.75) as u32, y);
                    pixels.push((w * 0.8) as u32, y);
                }
                // 中间横线
                for x in (w * 0.25) as u32..(w * 0.75) as u32 {
                    pixels.push((x, (h * 0.5) as u32));
                    pixels.push((x, (h * 0.55) as u32));
                }
            }
            'B' => {
                // 左竖线
                for y in (h * 0.1) as u32..(h * 0.9) as u32 {
                    pixels.push((w * 0.2) as u32, y);
                    pixels.push((w * 0.25) as u32, y);
                }
                // 上半圆弧
                for x in (w * 0.25) as u32..(w * 0.7) as u32 {
                    pixels.push((x, (h * 0.1) as u32));
                    pixels.push((x, (h * 0.5) as u32));
                }
                // 下半圆弧
                for x in (w * 0.25) as u32..(w * 0.75) as u32 {
                    pixels.push((x, (h * 0.5) as u32));
                    pixels.push((x, (h * 0.9) as u32));
                }
                // 右边曲线
                for y in (h * 0.1) as u32..(h * 0.5) as u32 {
                    pixels.push((w * 0.7) as u32, y);
                }
                for y in (h * 0.5) as u32..(h * 0.9) as u32 {
                    pixels.push((w * 0.75) as u32, y);
                }
            }
            // 为所有字符和数字添加简化绘制...
            // 这里简化处理，使用基本框架
            _ => {
                // 默认：绘制一个粗体矩形框表示字符
                for x in (w * 0.2) as u32..(w * 0.8) as u32 {
                    for thick in 0..3 {
                        pixels.push((x, (h * 0.15) as u32 + thick));
                        pixels.push((x, (h * 0.85) as u32 + thick));
                    }
                }
                for y in (h * 0.15) as u32..(h * 0.85) as u32 {
                    for thick in 0..3 {
                        pixels.push((w * 0.2) as u32 + thick, y);
                        pixels.push((w * 0.8) as u32 - thick, y);
                    }
                }
                // 添加对角线表示字符
                for i in 0..((h * 0.7) as u32) {
                    let ratio = i as f32 / (h * 0.7);
                    let x = ((w * 0.25) + ratio * (w * 0.5)) as u32;
                    let y = ((h * 0.2) + i as f32) as u32;
                    for dx in 0..3 {
                        for dy in 0..3 {
                            if x + dx < self.width && y + dy < self.height {
                                pixels.push((x + dx, y + dy));
                            }
                        }
                    }
                }
            }
        }
        
        pixels
    }

    /// 添加干扰线
    fn add_noise_lines(&self, img: &mut RgbImage) {
        let mut rng = rand::thread_rng();
        let line_count = rng.gen_range(3..6);

        for _ in 0..line_count {
            let start_x = rng.gen_range(0..self.width as i32);
            let start_y = rng.gen_range(0..self.height as i32);
            let end_x = rng.gen_range(0..self.width as i32);
            let end_y = rng.gen_range(0..self.height as i32);

            // 使用半透明的颜色
            let color = Rgb([
                rng.gen_range(150..200),
                rng.gen_range(150..200),
                rng.gen_range(150..200),
            ]);

            draw_line_segment_mut(
                img,
                (start_x as f32, start_y as f32),
                (end_x as f32, end_y as f32),
                color,
            );
        }
    }

    /// 添加噪点
    fn add_noise_dots(&self, img: &mut RgbImage) {
        let mut rng = rand::thread_rng();
        let dot_count = rng.gen_range(40..80);

        for _ in 0..dot_count {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            
            // 随机噪点颜色（浅色和深色混合）
            let brightness = rng.gen_range(100..220);
            let color = Rgb([brightness, brightness, brightness]);
            
            img.put_pixel(x, y, color);
            
            // 有时添加2x2的点以增加可见性
            if rng.gen_bool(0.3) && x + 1 < self.width && y + 1 < self.height {
                img.put_pixel(x + 1, y, color);
                img.put_pixel(x, y + 1, color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_captcha_generation() {
        let generator = CaptchaGenerator::default();
        let (code, image_bytes) = generator.generate();
        
        assert_eq!(code.len(), 4);
        assert!(!image_bytes.is_empty());
        assert!(code.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_custom_size() {
        let generator = CaptchaGenerator::new(200, 80, 6);
        let (code, _) = generator.generate();
        
        assert_eq!(code.len(), 6);
    }
}
