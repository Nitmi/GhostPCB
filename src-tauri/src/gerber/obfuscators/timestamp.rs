use super::{Obfuscator, Result};
use crate::gerber::types::GerberFileType;
use chrono::{Duration, Local, NaiveDateTime, Timelike};
use rand::Rng;
use regex::Regex;

pub struct TimestampObfuscator {
    new_datetime: NaiveDateTime,
}

impl TimestampObfuscator {
    pub fn new() -> Self {
        Self {
            new_datetime: generate_random_datetime(),
        }
    }

    pub fn with_datetime(datetime: NaiveDateTime) -> Self {
        Self { new_datetime: datetime }
    }
}

fn generate_random_datetime() -> NaiveDateTime {
    let now = Local::now().naive_local();
    let mut rng = rand::thread_rng();
    let days_ago = rng.gen_range(1..30);
    let hours = rng.gen_range(8..18);
    let minutes = rng.gen_range(0..60);
    let seconds = rng.gen_range(0..60);
    
    now - Duration::days(days_ago)
        - Duration::hours(now.time().hour() as i64)
        - Duration::minutes(now.time().minute() as i64)
        - Duration::seconds(now.time().second() as i64)
        + Duration::hours(hours)
        + Duration::minutes(minutes)
        + Duration::seconds(seconds)
}

impl Obfuscator for TimestampObfuscator {
    fn name(&self) -> &'static str {
        "时间戳修改"
    }

    fn obfuscate(&self, content: &str, _file_type: GerberFileType) -> Result<String> {
        let mut result = content.to_string();
        
        // 匹配 YYYY-MM-DD HH:MM:SS 格式
        let re1 = Regex::new(r"\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}").unwrap();
        let new_dt1 = self.new_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        result = re1.replace_all(&result, new_dt1.as_str()).to_string();

        // 匹配 YYYY/MM/DD 格式
        let re2 = Regex::new(r"\d{4}/\d{2}/\d{2}").unwrap();
        let new_dt2 = self.new_datetime.format("%Y/%m/%d").to_string();
        result = re2.replace_all(&result, new_dt2.as_str()).to_string();

        // 匹配 MM/DD/YYYY 格式
        let re3 = Regex::new(r"\d{2}/\d{2}/\d{4}").unwrap();
        let new_dt3 = self.new_datetime.format("%m/%d/%Y").to_string();
        result = re3.replace_all(&result, new_dt3.as_str()).to_string();

        // 匹配纯日期 YYYY-MM-DD
        let re4 = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
        let new_dt4 = self.new_datetime.format("%Y-%m-%d").to_string();
        result = re4.replace_all(&result, new_dt4.as_str()).to_string();

        Ok(result)
    }
}
