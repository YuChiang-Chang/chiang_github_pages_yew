use yew::prelude::*;

// use crate::ui::*;
// use crate::components::*;
// use crate::organisms::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub attr_id: String,
    #[prop_or_default]
    pub title: String,
    // 使用 Yew 的 Children 來接收子組件，可在組件內加入html
    #[prop_or_default]
    pub children: Children, 
}

#[function_component(Portfolio)]
pub fn portfolio(props: &Props) -> Html {
    html! {
        <section id={props.attr_id.clone()} class="portfolio">
            <h2>{props.title.clone()}</h2>
            <hr class="h1-hr" />
            { for props.children.iter() }
        </section>
    }
}

// #[function_component(Content1)]
// pub fn content() -> Html {
//     html! {
//         <main >
//             // <div class="content">
//             // </div>
//             // <section id="wev" class="portfolio">
//             //     <h2>{"網頁作品"}</h2>
//             //     <hr class="h1-hr" />
//             //         <organisms::work::Work8 />
//             //         <work::Work9 />
//             //         <work::Work11 />
//             //         <work::Work 
//             //             title="咖啡豆庫存管理系統"
//             //             media={Some(work::MediaType::Image { 
//             //                 src: "./static/images/螢幕擷取畫面_12-3-2024_203531_staging.beansinventory.jadeite.com.tw.jpeg".to_string(), 
//             //                 alt: "咖啡豆庫存管理系統".to_string(),
//             //             })}
//             //             content="使用 Vue 和 Django 構建的咖啡豆庫存管理系統，支持咖啡豆的增刪改操作，並實現了用戶管理功能。"
//             //             // tech_stack="技術堆疊： 前端：Vue、後端：Django、資料庫：MySQL"
//             //             link_url="https://github.com/YuChiang-Chang/roasted-beans-inventory-system"
//             //             link_text="https://github.com/YuChiang-Chang/roasted-beans-inventory-system"
//             //         />
//             //         // <work::Work12 />
//             // </section>
//             <h2>{"遊戲作品"}</h2>
//             <hr class="h1-hr" />
//             <section id="game" class="portfolio">
//             // <work::Work h3={"Unity Visual Effect Graph測試"} p_content={"使用Visual Effect Graph製作粒子特效。<br/> <br/> \n製作工具：Unity"}/>
//                 <work::Work1 />
//                 <work::Work2 />
//                 <work::Work3 />
//                 <work::Work4 />
//                 <work::Work5 />
//             </section>
//             <h2>{"模型作品"}</h2>
//             <hr class="h1-hr" />
//             <section id="model" class="portfolio">
//                 <work::Work6 />
//                 <work::Work7 />
//             </section>
//             <h2>{"其他"}</h2>
//             <hr class="h1-hr" />
//             <section id="others" class="portfolio">
//                 <work::Work10 />
//             </section>
//             </main>
//         }
//     }