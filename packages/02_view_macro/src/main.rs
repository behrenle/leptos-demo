use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <div class="flex absolute inset-0 flex-col justify-center items-center">
                <h1 class="text-2xl font-bold">Dev Summit 2024</h1>
                <p>Welcome to the Rust Ecosystem!</p>
            </div>
        }
    })
}
