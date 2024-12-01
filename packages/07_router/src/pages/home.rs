use leptos::prelude::*;

use crate::components::{card::Card, heading::Heading};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Card>
            <Heading>home</Heading>
            test
        </Card>
    }
}
