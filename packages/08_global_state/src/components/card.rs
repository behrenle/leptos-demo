use leptos::prelude::*;

#[component]
pub fn Card(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div class=format!(
            "flex flex-col justify-center p-4 rounded-md bg-neutral-800 {class}",
        )>{children()}</div>
    }
}
