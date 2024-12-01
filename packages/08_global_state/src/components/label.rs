use leptos::prelude::*;

use crate::class_names;

#[component]
pub fn Label(text: impl ToString, children: ChildrenFn) -> impl IntoView {
    view! {
        <label class="flex flex-col">
            <span class=class_names!(
                "text-xs",
                "text-white/50",
                "py-1",
                "px-2"
            )>{text.to_string()}</span>
            {children()}
        </label>
    }
}
