//! JavaScript bindings for the recipe language parser.

use serde::Serialize;
use wasm_bindgen::prelude::*;

/// Parse recipe text into an array of tagged token objects.
///
/// Whitespace and comments are preserved. Missing quantities and units are
/// represented as `null`. Parser and serialization errors throw JavaScript
/// `Error` objects.
#[wasm_bindgen(unchecked_return_type = "Token[]")]
pub fn parse(input: &str) -> Result<JsValue, JsError> {
    let tokens = recipe_lang::parse(input).map_err(|error| JsError::new(&error.to_string()))?;

    tokens
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|error| JsError::new(&error.to_string()))
}
