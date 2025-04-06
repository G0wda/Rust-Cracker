use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn TextField() -> Element{
    

    rsx!{
        document::Link { rel: "stylesheet", href: MAIN_CSS } 
        h1 { id:"H1",
            "Rust Cracker"
        }
    }
}

