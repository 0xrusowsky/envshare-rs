use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, BlobPropertyBag, File, FileReader, HtmlElement, Url};
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct BlockInput {
    value: String,
    height: i32,
}

impl BlockInput {
    pub fn new(value: String, height: i32) -> Self {
        Self { value, height }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}
