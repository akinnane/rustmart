mod api;
mod app;
mod components;
mod pages;
mod route;
mod types;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Hello {}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
