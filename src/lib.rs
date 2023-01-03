use wasm_bindgen::prelude::*;

pub use wasm_bindgen_rayon::init_thread_pool;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

#[wasm_bindgen] // modifies extern to call JS functions
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen] // modify fn to be called by JS
pub fn greet(name: &str) {
    alert(&format!("Hello, {} with rayon!", name));
}

// #[wasm_bindgen]
// pub fn sum(numbers: &[i32]) {
// pub fn sum(numbers: &[i32]) -> i32 {
// alert(&format!("Sum is: {}", numbers[0] + numbers[1]));
// numbers.par_iter().sum()
// }
