use leptos::prelude::*;

#[component]
pub fn Greeting(name: String) -> impl IntoView {
    view! { <p>{format!("Hello {name}, nice to meet you!")}</p> }
}

#[component]
pub fn GreetingV2(
    #[prop(into, default = String::from("mysterious being"))] name: String,
) -> impl IntoView {
    view! { <p>{format!("Hello {name}, nice to meet you!")}</p> }
}
