use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_open: bool,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    if !props.is_open {
        return html! {};
    }

    html! {
        <div class="modal">
            <div class="modal-content">
                { for props.children.iter() }
            </div>
        </div>
    }
}