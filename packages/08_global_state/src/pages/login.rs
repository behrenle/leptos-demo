use leptos::prelude::*;
use leptos_router::{hooks::use_navigate, NavigateOptions};

use crate::{
    class_names,
    components::{
        card::Card, heading::Heading, label::Label,
        password_input::PasswordInput, text_input::TextInput,
    },
    state::{use_dispatch, AppAction},
};

#[component]
pub fn LoginPage() -> impl IntoView {
    let navigate = use_navigate();
    let dispatch = use_dispatch();
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let login_error = RwSignal::<Option<String>>::new(None);

    view! {
        <Card>
            <Heading>Login</Heading>
            <Show when=move || login_error.get().is_some()>
                <p>{login_error.get()}</p>
            </Show>

            <form
                on:submit=move |event| {
                    event.prevent_default();
                    let username = username.get();
                    let password = password.get();
                    match dispatch
                        .run(AppAction::Login {
                            username,
                            password,
                        })
                    {
                        Ok(_) => navigate("/home", NavigateOptions::default()),
                        Err(err) => {
                            login_error.set(Some(err.to_string()));
                        }
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
