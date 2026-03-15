mod geometry;
mod physical;
mod silkscreen;
mod structure;
mod timestamp;

pub use geometry::GeometryObfuscator;
pub use physical::PhysicalObfuscator;
pub use silkscreen::SilkscreenObfuscator;
pub use structure::StructureObfuscator;
pub use timestamp::TimestampObfuscator;

use crate::gerber::error::Result;
use crate::gerber::types::GerberFileType;

/// 混淆器 trait
pub trait Obfuscator: Send + Sync {
    fn name(&self) -> &'static str;
    fn obfuscate(&self, content: &str, file_type: GerberFileType) -> Result<String>;
}
