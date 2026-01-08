use super::{Obfuscator, Result};
use crate::gerber::types::GerberFileType;
use rand::Rng;
use regex::Regex;

/// 钻孔坐标偏移范围（约 ±0.02mm）
const DRILL_OFFSET_MAX: i64 = 200;

pub struct GeometryObfuscator;

impl GeometryObfuscator {
    pub fn new() -> Self {
        Self
    }

    fn apply_drill_jitter(coord_str: &str) -> String {
        // 钻孔文件坐标格式可能是浮点数
        if let Ok(coord) = coord_str.parse::<f64>() {
            let mut rng = rand::thread_rng();
            let offset = rng.gen_range(-0.02..=0.02); // ±0.02mm
            let new_coord = coord + offset;
            // 保持原有精度格式
            if coord_str.contains('.') {
                let decimals = coord_str.split('.').nth(1).map(|s| s.len()).unwrap_or(3);
                format!("{:.prec$}", new_coord, prec = decimals)
            } else {
                format!("{:.5}", new_coord)
            }
        } else {
            coord_str.to_string()
        }
    }

    fn apply_gerber_coord_jitter(coord: i64) -> i64 {
        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(-DRILL_OFFSET_MAX..=DRILL_OFFSET_MAX);
        coord + offset
    }
}

impl Obfuscator for GeometryObfuscator {
    fn name(&self) -> &'static str {
        "几何结构扰动"
    }

    fn obfuscate(&self, content: &str, file_type: GerberFileType) -> Result<String> {
        if file_type.is_drill() {
            return self.obfuscate_drill(content);
        }
        
        // 对铜层应用微小偏移
        if file_type.is_copper_layer() {
            return self.obfuscate_copper(content);
        }

        Ok(content.to_string())
    }
}

impl GeometryObfuscator {
    fn obfuscate_drill(&self, content: &str) -> Result<String> {
        let mut result = String::new();
        // Excellon 钻孔格式: X25.24994Y8.763 或 X25Y8
        let coord_re = Regex::new(r"X(-?[\d.]+)Y(-?[\d.]+)").unwrap();

        for line in content.lines() {
            // 跳过头部定义行
            if line.starts_with(';') || line.starts_with('%') || line.starts_with('M') 
               || line.starts_with('T') || line.starts_with('G') || line.trim().is_empty() {
                result.push_str(line);
                result.push('\n');
                continue;
            }

            if coord_re.is_match(line) {
                let new_line = coord_re.replace_all(line, |caps: &regex::Captures| {
                    let x = Self::apply_drill_jitter(&caps[1]);
                    let y = Self::apply_drill_jitter(&caps[2]);
                    format!("X{}Y{}", x, y)
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

    fn obfuscate_copper(&self, content: &str) -> Result<String> {
        let mut result = String::new();
        let coord_re = Regex::new(r"X(-?\d+)Y(-?\d+)").unwrap();

        for line in content.lines() {
            // 跳过头部定义和特殊指令
            if line.starts_with('G') && !line.contains('X') || 
               line.starts_with('%') || line.starts_with('M') {
                result.push_str(line);
                result.push('\n');
                continue;
            }

            if coord_re.is_match(line) {
                let new_line = coord_re.replace_all(line, |caps: &regex::Captures| {
                    let x: i64 = caps[1].parse().unwrap_or(0);
                    let y: i64 = caps[2].parse().unwrap_or(0);
                    
                    let new_x = Self::apply_gerber_coord_jitter(x);
                    let new_y = Self::apply_gerber_coord_jitter(y);
                    
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
