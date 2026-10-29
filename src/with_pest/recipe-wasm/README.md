# recipe-wasm

`recipe-lang` 的 WebAssembly 绑定，支持浏览器和 Node.js。

## 构建

以下命令均在 `src/with_pest/recipe-wasm` 目录执行，需要 Rust、Node.js
以及与 Rust 依赖版本一致的 `wasm-bindgen-cli`：

```sh
# 使用 rustup 管理的工具链，避免 PATH 优先选中未安装 WASM target 的系统 Rust。
export PATH="$HOME/.cargo/bin:$PATH"
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.129 --locked

cargo build -p recipe-wasm --target wasm32-unknown-unknown --release

# 浏览器 ES module
wasm-bindgen ../../../target/wasm32-unknown-unknown/release/recipe_wasm.wasm \
  --target web --out-dir pkg

# Node.js CommonJS
wasm-bindgen ../../../target/wasm32-unknown-unknown/release/recipe_wasm.wasm \
  --target nodejs --out-dir pkg-node
```

输出包含 JavaScript、`.wasm` 和自动生成的 TypeScript 声明；
`pkg/`、`pkg-node/` 不纳入版本控制。
升级 `wasm-bindgen` 依赖后，需要同步 CLI 版本。

## 浏览器

通过 HTTP 静态服务器提供文件（不要使用 `file://`），并初始化 WASM：

```html
<script type="module">
  import init, { parse } from "./pkg/recipe_wasm.js";

  await init();
  console.log(parse("Boil {water}(100 ml) in &{pot} for t{5 minutes}."));
</script>
```

## Node.js

```js
const { parse } = require("./pkg-node/recipe_wasm.js");

console.log(parse("{water}(100 ml)"));
// [{
//   token: "Ingredient",
//   content: { name: "water", quantity: "100", unit: "ml" }
// }]
```

## API 与 TypeScript

```ts
import { parse, type Token } from "./pkg/recipe_wasm.js";

// 浏览器中需先 await init()。
const tokens: Token[] = parse("{salt}");
for (const token of tokens) {
  if (token.token === "Ingredient") {
    console.log(token.content.name); // string
    console.log(token.content.quantity); // string | null
  }
}
```

`parse(input: string): Token[]` 同步返回普通 JS 数组，不需要 `JSON.parse`。
Token 使用 `{ token, content }` 判别联合：

| token | content |
| --- | --- |
| Metadata | `{ key: string, value: string }` |
| Ingredient / RecipeRef | `{ name: string, quantity: string \| null, unit: string \| null }` |
| Timer / Material / Word / Space / Comment / Backstory | `string` |

- 数量保持原始字符串（如 `"1/2"`），缺失数量或单位返回 `null`。
- 保留解析器输出的空白和注释，不额外 trim 输入。
- 解析或序列化失败时抛出 JS `Error`，可使用 `try/catch`。
- 本模块沿用 `recipe-lang` 的宽松语法，不是严格校验器：
  无法识别的语法可能退回 `Word`，例如 `"{unclosed"`。
  原有数量校验和 backstory 解析限制不会在 WASM 层另行修正。

## 测试

```sh
# 原生编译检查
cargo check -p recipe-wasm
cargo check -p recipe-lang --no-default-features --features wasm

# 在 Node 中运行 Rust WASM 测试
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
  cargo test -p recipe-wasm --target wasm32-unknown-unknown

# 按上面的 Node.js 构建步骤生成 pkg-node 后，验证真正的 JS 导出接口
node --test tests/node.cjs
```
