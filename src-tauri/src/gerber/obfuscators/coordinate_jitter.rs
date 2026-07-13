use super::{Obfuscator, Result};
use crate::gerber::types::GerberFileType;
use rand::Rng;
use regex::Regex;

fn jitter_max_mm(file_type: GerberFileType) -> Option<f64> {
    match file_type {
        GerberFileType::BoardOutline => Some(0.005),
        GerberFileType::TopLayer
        | GerberFileType::BottomLayer
        | GerberFileType::InnerLayer
        | GerberFileType::TopSilkscreen
        | GerberFileType::BottomSilkscreen
        | GerberFileType::TopSolderMask
        | GerberFileType::BottomSolderMask
        | GerberFileType::TopPaste
        | GerberFileType::BottomPaste => Some(0.003),
        GerberFileType::Drill | GerberFileType::Unknown => None,
    }
}

#[derive(Clone, Copy)]
struct FormatInfo {
    x_decimals: u32,
    y_decimals: u32,
    is_inch: bool,
}

pub struct CoordinateJitterObfuscator;

impl CoordinateJitterObfuscator {
    pub fn new() -> Self {
        Self
    }

    fn detect_format(content: &str) -> Option<FormatInfo> {
        let re = Regex::new(r"%FSLAX(\d)(\d)Y(\d)(\d)\*%").ok()?;
        let caps = re.captures(content)?;
        Some(FormatInfo {
            x_decimals: caps[2].parse().unwrap_or(4),
            y_decimals: caps[4].parse().unwrap_or(4),
            is_inch: content.contains("%MOIN*%"),
        })
    }

    fn mm_to_raw(mm: f64, decimals: u32, is_inch: bool) -> i64 {
        let scale = 10_i64.pow(decimals) as f64;
        let value = if is_inch { mm / 25.4 } else { mm };
        (value * scale).round() as i64
    }

    fn should_skip_line(trimmed: &str) -> bool {
        trimmed.is_empty()
            || trimmed.starts_with('%')
            || trimmed.starts_with("G04")
            || trimmed.starts_with('M')
            || trimmed.starts_with("G36")
            || trimmed.starts_with("G37")
            || trimmed.starts_with("G75")
    }

    fn jitter_value(value: &str, max_raw: i64, rng: &mut impl Rng) -> String {
        let raw = value.parse::<i64>().unwrap_or(0);
        let delta = rng.gen_range(-max_raw..=max_raw);
        if delta == 0 {
            return value.to_string();
        }
        let width = value.trim_start_matches(|c: char| c == '+' || c == '-').len();
        let next = raw + delta;
        let sign = if next < 0 { "-" } else { "" };
        let digits = next.unsigned_abs().to_string();
        let padded = if digits.len() < width {
            format!("{:0>width$}", digits, width = width)
        } else {
            digits
        };
        format!("{}{}", sign, padded)
    }
}

impl Obfuscator for CoordinateJitterObfuscator {
    fn name(&self) -> &'static str {
        "全层坐标抖动"
    }

    fn obfuscate(&self, content: &str, file_type: GerberFileType) -> Result<String> {
        let max_mm = match jitter_max_mm(file_type) {
            Some(v) => v,
            None => return Ok(content.to_string()),
        };

        let fmt = match Self::detect_format(content) {
            Some(f) => f,
            None => return Ok(content.to_string()),
        };

        let max_x_raw = Self::mm_to_raw(max_mm, fmt.x_decimals, fmt.is_inch);
        let max_y_raw = Self::mm_to_raw(max_mm, fmt.y_decimals, fmt.is_inch);
        let mut rng = rand::thread_rng();
        let mut result = String::new();

        let x_re = Regex::new(r"X([+-]?\d+)").unwrap();
        let y_re = Regex::new(r"Y([+-]?\d+)").unwrap();

        for line in content.lines() {
            let trimmed = line.trim();
            if Self::should_skip_line(trimmed) || !trimmed.contains('X') {
                result.push_str(line);
                result.push('\n');
                continue;
            }

            let with_x = x_re.replace_all(line, |caps: &regex::Captures| {
                format!("X{}", Self::jitter_value(&caps[1], max_x_raw, &mut rng))
            });

            let jittered = y_re.replace_all(&with_x, |caps: &regex::Captures| {
                format!("Y{}", Self::jitter_value(&caps[1], max_y_raw, &mut rng))
            });

            result.push_str(&jittered);
            result.push('\n');
        }

        if result.ends_with('\n') && !content.ends_with('\n') {
            result.pop();
        }

        Ok(result)
    }
}
