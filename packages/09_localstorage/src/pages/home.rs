use leptos::prelude::*;
use leptos_router::{hooks::use_navigate, NavigateOptions};

use crate::{
    class_names,
    components::{card::Card, heading::Heading},
    state::{use_dispatch, use_state, AppAction},
};

#[component]
pub fn HomePage() -> impl IntoView {
    let state = use_state();
    let dispatch = use_dispatch();
    let navigate = use_navigate();

    Effect::new({
        let navigate = navigate.clone();
        move |_| {
            let state = state.get();
            if state.session.is_none() {
                navigate("/login", NavigateOptions::default());
            }
        }
    });

    view! {
        <Card class=class_names!("gap-4")>
            <Heading>home</Heading>
            <p>
                {format!(
                    "Hello {}, nice to meet you!",
                    state
                        .get()
                        .session
                        .map_or("Unknown".to_string(), |session| session.username),
                )}
            </p>
            <button
                on:click=move |_| {
                    dispatch.run(AppAction::Logout).expect("logout");
                    navigate("/login", NavigateOptions::default());
                }
                class=class_names!(
                    "p-2",
                    "border",
                    "border-neutral-500",
                    "hover:border-white",
                    "hover:bg-neutral-700"
                )
            >
                Logout
            </button>
        </Card>
    }
}
