use yew::prelude::*;

mod ui;
mod components;
mod modules;
mod pages;

use components::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <navbar::Navbar />
            <profile::Profile />
            <content::Content />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}