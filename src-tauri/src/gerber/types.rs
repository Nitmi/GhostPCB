use serde::{Deserialize, Serialize};

/// Gerber 文件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GerberFileType {
    TopLayer,           // .GTL
    BottomLayer,        // .GBL
    TopSilkscreen,      // .GTO
    BottomSilkscreen,   // .GBO
    TopSolderMask,      // .GTS
    BottomSolderMask,   // .GBS
    TopPaste,           // .GTP
    BottomPaste,        // .GBP
    BoardOutline,       // .GKO
    InnerLayer,         // .G1-.Gn
    Drill,              // .DRL
    Unknown,
}

impl GerberFileType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_uppercase().as_str() {
            "GTL" => Self::TopLayer,
            "GBL" => Self::BottomLayer,
            "GTO" => Self::TopSilkscreen,
            "GBO" => Self::BottomSilkscreen,
            "GTS" => Self::TopSolderMask,
            "GBS" => Self::BottomSolderMask,
            "GTP" => Self::TopPaste,
            "GBP" => Self::BottomPaste,
            "GKO" => Self::BoardOutline,
            "DRL" => Self::Drill,
            s if s.starts_with('G') && s.len() >= 2 => Self::InnerLayer,
            _ => Self::Unknown,
        }
    }

    pub fn is_silkscreen(&self) -> bool {
        matches!(self, Self::TopSilkscreen | Self::BottomSilkscreen)
    }

    pub fn is_drill(&self) -> bool {
        matches!(self, Self::Drill)
    }

    pub fn is_outline(&self) -> bool {
        matches!(self, Self::BoardOutline)
    }

    pub fn is_copper_layer(&self) -> bool {
        matches!(self, Self::TopLayer | Self::BottomLayer | Self::InnerLayer)
    }
}

/// 混淆配置选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObfuscateOptions {
    pub timestamp: bool,
    pub silkscreen: bool,
    pub geometry: bool,
    pub structure: bool,
    pub physical: bool,
}

impl Default for ObfuscateOptions {
    fn default() -> Self {
        Self {
            timestamp: true,
            silkscreen: true,
            geometry: true,
            structure: true,
            physical: true,
        }
    }
}

/// 处理请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRequest {
    pub input_path: String,
    pub output_dir: Option<String>,
    pub count: u32,
    pub options: ObfuscateOptions,
}

/// 处理结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    pub success: bool,
    pub output_files: Vec<String>,
    pub message: String,
}
