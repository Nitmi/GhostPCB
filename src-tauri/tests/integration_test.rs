//! 集成测试
//! 
//! 测试前请将 Gerber.zip 放到 tests/fixtures/ 目录下

use ghostpcb_lib::*;
use std::path::Path;

/// 测试文件路径
const TEST_GERBER_ZIP: &str = "tests/fixtures/Gerber.zip";

#[test]
fn test_process_gerber_zip() {
    let zip_path = Path::new(TEST_GERBER_ZIP);
    
    if !zip_path.exists() {
        println!("⚠️ 测试文件不存在: {}", TEST_GERBER_ZIP);
        println!("请将 Gerber.zip 放到 src-tauri/tests/fixtures/ 目录下");
        return;
    }

    let output_base = std::env::temp_dir().join("ghostpcb_test");
    
    let request = ProcessRequest {
        input_path: zip_path.to_string_lossy().to_string(),
        output_dir: Some(output_base.to_string_lossy().to_string()),
        count: 2,
        options: ObfuscateOptions::default(),
    };

    let result = ghostpcb_lib::gerber::GerberProcessor::process(&request);
    
    assert!(result.is_ok(), "处理失败: {:?}", result.err());
    
    let result = result.unwrap();
    assert!(result.success);
    assert_eq!(result.output_files.len(), 2);
    
    for file in &result.output_files {
        let path = Path::new(file);
        assert!(path.exists(), "输出文件不存在: {}", file);
        // 验证文件名格式: Gerber_PCB1_YYYY-MM-DD_序号.zip
        let filename = path.file_name().unwrap().to_str().unwrap();
        assert!(filename.starts_with("Gerber_PCB1_"), "文件名应以 Gerber_PCB1_ 开头: {}", filename);
        assert!(filename.ends_with(".zip"), "文件名应以 .zip 结尾: {}", filename);
        // 验证输出目录包含 GhostPCB_ 前缀
        let parent = path.parent().unwrap().file_name().unwrap().to_str().unwrap();
        assert!(parent.starts_with("GhostPCB_"), "输出目录应以 GhostPCB_ 开头: {}", parent);
        println!("✅ 生成: {}", file);
    }

    println!("✅ 测试通过！消息: {}", result.message);
    println!("📁 输出基础目录: {}", output_base.display());

    // 注释掉清理代码以便查看生成的文件
    // let _ = std::fs::remove_dir_all(&output_base);
}

#[test]
fn test_timestamp_obfuscator() {
    use ghostpcb_lib::gerber::obfuscators::{Obfuscator, TimestampObfuscator};
    use ghostpcb_lib::gerber::types::GerberFileType;

    let input = "G04 EasyEDA Pro v3.2.58, 2026-01-05 14:09:15*\nG04 Test*";
    let obfuscator = TimestampObfuscator::new();
    let result = obfuscator.obfuscate(input, GerberFileType::Unknown).unwrap();
    
    assert!(!result.contains("2026-01-05 14:09:15"), "时间戳未被替换");
    assert!(result.contains("G04 EasyEDA Pro"), "其他内容不应改变");
    println!("原始: {}", input);
    println!("混淆后: {}", result);
}

#[test]
fn test_silkscreen_obfuscator() {
    use ghostpcb_lib::gerber::obfuscators::{Obfuscator, SilkscreenObfuscator};
    use ghostpcb_lib::gerber::types::GerberFileType;

    let input = "X1000000Y2000000D01*\nX1500000Y2500000D03*";
    let obfuscator = SilkscreenObfuscator::new();
    
    let result = obfuscator.obfuscate(input, GerberFileType::TopSilkscreen).unwrap();
    assert_ne!(input, result, "丝印层坐标应该被修改");
    
    let result2 = obfuscator.obfuscate(input, GerberFileType::TopLayer).unwrap();
    assert_eq!(input.trim(), result2.trim(), "非丝印层不应被修改");
}

#[test]
fn test_geometry_obfuscator_drill() {
    use ghostpcb_lib::gerber::obfuscators::{GeometryObfuscator, Obfuscator};
    use ghostpcb_lib::gerber::types::GerberFileType;

    let input = ";TYPE=PLATED\nT01\nX25.24994Y8.763\nX23.876Y9.906";
    let obfuscator = GeometryObfuscator::new();
    let result = obfuscator.obfuscate(input, GerberFileType::Drill).unwrap();
    
    assert!(!result.contains("X25.24994Y8.763"), "钻孔坐标应该被修改");
    assert!(result.contains(";TYPE=PLATED"), "头部注释不应改变");
    println!("原始:\n{}", input);
    println!("混淆后:\n{}", result);
}

#[test]
fn test_structure_obfuscator() {
    use ghostpcb_lib::gerber::obfuscators::{Obfuscator, StructureObfuscator};
    use ghostpcb_lib::gerber::types::GerberFileType;

    let input = "G04 Layer: TopLayer*\nD10*\nX100Y200D01*";
    let obfuscator = StructureObfuscator::new();
    let result = obfuscator.obfuscate(input, GerberFileType::TopLayer).unwrap();
    
    assert!(result.contains("Build ID:"), "应该添加随机 Build ID");
    println!("混淆后:\n{}", result);
}
