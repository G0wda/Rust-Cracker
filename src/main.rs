use components::TextField::TextField;
use dioxus::prelude::*;
mod components;




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
