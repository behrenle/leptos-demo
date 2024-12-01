use components::{card::Card, page::Page};
use leptos::{logging::log, prelude::*};

mod class_names;
mod components;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        let username = RwSignal::new(String::new());
        let password = RwSignal::new(String::new());

        view! {
            <Page>
                <Card>
                    <h1 class="text-2xl font-bold">Login</h1>
                    <form
                        on:submit=move |event| {
                            event.prevent_default();
                            let username = username.get();
                            let password = password.get();
                            if username.is_empty() || password.is_empty() {
                                log!("username or password not provided");
                            } else {
                                log!("login: username={username} password={password}");
                            }
                        }
                        class=class_names!(
                            "flex",
                            "flex-col",
                            "gap-2",
                        )
                    >
                        <label class="flex flex-col">
                            <span class=class_names!(
                                "text-xs",
                                "text-white/50",
                                "py-1",
                                "px-2"
                            )>Username</span>
                            <input
                                type="text"
                                prop:value=username
                                on:input:target=move |event| {
                                    username.set(event.target().value())
                                }
                                class=class_names!(
                                    "py-1",
                                    "px-2",
                                    "rounded-sm",
                                    "border",
                                    "outline-none",
                                    "bg-neutral-700",
                                    "border-neutral-600",
                                    "focus-visible:border-neutral-400"
                                )
                            />
                        </label>
                        <label class="flex flex-col">
                            <span class=class_names!(
                                "text-xs",
                                "text-white/50",
                                "py-1",
                                "px-2"
                            )>Password</span>
                            <input
                                type="password"
                                prop:value=password
                                on:input:target=move |event| {
                                    password.set(event.target().value())
                                }
                                class=class_names!(
                                    "py-1",
                                    "px-2",
                                    "rounded-sm",
                                    "border",
                                    "outline-none",
                                    "bg-neutral-700",
                                    "border-neutral-600",
                                    "focus-visible:border-neutral-400"
                                )
                            />
                        </label>
                        <button
                            type="submit"
                            class="p-2 mt-4 bg-lime-600 rounded-sm hover:bg-lime-400"
                        >
                            Login
                        </button>
                    </form>
                </Card>
            </Page>
        }
    })
}
