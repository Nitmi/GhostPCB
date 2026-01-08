use super::{Obfuscator, Result};
use crate::gerber::types::GerberFileType;
use rand::Rng;

pub struct StructureObfuscator;

impl StructureObfuscator {
    pub fn new() -> Self {
        Self
    }

    /// 生成随机 ID
    fn generate_random_id() -> String {
        let mut rng = rand::thread_rng();
        let id: u32 = rng.gen_range(100000..999999);
        format!("{:X}", id)
    }

    /// 在文件头部添加随机注释
    fn add_random_comment(content: &str) -> String {
        let random_id = Self::generate_random_id();
        let comment = format!("G04 Build ID: {}*\n", random_id);
        
        // 在第一个 G04 注释后插入
        if let Some(pos) = content.find("G04") {
            if let Some(end_pos) = content[pos..].find('\n') {
                let insert_pos = pos + end_pos + 1;
                let mut result = content.to_string();
                result.insert_str(insert_pos, &comment);
                return result;
            }
        }
        
        // 如果没找到，在开头插入
        format!("{}{}", comment, content)
    }

    /// 插入冗余 D-code 指令
    fn insert_redundant_dcodes(content: &str) -> String {
        let mut result = String::new();
        let mut rng = rand::thread_rng();
        let mut current_dcode: Option<String> = None;

        for line in content.lines() {
            // 检测当前选择的 D-code
            if line.starts_with('D') && line.ends_with('*') && !line.contains('X') && !line.contains('Y') {
                current_dcode = Some(line.to_string());
            }

            result.push_str(line);
            result.push('\n');

            // 随机在某些行后插入冗余 D-code 选择
            if let Some(ref dcode) = current_dcode {
                if rng.gen_bool(0.05) { // 5% 概率
                    result.push_str(dcode);
                    result.push('\n');
                }
            }
        }

        if result.ends_with('\n') && !content.ends_with('\n') {
            result.pop();
        }

        result
    }
}

impl Obfuscator for StructureObfuscator {
    fn name(&self) -> &'static str {
        "文件结构混淆"
    }

    fn obfuscate(&self, content: &str, file_type: GerberFileType) -> Result<String> {
        // 钻孔文件格式不同，跳过
        if file_type.is_drill() {
            return Ok(content.to_string());
        }

        let mut result = content.to_string();
        
        // 添加随机注释
        result = Self::add_random_comment(&result);
        
        // 插入冗余指令
        result = Self::insert_redundant_dcodes(&result);

        Ok(result)
    }
}
