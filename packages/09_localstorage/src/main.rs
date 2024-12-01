use components::page::Page;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use pages::{home::HomePage, login::LoginPage, register::RegisterPage};
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
                            <Route path=path!("/login") view=LoginPage />
                            <Route path=path!("/") view=HomePage />
                            <Route path=path!("/register") view=RegisterPage />
                        </Routes>
                    </Page>
                </Router>
            </StateProvider>
        }
    })
}
