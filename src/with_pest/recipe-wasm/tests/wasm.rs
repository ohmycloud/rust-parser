#![cfg(target_arch = "wasm32")]

use recipe_wasm::parse;
use serde_json::{Value, json};
use wasm_bindgen_test::wasm_bindgen_test;

fn parse_value(input: &str) -> Value {
    let result = parse(input).expect("recipe should parse");
    assert!(js_sys::Array::is_array(&result));
    serde_wasm_bindgen::from_value(result).expect("tokens should be JSON-compatible")
}

#[wasm_bindgen_test]
fn empty_input_returns_an_empty_array() {
    assert_eq!(parse_value(""), json!([]));
}

#[wasm_bindgen_test]
fn metadata_is_a_tagged_object() {
    assert_eq!(
        parse_value(">> name: Soup\n"),
        json!([
            {"token": "Metadata", "content": {"key": "name", "value": "Soup"}},
            {"token": "Space", "content": "\n"}
        ])
    );
}

#[wasm_bindgen_test]
fn ingredients_and_references_preserve_amounts_and_nulls() {
    assert_eq!(
        parse_value("{salt} {water}(100 ml) @{stock}(1/2 cup) @{sauce}"),
        json!([
            {"token": "Ingredient", "content": {"name": "salt", "quantity": null, "unit": null}},
            {"token": "Space", "content": " "},
            {"token": "Ingredient", "content": {"name": "water", "quantity": "100", "unit": "ml"}},
            {"token": "Space", "content": " "},
            {"token": "RecipeRef", "content": {"name": "stock", "quantity": "1/2", "unit": "cup"}},
            {"token": "Space", "content": " "},
            {"token": "RecipeRef", "content": {"name": "sauce", "quantity": null, "unit": null}}
        ])
    );
}

#[wasm_bindgen_test]
fn timers_materials_comments_and_backstory_are_preserved() {
    assert_eq!(
        parse_value("&{pot} t{5 minutes} /* note */\n---\nA story"),
        json!([
            {"token": "Material", "content": "pot"},
            {"token": "Space", "content": " "},
            {"token": "Timer", "content": "5 minutes"},
            {"token": "Space", "content": " "},
            {"token": "Comment", "content": "note"},
            {"token": "Backstory", "content": "A story"}
        ])
    );
}

#[wasm_bindgen_test]
fn unicode_and_whitespace_survive_the_boundary() {
    assert_eq!(
        parse_value("搅拌\t🍲\r\n"),
        json!([
            {"token": "Word", "content": "搅拌"},
            {"token": "Space", "content": "\t"},
            {"token": "Word", "content": "🍲"},
            {"token": "Space", "content": "\r\n"}
        ])
    );
}

#[wasm_bindgen_test]
fn result_does_not_borrow_the_rust_input() {
    let result = {
        let input = String::from("{salt}");
        parse(&input).unwrap()
    };
    let value: Value = serde_wasm_bindgen::from_value(result).unwrap();
    assert_eq!(
        value,
        json!([{"token": "Ingredient", "content": {"name": "salt", "quantity": null, "unit": null}}])
    );
}

#[wasm_bindgen_test]
fn unrecognized_syntax_keeps_the_underlying_parsers_word_fallback() {
    assert_eq!(
        parse_value("{unclosed"),
        json!([{"token": "Word", "content": "{unclosed"}])
    );
}
