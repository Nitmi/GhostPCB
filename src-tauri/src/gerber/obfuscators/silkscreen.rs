use super::{Obfuscator, Result};
use crate::gerber::types::GerberFileType;
use rand::Rng;
use regex::Regex;

/// 丝印层整体平移范围（毫米）
const SILKSCREEN_SHIFT_MIN_MM: f64 = 0.01;
const SILKSCREEN_SHIFT_MAX_MM: f64 = 0.03;

#[derive(Clone, Copy)]
struct CoordinateFormat {
    decimal_digits: u32,
    unit: Unit,
}

#[derive(Clone, Copy)]
enum Unit {
    Millimeter,
    Inch,
}

pub struct SilkscreenObfuscator;

impl SilkscreenObfuscator {
    pub fn new() -> Self {
        Self
    }

    fn detect_format(content: &str) -> CoordinateFormat {
        let format_re = Regex::new(r"%FSLAX(\d)(\d)Y(\d)(\d)\*%").unwrap();
        let decimal_digits = format_re
            .captures(content)
            .and_then(|caps| caps.get(2))
            .and_then(|m| m.as_str().parse::<u32>().ok())
            .unwrap_or(4);
        let unit = if content.contains("%MOIN*%") {
            Unit::Inch
        } else {
            Unit::Millimeter
        };

        CoordinateFormat {
            decimal_digits,
            unit,
        }
    }

    fn mm_to_gerber_units(mm: f64, format: CoordinateFormat) -> i64 {
        let scale = 10_i64.pow(format.decimal_digits) as f64;
        match format.unit {
            Unit::Millimeter => (mm * scale).round() as i64,
            Unit::Inch => ((mm / 25.4) * scale).round() as i64,
        }
    }

    fn select_layer_shift(format: CoordinateFormat) -> (i64, i64) {
        let mut rng = rand::thread_rng();
        let min = Self::mm_to_gerber_units(SILKSCREEN_SHIFT_MIN_MM, format).max(1);
        let max = Self::mm_to_gerber_units(SILKSCREEN_SHIFT_MAX_MM, format).max(min);
        (rng.gen_range(min..=max), rng.gen_range(min..=max))
    }

    fn shift_line(line: &str, shift_x: i64, shift_y: i64) -> String {
        let x_re = Regex::new(r"X(-?\d+)").unwrap();
        let y_re = Regex::new(r"Y(-?\d+)").unwrap();

        let shifted_x = x_re.replace_all(line, |caps: &regex::Captures| {
            let value = caps[1].parse::<i64>().unwrap_or(0);
            format!("X{}", value + shift_x)
        });

        y_re.replace_all(&shifted_x, |caps: &regex::Captures| {
            let value = caps[1].parse::<i64>().unwrap_or(0);
            format!("Y{}", value + shift_y)
        })
        .to_string()
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

        let format = Self::detect_format(content);
        let (shift_x, shift_y) = Self::select_layer_shift(format);
        let mut result = String::new();
        let draw_re = Regex::new(r"D0[123]\*?$").unwrap();
        let coord_re = Regex::new(r"[XY]-?\d+").unwrap();

        for line in content.lines() {
            let trimmed = line.trim();

            // 跳过格式定义行（以 % 开头）、注释行（G04）、其他控制指令
            if trimmed.starts_with('%')
                || trimmed.starts_with("G04")
                || trimmed.starts_with('M')
                || trimmed.is_empty()
                || trimmed.starts_with("G36")
                || trimmed.starts_with("G37")
                || trimmed.starts_with("G75")
            {
                result.push_str(line);
                result.push('\n');
                continue;
            }

            if draw_re.is_match(trimmed) && coord_re.is_match(trimmed) {
                result.push_str(&Self::shift_line(line, shift_x, shift_y));
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
