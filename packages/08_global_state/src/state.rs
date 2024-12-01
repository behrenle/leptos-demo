use leptos::prelude::*;

mod action;
mod session;
mod state;

pub use action::*;
pub use state::*;

#[derive(Debug, Clone)]
struct Dispatch(Callback<AppAction, Result<(), &'static str>>);

#[component]
pub fn StateProvider(children: Children) -> impl IntoView {
    let state = RwSignal::new(State::default());
    provide_context(state.read_only());

    let dispatch =
        Callback::new(move |action: AppAction| -> Result<(), &'static str> {
            let mut new_state = state.get();
            new_state.apply(action)?;
            state.set(new_state);
            Ok(())
        });

    provide_context(Dispatch(dispatch));

    view! { <>{children()}</> }
}

pub fn use_state() -> ReadSignal<State> {
    use_context::<ReadSignal<State>>().expect("context")
}

pub fn use_dispatch() -> Callback<AppAction, Result<(), &'static str>> {
    use_context::<Dispatch>().expect("context").0
}
