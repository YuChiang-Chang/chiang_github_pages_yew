use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub h3: String,
    #[prop_or_default]
    pub p_content: String,
    #[prop_or_default]
    pub p_tool: String,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    html! {
        <article class="work">
            <h3>{props.h3.clone()}</h3>
            <p>
                {props.p_content.clone()}
            </p>
            <br/>
            <br/>
            <p>
                {props.p_tool.clone()}
            </p>
        </article>
    }
}