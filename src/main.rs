use yew::prelude::*;

// mod ui;
mod components;
// mod modules;
// mod pages;

// use ui::*;
use components::*;
// use modules::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <organisms::navbar::Navbar />
            <organisms::profile::Profile />
            <pages::home::HomePage />
            // <organisms::portfolio::Content1 />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}