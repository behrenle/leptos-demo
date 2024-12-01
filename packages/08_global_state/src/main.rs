use components::page::Page;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use pages::{home::HomePage, login::LoginPage};
use state::StateProvider;

mod class_names;
mod components;
mod pages;
mod state;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <StateProvider>
                <Router>
                    <Page>
                        <Routes fallback=|| "Not found">
                            <Route path=path!("/") view=LoginPage />
                            <Route path=path!("/home") view=HomePage />
                        </Routes>
                    </Page>
                </Router>
            </StateProvider>
        }
    })
}
