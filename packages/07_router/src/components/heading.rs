use leptos::prelude::*;

#[component]
pub fn Heading(children: Children) -> impl IntoView {
    view! { <h1 class="text-2xl font-bold">{children()}</h1> }
}
