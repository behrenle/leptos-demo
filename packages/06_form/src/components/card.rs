use leptos::prelude::*;

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center p-4 rounded-md bg-neutral-800">
            {children()}
        </div>
    }
}
