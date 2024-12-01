use leptos::prelude::*;

#[component]
pub fn Page(children: Children) -> impl IntoView {
    view! {
        <div class="flex absolute inset-0 flex-col justify-center items-center text-white bg-black">
            {children()}
        </div>
    }
}
