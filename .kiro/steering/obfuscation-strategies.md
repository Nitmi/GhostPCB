# 混淆策略实现指南

## 策略概览

| 策略 | 风险等级 | 效果 | 实现复杂度 |
|------|----------|------|------------|
| 时间戳修改 | 🟢 无风险 | 中等 | 简单 |
| 丝印层扰动 | 🟢 无风险 | 高 | 中等 |
| 几何结构扰动 | 🟡 低风险 | 高 | 中等 |
| 文件结构混淆 | 🟢 无风险 | 中等 | 复杂 |
| 物理参数微调 | 🟡 低风险 | 中等 | 简单 |

---

## 1. 时间戳修改 (timestamp.rs)

### 目标

替换 Gerber 文件中的时间信息，生成随机但合理的时间戳。

### 实现要点

```rust
use regex::Regex;
use chrono::{NaiveDateTime, Duration};
use rand::Rng;

/// 生成随机时间（过去 30 天内）
fn generate_random_datetime() -> NaiveDateTime {
    let now = chrono::Local::now().naive_local();
    let days_ago = rand::thread_rng().gen_range(1..30);
    let hours = rand::thread_rng().gen_range(8..18);
    let minutes = rand::thread_rng().gen_range(0..60);
    now - Duration::days(days_ago) + Duration::hours(hours) + Duration::minutes(minutes)
}

/// 常见时间格式正则
const DATE_PATTERNS: &[&str] = &[
    r"\d{4}-\d{2}-\d{2}",           // 2024-01-15
    r"\d{4}/\d{2}/\d{2}",           // 2024/01/15
    r"\d{2}/\d{2}/\d{4}",           // 01/15/2024
    r"\d{2}-\w{3}-\d{4}",           // 15-Jan-2024
];
```

### 匹配位置

- `G04` 注释行中的日期
- 文件头部元数据

---

## 2. 丝印层扰动 (silkscreen.rs)

### 目标

微调丝印层坐标，不影响电气特性。

### 实现要点

```rust
/// 坐标偏移范围（单位：Gerber 内部单位）
const SILKSCREEN_OFFSET_RANGE: i64 = 500; // 约 0.05mm

/// 对坐标应用随机偏移
fn apply_coordinate_jitter(coord: i64) -> i64 {
    let offset = rand::thread_rng().gen_range(-SILKSCREEN_OFFSET_RANGE..=SILKSCREEN_OFFSET_RANGE);
    coord + offset
}

/// 匹配坐标指令
/// X123456Y789012D03*
fn parse_coordinate_line(line: &str) -> Option<(i64, i64, &str)> {
    // 解析 X, Y 坐标和后续指令
}
```

### 目标文件

- `.GTO` (TopSilkscreen)
- `.GBO` (BottomSilkscreen)

### 安全范围

- 位移: ±0.01mm ~ ±0.05mm
- 字体缩放: ±1%

---

## 3. 几何结构扰动 (geometry.rs)

### 目标

微调钻孔和过孔坐标，在公差范围内改变特征向量。

### 实现要点

```rust
/// 钻孔坐标偏移范围（更保守）
const DRILL_OFFSET_RANGE: i64 = 200; // 约 0.02mm

/// 过孔直径微调范围
const VIA_DIAMETER_VARIATION: f64 = 0.01; // mm

/// 处理 Excellon 钻孔文件
fn process_drill_file(content: &str) -> String {
    // 解析钻孔坐标并应用偏移
}
```

### 目标文件

- `.DRL` (钻孔文件)
- 所有铜层的过孔坐标

### 注意事项

- 偏移量必须在 PCB 制造公差内
- 避免导致短路或开路

---

## 4. 文件结构混淆 (structure.rs)

### 目标

改变 Gerber 文件的二进制表示，但保持逻辑等价。

### 实现要点

```rust
/// 插入冗余指令
fn insert_redundant_commands(content: &str) -> String {
    // 在安全位置插入无意义的 D-code 选择
    // 例如：D10*D10* (重复选择同一光圈)
}

/// 打乱非依赖指令顺序
fn shuffle_independent_commands(commands: Vec<Command>) -> Vec<Command> {
    // 识别可交换的指令块并随机排序
}

/// 添加随机注释
fn add_random_comments(content: &str) -> String {
    let comment = format!("G04 Build: {}*", generate_random_id());
    // 在文件头部添加
}
```

### 混淆方法

1. 插入冗余 D-code 选择指令
2. 打乱独立指令块顺序
3. 添加随机注释
4. 改变数值表示（前导零处理）

---

## 5. 物理参数微调 (physical.rs)

### 目标

微调板框尺寸和倒角，改变物理特征记录。

### 实现要点

```rust
/// 外框尺寸偏移
const OUTLINE_OFFSET: i64 = 100; // 约 0.01mm

/// 处理板框文件
fn process_outline(content: &str) -> String {
    // 解析外框坐标
    // 对所有坐标应用统一偏移（保持形状）
}

/// 微调圆角半径
fn adjust_corner_radius(radius: f64) -> f64 {
    let variation = rand::thread_rng().gen_range(-0.05..=0.05);
    radius + variation
}
```

### 目标文件

- `.GKO` (BoardOutline)

### 安全范围

- 外框: ±0.01mm
- 倒角: ±0.05mm

---

## 混淆器组合

```rust
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
        // ... 其他策略
        
        Self { obfuscators }
    }
    
    pub fn process(&self, content: &str, file_type: GerberFileType) -> Result<String> {
        let mut result = content.to_string();
        for obfuscator in &self.obfuscators {
            result = obfuscator.obfuscate(&result, file_type)?;
        }
        Ok(result)
    }
}
```

## 随机种子管理

每次生成不同的 Gerber 时，使用不同的随机种子：

```rust
use rand::SeedableRng;
use rand::rngs::StdRng;

fn create_rng_for_iteration(base_seed: u64, iteration: u32) -> StdRng {
    StdRng::seed_from_u64(base_seed.wrapping_add(iteration as u64))
}
```
