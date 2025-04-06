use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn TextField() -> Element{
    let mut _count:Signal<i32> = use_signal(|| 0);
    let con = true;

    rsx!{
        document::Link { rel: "stylesheet", href: MAIN_CSS } 
       p{ id:"tex",
        "Hello Tailwind CSS!"
       } 
       input { 
        hidden: con,
        placeholder: " Enter the hash value",id:"inp"
        }
    }
}