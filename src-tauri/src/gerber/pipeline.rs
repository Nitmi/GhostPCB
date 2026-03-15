use crate::gerber::error::Result;
use crate::gerber::obfuscators::*;
use crate::gerber::types::{GerberFileType, ObfuscateOptions};

/// 混淆处理管道
pub struct ObfuscationPipeline {
    obfuscators: Vec<Box<dyn Obfuscator>>,
}

impl ObfuscationPipeline {
    pub fn from_options(_options: &ObfuscateOptions) -> Self {
        let mut obfuscators: Vec<Box<dyn Obfuscator>> = Vec::new();

        // v2.0 固定策略：只保留丝印扰动，其他策略不再开放给用户选择。
        obfuscators.push(Box::new(SilkscreenObfuscator::new()));

        Self { obfuscators }
    }

    pub fn process(&self, content: &str, file_type: GerberFileType) -> Result<String> {
        let mut result = content.to_string();

        for obfuscator in &self.obfuscators {
            result = obfuscator.obfuscate(&result, file_type)?;
        }

        Ok(result)
    }

    pub fn obfuscator_names(&self) -> Vec<&'static str> {
        self.obfuscators.iter().map(|o| o.name()).collect()
    }
}
