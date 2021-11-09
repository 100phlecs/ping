// main.rs

use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

pub fn main() {
    sycamore::render(|| view! {
        p { "Hello, World!" }
    });
}