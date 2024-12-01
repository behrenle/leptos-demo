use components::{card::Card, page::Page};
use leptos::prelude::*;

mod components;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <Page>
                <Card>
                    <h1 class="mb-2 text-2xl font-bold">Dev Summit 2024</h1>
                    <p>Welcome to the Rust Ecosystem!</p>
                </Card>
            </Page>
        }
    })
}
