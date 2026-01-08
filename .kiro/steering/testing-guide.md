# 测试指南

## 测试策略

### 单元测试

每个混淆器模块应包含单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_replacement() {
        let input = "G04 Created: 2024-01-15*";
        let result = TimestampObfuscator::new().obfuscate(input, GerberFileType::Unknown).unwrap();
        assert_ne!(input, result);
        assert!(result.contains("G04"));
    }

    #[test]
    fn test_coordinate_jitter() {
        let input = "X100000Y200000D03*";
        let result = apply_coordinate_jitter_to_line(input);
        // 验证坐标已改变但格式正确
    }
}
```

### 集成测试

使用 `gerber-sample-unzipped` 目录中的样本文件：

```rust
#[test]
fn test_full_pipeline() {
    let sample_path = "gerber-sample-unzipped/Gerber";
    // 读取样本文件
    // 应用所有混淆策略
    // 验证输出文件格式正确
}
```

## 验证要点

### 格式验证

- 输出文件仍是有效的 Gerber 格式
- 所有必要的头部指令保留
- 文件以 `M02*` 正确结束

### 差异验证

- 输出文件与输入文件的哈希不同
- 多次生成的文件彼此不同

### 安全验证

- 坐标偏移在安全范围内
- 不会产生短路或开路风险

## 手动测试

1. 使用 Gerber 查看器（如 Gerbv、KiCad）打开处理后的文件
2. 目视检查是否有明显异常
3. 对比原始文件和处理后文件的渲染结果

## 样本文件说明

项目包含的样本文件位于 `gerber-sample-unzipped/Gerber/`：

| 文件 | 用途 |
|------|------|
| `Gerber_TopLayer.GTL` | 顶层铜箔测试 |
| `Gerber_BottomLayer.GBL` | 底层铜箔测试 |
| `Gerber_TopSilkscreenLayer.GTO` | 丝印扰动测试 |
| `Gerber_BottomSilkscreenLayer.GBO` | 丝印扰动测试 |
| `Gerber_BoardOutlineLayer.GKO` | 外框微调测试 |
| `Drill_PTH_Through.DRL` | 钻孔偏移测试 |
