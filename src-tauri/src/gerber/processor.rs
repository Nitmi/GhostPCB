use crate::gerber::error::{GhostPcbError, Result};
use crate::gerber::pipeline::ObfuscationPipeline;
use crate::gerber::types::{GerberFileType, ObfuscateOptions, ProcessRequest, ProcessResult};
use chrono::Local;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

pub struct GerberProcessor;

impl GerberProcessor {
    /// 处理 Gerber ZIP 文件
    pub fn process(request: &ProcessRequest) -> Result<ProcessResult> {
        let input_path = Path::new(&request.input_path);
        
        if !input_path.exists() {
            return Err(GhostPcbError::FileNotFound(request.input_path.clone()));
        }

        // 确定输出目录
        let output_dir = Self::get_output_dir(input_path, request.output_dir.as_deref())?;
        fs::create_dir_all(&output_dir)?;

        let mut output_files = Vec::new();
        let original_name = input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("gerber");

        // 生成指定数量的混淆文件
        for i in 1..=request.count {
            let output_filename = format!("{}_{}.zip", original_name, i);
            let output_path = output_dir.join(&output_filename);
            
            Self::process_single(input_path, &output_path, &request.options)?;
            output_files.push(output_path.to_string_lossy().to_string());
        }

        Ok(ProcessResult {
            success: true,
            output_files,
            message: format!("成功生成 {} 个混淆文件", request.count),
        })
    }

    /// 获取输出目录
    fn get_output_dir(input_path: &Path, custom_dir: Option<&str>) -> Result<PathBuf> {
        if let Some(dir) = custom_dir {
            return Ok(PathBuf::from(dir));
        }

        // 默认: 原文件同级目录/GhostPCB_日期_原文件名/
        let parent = input_path.parent().unwrap_or(Path::new("."));
        let original_name = input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("gerber");
        let date = Local::now().format("%Y%m%d").to_string();
        let dir_name = format!("GhostPCB_{}_{}", date, original_name);
        
        Ok(parent.join(dir_name))
    }

    /// 处理单个文件
    fn process_single(input_path: &Path, output_path: &Path, options: &ObfuscateOptions) -> Result<()> {
        let temp_dir = TempDir::new()?;
        
        // 解压 ZIP
        Self::extract_zip(input_path, temp_dir.path())?;
        
        // 创建处理管道
        let pipeline = ObfuscationPipeline::from_options(options);
        
        // 处理所有文件
        Self::process_directory(temp_dir.path(), &pipeline)?;
        
        // 重新打包
        Self::create_zip(temp_dir.path(), output_path)?;
        
        Ok(())
    }

    /// 解压 ZIP 文件
    fn extract_zip(zip_path: &Path, dest_dir: &Path) -> Result<()> {
        let file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = dest_dir.join(file.name());

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    fs::create_dir_all(p)?;
                }
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        Ok(())
    }

    /// 处理目录中的所有 Gerber 文件
    fn process_directory(dir: &Path, pipeline: &ObfuscationPipeline) -> Result<()> {
        for entry in walkdir::WalkDir::new(dir) {
            let entry = entry.map_err(|e| GhostPcbError::IoError(e.into()))?;
            let path = entry.path();
            
            if path.is_file() {
                let ext = path.extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                
                let file_type = GerberFileType::from_extension(ext);
                
                // 只处理已知的 Gerber 文件类型
                if !matches!(file_type, GerberFileType::Unknown) {
                    Self::process_file(path, file_type, pipeline)?;
                }
            }
        }
        
        Ok(())
    }

    /// 处理单个文件
    fn process_file(path: &Path, file_type: GerberFileType, pipeline: &ObfuscationPipeline) -> Result<()> {
        let mut content = String::new();
        File::open(path)?.read_to_string(&mut content)?;
        
        let processed = pipeline.process(&content, file_type)?;
        
        let mut file = File::create(path)?;
        file.write_all(processed.as_bytes())?;
        
        Ok(())
    }

    /// 创建 ZIP 文件
    fn create_zip(source_dir: &Path, zip_path: &Path) -> Result<()> {
        let file = File::create(zip_path)?;
        let mut zip = ZipWriter::new(file);
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        for entry in walkdir::WalkDir::new(source_dir) {
            let entry = entry.map_err(|e| GhostPcbError::IoError(e.into()))?;
            let path = entry.path();
            let name = path.strip_prefix(source_dir)
                .map_err(|_| GhostPcbError::ProcessError("路径处理错误".to_string()))?;

            if path.is_file() {
                zip.start_file(name.to_string_lossy(), options)?;
                let mut f = File::open(path)?;
                std::io::copy(&mut f, &mut zip)?;
            } else if !name.as_os_str().is_empty() {
                zip.add_directory(name.to_string_lossy(), options)?;
            }
        }

        zip.finish()?;
        Ok(())
    }
}
