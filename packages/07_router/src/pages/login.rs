use leptos::prelude::*;
use leptos_router::{hooks::use_navigate, NavigateOptions};

use crate::{
    class_names,
    components::{
        card::Card, heading::Heading, label::Label,
        password_input::PasswordInput, text_input::TextInput,
    },
};

#[component]
pub fn LoginPage() -> impl IntoView {
    let navigate = use_navigate();
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let login_failed = RwSignal::new(false);

    view! {
        <Card>
            <Heading>Login</Heading>
            <Show when=move || login_failed.get()>
                <p>Invalid login credentials</p>
            </Show>

            <form
                on:submit=move |event| {
                    event.prevent_default();
                    let username = username.get();
                    let password = password.get();
                    if username == "username" || password == "password" {
                        login_failed.set(false);
                        navigate("/home", NavigateOptions::default());
                    } else {
                        login_failed.set(true);
                    }
                }
                class=class_names!(
                    "flex",
                    "flex-col",
                    "gap-2",
                )
            >
                <Label text="username">
                    <TextInput
                        value=username
                        on_change=move |value| {
                            username.set(value);
                        }
                    />
                </Label>

                <Label text="Password">
                    <PasswordInput
                        value=password
                        on_change=move |value| {
                            password.set(value);
                        }
                    />
                </Label>

                <button
                    type="submit"
                    class="p-2 mt-4 bg-lime-600 rounded-sm hover:bg-lime-400"
                >
                    Login
                </button>
            </form>
        </Card>
    }
}
