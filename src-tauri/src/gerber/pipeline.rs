use crate::gerber::error::Result;
use crate::gerber::obfuscators::*;
use crate::gerber::types::{GerberFileType, ObfuscateOptions};

/// 混淆处理管道
pub struct ObfuscationPipeline {
    obfuscators: Vec<Box<dyn Obfuscator>>,
}

impl ObfuscationPipeline {
    pub fn from_options(options: &ObfuscateOptions) -> Self {
        let mut obfuscators: Vec<Box<dyn Obfuscator>> = Vec::new();

        if options.timestamp {
            obfuscators.push(Box::new(TimestampObfuscator::new()));
        }
        if options.silkscreen {
            obfuscators.push(Box::new(SilkscreenObfuscator::new()));
        }
        if options.geometry {
            obfuscators.push(Box::new(GeometryObfuscator::new()));
        }
        if options.structure {
            obfuscators.push(Box::new(StructureObfuscator::new()));
        }
        if options.physical {
            obfuscators.push(Box::new(PhysicalObfuscator::new()));
        }

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
