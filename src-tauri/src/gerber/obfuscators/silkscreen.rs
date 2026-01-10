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
        // 匹配 G01/G02/G03 坐标指令: G01X4461125Y2961782D01*
        let coord_re = Regex::new(r"^(G0[123])?X(-?\d+)Y(-?\d+)(D0[123]\*?)$").unwrap();

        for line in content.lines() {
            let trimmed = line.trim();
            
            // 跳过格式定义行（以 % 开头）、注释行（G04）、其他控制指令
            if trimmed.starts_with('%') || trimmed.starts_with("G04") || 
               trimmed.starts_with('M') || trimmed.is_empty() ||
               trimmed.starts_with("G36") || trimmed.starts_with("G37") ||
               trimmed.starts_with("G75") {
                result.push_str(line);
                result.push('\n');
                continue;
            }
            
            if coord_re.is_match(trimmed) {
                let new_line = coord_re.replace(trimmed, |caps: &regex::Captures| {
                    let prefix = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                    let x: i64 = caps[2].parse().unwrap_or(0);
                    let y: i64 = caps[3].parse().unwrap_or(0);
                    let suffix = &caps[4];
                    
                    let new_x = Self::apply_coordinate_jitter(x);
                    let new_y = Self::apply_coordinate_jitter(y);
                    
                    format!("{}X{}Y{}{}", prefix, new_x, new_y, suffix)
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
