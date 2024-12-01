use leptos::prelude::*;

use crate::class_names;

#[component]
pub fn PasswordInput(
    #[prop(into)] on_change: Callback<(String,)>,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    view! {
        <input
            type="password"
            class=class_names!(
                "py-1",
                "px-2",
                "rounded-sm",
                "border",
                "outline-none",
                "bg-neutral-700",
                "border-neutral-600",
                "focus-visible:border-neutral-400"
            )
            prop:value=value
            on:input:target=move |event| {
                on_change.run((event.target().value(),));
            }
        />
    }
}
