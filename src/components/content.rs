use yew::prelude::*;

use crate::ui::*;

#[function_component(Content)]
pub fn content() -> Html {
    html! {
        <main id="content" class="portfolio">
            <work::Work h3={"Unity Visual Effect Graph測試"} p_content={"使用Visual Effect Graph製作粒子特效。<br/> <br/> 製作工具：Unity"}/>
            <work::Work1 />
            <work::Work2 />
            <work::Work3 />
            <work::Work4 />
            <work::Work5 />
        </main>
    }
}