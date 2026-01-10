use super::{Obfuscator, Result};
use crate::gerber::types::GerberFileType;
use rand::Rng;
use regex::Regex;

/// 外框尺寸偏移（约 ±0.01mm，单位是 Gerber 内部单位）
const OUTLINE_OFFSET: i64 = 100;

/// 最大板子尺寸限制 100mm × 100mm（Gerber 内部单位，假设精度为 4.5 即 10^5）
const MAX_BOARD_SIZE: i64 = 100_00000; // 100mm = 100 * 10^5

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
    
    /// 解析板框文件，获取当前板子的最大坐标
    fn get_board_bounds(content: &str) -> (i64, i64) {
        let coord_re = Regex::new(r"X(-?\d+)Y(-?\d+)").unwrap();
        let mut max_x: i64 = 0;
        let mut max_y: i64 = 0;
        
        for line in content.lines() {
            let trimmed = line.trim();
            // 跳过格式定义行
            if trimmed.starts_with('%') || trimmed.starts_with("G04") {
                continue;
            }
            
            if let Some(caps) = coord_re.captures(trimmed) {
                if let (Ok(x), Ok(y)) = (caps[1].parse::<i64>(), caps[2].parse::<i64>()) {
                    max_x = max_x.max(x.abs());
                    max_y = max_y.max(y.abs());
                }
            }
        }
        
        (max_x, max_y)
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

        // 获取当前板子尺寸
        let (max_x, max_y) = Self::get_board_bounds(content);
        
        // 计算允许的最大偏移量，确保不超过 100mm
        let offset = Self::get_uniform_offset();
        
        // 如果偏移后会超过 100mm，则只使用负偏移或不偏移
        let safe_offset = if offset > 0 {
            let new_max_x = max_x + offset;
            let new_max_y = max_y + offset;
            if new_max_x > MAX_BOARD_SIZE || new_max_y > MAX_BOARD_SIZE {
                // 板子已经接近或超过 100mm，使用负偏移
                -offset.abs()
            } else {
                offset
            }
        } else {
            offset
        };

        let mut result = String::new();
        // 匹配 G01/G02/G03 坐标指令或纯坐标指令
        let coord_re = Regex::new(r"^(G0[123])?X(-?\d+)Y(-?\d+)(D\d+\*?)?$").unwrap();

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
                    let new_x = x + safe_offset;
                    let new_y = y + safe_offset;
                    
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
