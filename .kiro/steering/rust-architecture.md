# Rust 后端架构指南

## 模块结构

```
src-tauri/src/
├── lib.rs                 # Tauri 入口，注册命令
├── main.rs                # 程序入口
├── commands/              # Tauri 命令层
│   ├── mod.rs
│   └── process.rs         # 处理相关命令
└── gerber/                # Gerber 核心处理模块
    ├── mod.rs
    ├── parser.rs          # Gerber 文件解析
    ├── writer.rs          # Gerber 文件写入
    ├── types.rs           # 数据类型定义
    └── obfuscators/       # 混淆策略实现
        ├── mod.rs
        ├── timestamp.rs   # 时间戳修改
        ├── silkscreen.rs  # 丝印层扰动
        ├── geometry.rs    # 几何结构扰动
        ├── structure.rs   # 文件结构混淆
        └── physical.rs    # 物理参数微调
```

## 推荐依赖

```toml
[dependencies]
# 已有
tauri = { version = "2", features = [] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# 需要添加
zip = "2"                  # ZIP 文件处理
regex = "1"                # 正则表达式
rand = "0.8"               # 随机数生成
chrono = "0.4"             # 时间处理
thiserror = "2"            # 错误处理
walkdir = "2"              # 目录遍历
```

## 核心数据结构

```rust
/// 混淆配置选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObfuscateOptions {
    pub timestamp: bool,        // 时间戳修改
    pub silkscreen: bool,       // 丝印层扰动
    pub geometry: bool,         // 几何结构扰动
    pub structure: bool,        // 文件结构混淆
    pub physical: bool,         // 物理参数微调
}

/// 处理请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRequest {
    pub input_path: String,           // 输入 ZIP 路径
    pub output_dir: Option<String>,   // 输出目录（可选）
    pub count: u32,                    // 生成数量
    pub options: ObfuscateOptions,    // 混淆选项
}

/// 处理结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    pub success: bool,
    pub output_files: Vec<String>,
    pub message: String,
}
```

## 设计模式

### 策略模式 - 混淆器

```rust
pub trait Obfuscator: Send + Sync {
    fn name(&self) -> &'static str;
    fn obfuscate(&self, content: &str, file_type: GerberFileType) -> Result<String>;
}
```

每种混淆策略实现此 trait，便于扩展和组合。

### 处理流程

1. 解压 ZIP 到临时目录
2. 识别 Gerber 文件类型
3. 按配置应用混淆策略
4. 重新打包为 ZIP
5. 清理临时文件

## Tauri 命令示例

```rust
#[tauri::command]
async fn process_gerber(request: ProcessRequest) -> Result<ProcessResult, String> {
    // 实现处理逻辑
}

#[tauri::command]
async fn select_file() -> Result<Option<String>, String> {
    // 文件选择对话框
}
```

## 错误处理

使用 `thiserror` 定义统一错误类型：

```rust
#[derive(Debug, thiserror::Error)]
pub enum GhostPcbError {
    #[error("ZIP 文件读取失败: {0}")]
    ZipReadError(#[from] zip::result::ZipError),
    
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Gerber 解析错误: {0}")]
    ParseError(String),
}
```
