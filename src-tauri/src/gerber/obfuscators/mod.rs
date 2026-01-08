mod timestamp;
mod silkscreen;
mod geometry;
mod structure;
mod physical;

pub use timestamp::TimestampObfuscator;
pub use silkscreen::SilkscreenObfuscator;
pub use geometry::GeometryObfuscator;
pub use structure::StructureObfuscator;
pub use physical::PhysicalObfuscator;

use crate::gerber::types::GerberFileType;
use crate::gerber::error::Result;

/// 混淆器 trait
pub trait Obfuscator: Send + Sync {
    fn name(&self) -> &'static str;
    fn obfuscate(&self, content: &str, file_type: GerberFileType) -> Result<String>;
}
