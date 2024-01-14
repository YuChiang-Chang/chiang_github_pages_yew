use yew::prelude::*;

// use crate::ui::*;
use crate::components::*;

#[function_component(Content)]
pub fn content() -> Html {
    html! {
        <main >
            // <div class="content">
            // </div>
            <h2>{"遊戲作品"}</h2>
            <hr class="h1-hr" />
            <section id="game" class="portfolio">
                // <work::Work h3={"Unity Visual Effect Graph測試"} p_content={"使用Visual Effect Graph製作粒子特效。<br/> <br/> \n製作工具：Unity"}/>
                <work::Work1 />
                <work::Work2 />
                <work::Work3 />
                <work::Work4 />
                <work::Work5 />
            </section>
            <h2>{"模型作品"}</h2>
            <hr class="h1-hr" />
            <section id="model" class="portfolio">
                <work::Work6 />
                <work::Work7 />
            </section>
            <h2>{"網頁作品"}</h2>
            <hr class="h1-hr" />
            <section id="web" class="portfolio">
                <work::Work8 />
                <work::Work9 />
            </section>
        </main>
    }
}