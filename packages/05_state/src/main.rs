use components::{card::Card, page::Page};
use leptos::prelude::*;

mod components;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        let count = RwSignal::new(0);

        view! {
            <Page>
                <Card>
                    <h1 class="text-2xl font-bold">Dev Summit 2024</h1>
                    <p class="my-2">
                        {move || format!("Click count: {}", count.get())}
                    </p>
                    <button
                        on:click=move |_| count.update(|count| *count += 1)
                        class="p-2 rounded-sm border hover:border-white border-neutral-500 hover:bg-neutral-700"
                    >
                        Click me
                    </button>
                </Card>
            </Page>
        }
    })
}
