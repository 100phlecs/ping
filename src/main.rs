// main.rs

use sycamore::prelude::*;

pub fn main() {
    sycamore::render(|| view! {
        p { "Hello, World!" }
    });
}
