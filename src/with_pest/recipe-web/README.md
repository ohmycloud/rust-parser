# Mise · Recipe playground

使用 **Rust + Yew + WebAssembly + Trunk** 编写的菜谱编辑器，无后端、无需 Node.js 构建，
所有解析均在浏览器内完成。界面延续参考图的“源码 → 菜谱”工作流，重新设计了排版、
配色、编辑体验、空状态与语法指南。

## 启动

```sh
cd ~/opt/rust/rust-parser/src/with_pest/recipe-web

# 确保使用已安装 WASM target 的 rustup 工具链，而非系统 Rust。
export PATH="$HOME/.cargo/bin:$PATH"
rustup target add wasm32-unknown-unknown
cargo install trunk --locked # 如果尚未安装

trunk serve --open
```

默认访问 **http://127.0.0.1:8080**。Trunk 监听源码变化并重新构建。
`Trunk.toml` 指定了与 Rust 依赖匹配的 wasm-bindgen 工具版本；首次构建可能需要联网下载。
如果升级 wasm-bindgen 依赖，也需要同步该配置。

### 生产构建

```sh
trunk build --release

# 本地验证构建结果
python3 -m http.server 8080 --bind 127.0.0.1 --directory dist
```

将 `dist/` 发布到任意支持 WASM MIME 类型的静态服务器即可，不需要 Rust 服务端。
生产部署建议使用 HTTPS（剪贴板 API 需要安全上下文；localhost 可直接使用）。

部署在子路径时，例如 `https://example.com/recipes/`：

```sh
trunk build --release --public-url /recipes/
```

## 功能

- **实时预览**：元数据、食材、用具、引用菜谱、步骤、计时提示、标签和 backstory。
- **源码编辑**：语法着色、行号、同步横向/纵向滚动、字符与行数统计。
- **紧凑滚动条**：页面、编辑器与 Language guide 弹窗使用固定 20px 长、4px 粗的滑块，交互区域宽 20px。
  支持拖动、轨道点击、方向键、Home/End 和 PageUp/PageDown；滚轮与触摸仍由浏览器处理。
  无溢出时隐藏滚动条，打印时也隐藏。长度可通过 `src/scrollbar.rs` 的 `THUMB_LENGTH` 调整。
- **食材清单**：可勾选；相同食材且相同数量/单位时去重，不擅自合并不同单位。
- **本地草稿**：输入时写入 localStorage，刷新恢复；清空后的空草稿同样保留。
  存储不可用时编辑仍然正常，并显示状态。食材勾选状态不保存。
- **两个可切换示例**：hummus 与 tomato pasta；切换会替换当前草稿。
- **复制源码**：操作结果通过可访问的状态提示显示；不支持剪贴板时提示手动复制。
- **打印菜谱**：专用打印样式隐藏编辑器和导航。
- **语法指南**：原生 `dialog`，支持焦点约束、点击外部遮罩关闭、关闭按钮与 Escape；内部选中文字或拖动滚动条不会误关闭。
- **响应式设计**：桌面双栏，平板和手机单栏；支持系统的减少动态效果设置。
- 使用系统字体与本地 SVG，无第三方字体、图片、分析脚本或 API 请求。
- 用户文本通过 Yew 文本节点渲染，不作为 HTML 插入。

## 为什么 Rust 前端不调用 recipe-wasm 的 JS API？

两个入口复用同一个解析核心，不需要维护两套语法：

```text
recipe-lang  ← Pest 语法和 Token
  ├── recipe-wasm → JS/TS 的 parse(input): Token[]
  └── recipe-web  → Rust 视图模型 → Yew → 浏览器 WASM
```

`recipe-wasm` 是给 JavaScript/TypeScript 消费者的绑定层。
Rust 编写的前端可以直接链接 `recipe-lang`，避免跨 JS/WASM 边界往返序列化，
同时保留类型安全。它不是另写一个解析器，也不会影响现有 JS API。

## 输入示例

```text
>> name: A little soup
>> description: Simple things, done well.
>> servings: 2
>> time: 15 minutes
>> tags: vegan, weeknight

Warm {olive oil}(1 tbsp) in a &{pot}.
Add {tomatoes}(400 g) and {water}(200 ml).

Simmer for t{10 minutes}. Finish with {salt}.
---
A recipe for a quiet evening.
```

**空行分隔步骤**，同一步骤内的换行视为软换行。元数据用 `>> key: value`，
名称、描述、份数和总耗时分别使用 `name`、`description`、`servings`、`time`；
总耗时只展示作者填写的数据，不猜测或累加所有计时器。

### 当前语法限制

这些限制来自现有 `recipe-lang`，本前端没有暗中修改底层语法：

- 食材、用具、引用名称使用当前 grammar 的 Latin-character 字符集。
  中文和 emoji **说明文字**可保留，但 `{番茄}` 尚不能识别为 Ingredient。
- 这是宽松解析器，不是严格语法校验器。无法识别的内容可能退回普通 Word；
  所以“成功显示”不等于所有标记都被正确识别。
- backstory 的 `---` 要独占一行、无行尾空白；上一行也应避免尾随空白。
  原解析器的 backstory 空白边界与数量校验问题尚未修正。
- 食材数量保留原始字符串，包括分数；不自动换算单位、份量或营养信息。
- 用具、食材和时间高亮是语义提示；没有实现实际倒计时或相关菜谱跳转。
- 不发送菜谱到服务器；当前草稿会以明文保存在本机 localStorage。
  清空会覆盖保存的草稿。

## 代码结构

| 文件 | 职责 |
| --- | --- |
| `src/lib.rs` | 从共享 Token 构建拥有所有权的菜谱视图模型、步骤分段、示例与单元测试 |
| `src/app.rs` | Yew 页面、编辑器、语义预览、浏览器存储和剪贴板交互 |
| `src/main.rs` | WASM 入口；原生运行时给出启动说明 |
| `src/scrollbar.rs` / `src/scrollbars.rs` | 滑块坐标计算与单元测试、可访问的滚动条组件 |
| `styles.css` | 编辑风格主题、响应式布局、对话框和打印样式 |
| `index.html` / `Trunk.toml` | HTML 入口与构建配置 |

## 检查

```sh
cargo test -p recipe-web
cargo check -p recipe-web --target wasm32-unknown-unknown
cargo clippy -p recipe-web --all-targets --target wasm32-unknown-unknown -- -D warnings
cargo fmt -p recipe-web -- --check
trunk build --release
```

模型测试覆盖示例、空内容、注释与引用、食材去重、软换行、Unicode 与 HTML 文本保留。
浏览器验证覆盖实时解析、勾选、示例切换、草稿恢复、空草稿恢复、复制、
原生对话框、HTML 转义、编辑层滚动同步以及 390px 手机无横向溢出。
