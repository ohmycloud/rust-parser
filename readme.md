# Rust Parser Examples

这是一个展示不同 Rust 解析器库（nom、winnow、pest）用法的示例项目。项目包含多个独立的包，每个包都展示了不同的解析技术和实际应用场景。

## 项目结构

项目采用 Cargo workspace 组织，包含以下包：

### 解析器库包
1. **`src/with_nom/iot-log-parser`** - 使用 nom 解析器库解析 IoT 设备日志（电池管理系统数据）
2. **`src/with_winnow/recipe-parser`** - 使用 winnow 解析器库解析 recipe-lang 格式的食谱文件
3. **`src/with_winnow/winnowcurl`** - 使用 winnow 解析器库解析 cURL 命令行命令

### CLI 工具包
1. **`src/with_winnow/recp`** - 食谱文件显示工具（依赖 recipe-parser）

### 根项目二进制程序
根 `Cargo.toml` 还定义了多个二进制程序，展示不同的解析技术：
- `nom_app` - nom 示例应用（交易记录解析）
- `winnow_app` - winnow 示例应用
- `inclusion` - 包含解析示例
- `mqtt` - MQTT 协议解析
- `transaction_app` - 使用 pest 的交易解析
- `csv_app` - 使用 pest 的 CSV 解析
- `ini_app` - 使用 pest 的 INI 文件解析

## 安装和构建

```bash
# 克隆项目
git clone <repository-url>
cd rust-parser

# 构建所有包
cargo build

# 构建特定包
cargo build -p recipe-parser
cargo build -p winnowcurl
cargo build -p iot_log_parser
```

## 使用示例

### 1. 运行测试

```bash
# 运行特定测试（开启 winnow 的调试功能）
cargo test --features winnow/debug test_parse_destinations -- --nocapture
```

### 2. 食谱解析和显示

```bash
# 显示食谱文件（使用 recp CLI 工具）
cargo run -p recp show src/recipes/hummus.recp
cargo run -p recp show src/recipes/buddha-bowl.recp
cargo run -p recp show src/recipes/potatoes-jean-claud.recp
cargo run -p recp show src/recipes/spaghetti-a-la-veganesa.recp

# 也可以显示多个食谱文件
cargo run -p recp show src/recipes/*.recp
```

### 3. cURL 命令解析

```bash
# 解析完整的 cURL 命令
cargo run -p winnowcurl parse "curl 'http://example.com' -X GET -H 'Accept: application/json'"

# 解析并只显示特定部分（方法、头部、数据、标志、URL）
cargo run -p winnowcurl parse "curl 'http://example.com' -X POST -H 'Content-Type: application/json' -d '{\"key\":\"value\"}'" --part method
cargo run -p winnowcurl parse "curl 'http://example.com' -H 'Accept: application/json' -H 'Authorization: Bearer token'" --part header
```

### 4. IoT 日志解析

```bash
# 运行 IoT 日志解析器示例
cargo run -p iot_log_parser --bin iot_parser

# 也可以直接运行二进制程序
cargo run --bin iot_parser
```

### 5. 其他解析示例

```bash
# 运行交易解析应用（使用 pest）
cargo run --bin transaction_app

# 运行 nom 示例应用
cargo run --bin nom_app

# 运行 winnow 示例应用
cargo run --bin winnow_app

# 运行 CSV 解析应用（使用 pest）
cargo run --bin csv_app

# 运行 INI 文件解析应用（使用 pest）
cargo run --bin ini_app

# 运行 MQTT 协议解析
cargo run --bin mqtt

# 运行包含解析示例
cargo run --bin inclusion
```

## 作为库使用

### recipe-parser 库

```rust
use recipe_parser::parse;

let recipe_text = r#"
# Hummus
@chickpeas 400 g
@tahini 60 ml
@olive-oil 30 ml
@lemon-juice 30 ml
@garlic 2 cloves
@salt 1 tsp

Blend all ingredients until smooth.
"#;

match parse(recipe_text) {
    Ok(tokens) => {
        // 处理解析结果
        for token in tokens {
            println!("{:?}", token);
        }
    }
    Err(e) => eprintln!("解析失败: {}", e),
}
```

### winnowcurl 库

```rust
use winnowcurl::curl::parser::curl_cmd_parse;

let curl_command = "curl 'http://example.com/api' -X POST -H 'Content-Type: application/json'";

match curl_cmd_parse(curl_command) {
    Ok(parsed) => {
        for component in parsed {
            println!("{:?}", component);
        }
    }
    Err(e) => eprintln!("解析失败: {}", e),
}
```

### iot-log-parser 库

```rust
use log_parser::parser::parser::parse_log;

let log_line = "2024-05-05 00:00:21.525  [zjkg:0#10.0.1.88:1883]  D:{\"ver\":211,\"mid\":\"pack2\",\"nm\":\"pack2\"}";

match parse_log(log_line) {
    Ok((remaining, message)) => {
        if let Some(msg) = message {
            println!("解析成功: {:?}", msg);
        }
    }
    Err(e) => eprintln!("解析失败: {}", e),
}
```

## 测试

```bash
# 运行所有测试
cargo test

# 运行特定包的测试
cargo test -p recipe-parser
cargo test -p winnowcurl
cargo test -p iot_log_parser

# 运行特定测试用例
cargo test test_parse_ingredients
cargo test test_curl_parser
```

## 食谱文件格式

项目包含一个简单的食谱语言（recipe-lang），示例文件位于 `src/recipes/` 目录：

- 成分：`@成分名称 数量 单位`（例如：`@chickpeas 400 g`）
- 引用其他食谱：`&食谱名称 数量 单位`（例如：`&hummus 200 g`）
- 计时器：`~时间`（例如：`~30 minutes`）
- 材料：`#材料名称`（例如：`#blender`）
- 元数据：`key: value`（例如：`serves: 4`）
- 注释：以 `//` 开头

## 许可证

项目代码基于 MIT 许可证开源。

## 贡献

欢迎提交 Issue 和 Pull Request 来改进这个项目。
