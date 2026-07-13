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

        // 全层逐坐标独立抖动 (先抖动，后丝印整体偏移)
        obfuscators.push(Box::new(CoordinateJitterObfuscator::new()));
        // 丝印层整体平移
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
