use super::{Obfuscator, Result};
use crate::gerber::types::GerberFileType;
use rand::Rng;
use regex::Regex;

/// 丝印层坐标偏移范围（Gerber 内部单位，约 0.01-0.05mm）
const SILKSCREEN_OFFSET_MIN: i64 = 100;
const SILKSCREEN_OFFSET_MAX: i64 = 500;

pub struct SilkscreenObfuscator;

impl SilkscreenObfuscator {
    pub fn new() -> Self {
        Self
    }

    fn apply_coordinate_jitter(coord: i64) -> i64 {
        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(-SILKSCREEN_OFFSET_MAX..=SILKSCREEN_OFFSET_MAX);
        // 确保偏移量至少有最小值
        let offset = if offset.abs() < SILKSCREEN_OFFSET_MIN {
            if offset >= 0 { SILKSCREEN_OFFSET_MIN } else { -SILKSCREEN_OFFSET_MIN }
        } else {
            offset
        };
        coord + offset
    }
}

impl Obfuscator for SilkscreenObfuscator {
    fn name(&self) -> &'static str {
        "丝印层扰动"
    }

    fn obfuscate(&self, content: &str, file_type: GerberFileType) -> Result<String> {
        // 只处理丝印层
        if !file_type.is_silkscreen() {
            return Ok(content.to_string());
        }

        let mut result = String::new();
        // 匹配坐标指令: X123456Y789012D01* 或 X123456Y789012D02* 等
        let coord_re = Regex::new(r"X(-?\d+)Y(-?\d+)(D0[123]\*|D0[123]$|\*)").unwrap();

        for line in content.lines() {
            if coord_re.is_match(line) {
                let new_line = coord_re.replace_all(line, |caps: &regex::Captures| {
                    let x: i64 = caps[1].parse().unwrap_or(0);
                    let y: i64 = caps[2].parse().unwrap_or(0);
                    let suffix = &caps[3];
                    
                    let new_x = Self::apply_coordinate_jitter(x);
                    let new_y = Self::apply_coordinate_jitter(y);
                    
                    format!("X{}Y{}{}", new_x, new_y, suffix)
                });
                result.push_str(&new_line);
            } else {
                result.push_str(line);
            }
            result.push('\n');
        }

        // 移除最后多余的换行
        if result.ends_with('\n') && !content.ends_with('\n') {
            result.pop();
        }

        Ok(result)
    }
}
