use dioxus::prelude::*;
mod components;

use components::TextField::TextField;



fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
       
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        TextField{}
    }
}
