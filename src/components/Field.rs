use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn TextField() -> Element{
    let mut hash_value = use_signal(|| String::from(""));
    rsx!{
        document::Link { rel: "stylesheet", href: MAIN_CSS } 
        h1 { id:"H1",
            "Rust Cracker"
        }
        input{id:"inp",
            placeholder: "Enter the Hash Value", oninput: move |e| hash_value.set(e.value()) 
        }
        button{ id:"btn",
            "Crack the Hash"
        }
    }
}

