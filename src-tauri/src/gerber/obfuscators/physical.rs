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
        // 匹配 G01/G02/G03 坐标指令或纯坐标指令
        let coord_re = Regex::new(r"^(G0[123])?X(-?\d+)Y(-?\d+)(D\d+\*?)?$").unwrap();
        
        // 对整个板框应用统一偏移，保持形状
        let offset = Self::get_uniform_offset();

        for line in content.lines() {
            let trimmed = line.trim();
            
            // 跳过格式定义行（以 % 开头）、注释行（G04）、控制指令
            if trimmed.starts_with('%') || trimmed.starts_with("G04") || 
               trimmed.starts_with('M') || trimmed.is_empty() ||
               trimmed.starts_with("G36") || trimmed.starts_with("G37") ||
               trimmed.starts_with("G75") {
                result.push_str(line);
                result.push('\n');
                continue;
            }
            
            // 跳过 D 码选择指令 (如 D10*)
            if trimmed.starts_with('D') && !trimmed.contains('X') {
                result.push_str(line);
                result.push('\n');
                continue;
            }

            if coord_re.is_match(trimmed) {
                let new_line = coord_re.replace(trimmed, |caps: &regex::Captures| {
                    let prefix = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                    let x: i64 = caps[2].parse().unwrap_or(0);
                    let y: i64 = caps[3].parse().unwrap_or(0);
                    let suffix = caps.get(4).map(|m| m.as_str()).unwrap_or("");
                    
                    // 对所有坐标应用统一偏移
                    let new_x = x + offset;
                    let new_y = y + offset;
                    
                    format!("{}X{}Y{}{}", prefix, new_x, new_y, suffix)
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
