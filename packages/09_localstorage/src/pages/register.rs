use leptos::prelude::*;
use leptos_router::{hooks::use_navigate, NavigateOptions};

use crate::{
    components::{
        card::Card, heading::Heading, label::Label,
        password_input::PasswordInput, text_input::TextInput,
    },
    state::{use_dispatch, AppAction},
};

#[component]
pub fn RegisterPage() -> impl IntoView {
    let dispatch = use_dispatch();
    let navigate = use_navigate();
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let confirm_password = RwSignal::new(String::new());
    let error = RwSignal::<Option<String>>::new(None);

    view! {
        <Card>
            <Heading>Register</Heading>
            <Show when=move || error.get().is_some()>
                <p class="p-2 mt-2 text-xs text-red-400 rounded-sm border border-red-400">
                    {error.get()}
                </p>
            </Show>
            <form
                class="flex flex-col gap-2"
                on:submit=move |event| {
                    event.prevent_default();
                    let username = username.get();
                    let password = password.get();
                    let confirm_password = confirm_password.get();
                    if username.is_empty() {
                        error.set(Some("Missing username".to_string()));
                    } else if password.is_empty() {
                        error.set(Some("Missing password".to_string()));
                    } else if password != confirm_password {
                        error.set(Some("Passwords do not match".to_string()));
                    } else {
                        match dispatch
                            .run(AppAction::Register {
                                username,
                                password,
                            })
                        {
                            Err(err) => error.set(Some(err.to_string())),
                            _ => {
                                navigate("/", NavigateOptions::default());
                            }
                        }
                    }
                }
            >
                <Label text="Username">
                    <TextInput
                        value=username
                        on_change=move |value| username.set(value)
                    />
                </Label>

                <Label text="Password">
                    <PasswordInput
                        value=password
                        on_change=move |value| password.set(value)
                    />
                </Label>

                <Label text="Confirm password">
                    <PasswordInput
                        value=confirm_password
                        on_change=move |value| confirm_password.set(value)
                    />
                </Label>

                <button
                    type="submit"
                    class="p-2 mt-4 bg-lime-600 rounded-sm hover:bg-lime-400"
                >
                    Register
                </button>
            </form>
        </Card>
    }
}
