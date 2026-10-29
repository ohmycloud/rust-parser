const assert = require("node:assert/strict");
const { test } = require("node:test");
const { parse } = require("../pkg-node/recipe_wasm.js");

test("the generated binding returns ordinary JS arrays and objects", () => {
  assert.deepEqual(parse("{water}(100 ml)"), [
    {
      token: "Ingredient",
      content: { name: "water", quantity: "100", unit: "ml" },
    },
  ]);
  assert.deepEqual(parse(""), []);
});

test("missing amounts are null, not undefined", () => {
  assert.deepEqual(parse("@{stock}"), [
    {
      token: "RecipeRef",
      content: { name: "stock", quantity: null, unit: null },
    },
  ]);
});

test("the generated UTF-8 binding preserves Unicode and whitespace", () => {
  assert.deepEqual(parse("搅拌 🍲\n"), [
    { token: "Word", content: "搅拌" },
    { token: "Space", content: " " },
    { token: "Word", content: "🍲" },
    { token: "Space", content: "\n" },
  ]);
});

test("results remain valid across repeated calls", () => {
  const first = parse("{salt}");
  for (let index = 0; index < 1000; index += 1) {
    assert.equal(parse(`{water}(${index} ml)`)[0].content.quantity, String(index));
  }
  assert.deepEqual(first, [
    {
      token: "Ingredient",
      content: { name: "salt", quantity: null, unit: null },
    },
  ]);
});
