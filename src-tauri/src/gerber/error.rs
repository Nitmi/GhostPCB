#[derive(Debug, thiserror::Error)]
pub enum GhostPcbError {
    #[error("ZIP 文件读取失败: {0}")]
    ZipReadError(#[from] zip::result::ZipError),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("文件不存在: {0}")]
    FileNotFound(String),

    #[error("无效的 Gerber 文件: {0}")]
    InvalidGerber(String),

    #[error("处理失败: {0}")]
    ProcessError(String),
}

pub type Result<T> = std::result::Result<T, GhostPcbError>;
