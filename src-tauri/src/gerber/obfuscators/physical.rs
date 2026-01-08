use super::{Obfuscator, Result};
use crate::gerber::types::GerberFileType;
use rand::Rng;
use regex::Regex;

/// 外框尺寸偏移（约 ±0.01mm）
const OUTLINE_OFFSET: i64 = 100;

pub struct PhysicalObfuscator;

impl PhysicalObfuscator {
    pub fn new() -> Self {
        Self
    }

    fn get_uniform_offset() -> i64 {
        let mut rng = rand::thread_rng();
        let sign = if rng.gen_bool(0.5) { 1 } else { -1 };
        sign * rng.gen_range(50..=OUTLINE_OFFSET)
    }
}

impl Obfuscator for PhysicalObfuscator {
    fn name(&self) -> &'static str {
        "物理参数微调"
    }

    fn obfuscate(&self, content: &str, file_type: GerberFileType) -> Result<String> {
        // 只处理板框文件
        if !file_type.is_outline() {
            return Ok(content.to_string());
        }

        let mut result = String::new();
        let coord_re = Regex::new(r"X(-?\d+)Y(-?\d+)").unwrap();
        
        // 对整个板框应用统一偏移，保持形状
        let offset = Self::get_uniform_offset();

        for line in content.lines() {
            // 跳过头部定义
            if line.starts_with('%') || line.starts_with('M') || 
               (line.starts_with('G') && !line.contains('X')) {
                result.push_str(line);
                result.push('\n');
                continue;
            }

            if coord_re.is_match(line) {
                let new_line = coord_re.replace_all(line, |caps: &regex::Captures| {
                    let x: i64 = caps[1].parse().unwrap_or(0);
                    let y: i64 = caps[2].parse().unwrap_or(0);
                    
                    // 对所有坐标应用统一偏移
                    let new_x = x + offset;
                    let new_y = y + offset;
                    
                    format!("X{}Y{}", new_x, new_y)
                });
                result.push_str(&new_line);
            } else {
                result.push_str(line);
            }
            result.push('\n');
        }

        if result.ends_with('\n') && !content.ends_with('\n') {
            result.pop();
        }

        Ok(result)
    }
}
