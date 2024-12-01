use components::{
    card::Card,
    greeting::{Greeting, GreetingV2},
    page::Page,
};
use leptos::prelude::*;

mod components;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <Page>
                <Card>
                    <h1 class="mb-2 text-2xl font-bold">Dev Summit 2024</h1>
                    <Greeting name=String::from("Ferris") />
                    <GreetingV2 name="Ferris" />
                    <GreetingV2 />
                </Card>
            </Page>
        }
    })
}
