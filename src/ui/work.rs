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

#[function_component(Work)]
pub fn work(props: &Props) -> Html {
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

#[function_component(Work1)]
pub fn work1() -> Html {
    html! {
        <article class="work">
            <h3>{"Unity Visual Effect Graph測試"}</h3>
            <p>
                {"使用Visual Effect Graph製作粒子特效。"}
            </p>
            <br/>
            <br/>
            <p>
                {"製作工具：Unity"}
            </p>
        </article>
    }
}
#[function_component(Work2)]
pub fn work2() -> Html {
    html! {
        <article class="work">
            <h3>{"Unreal Engine 5佈景測試"}</h3>
            <p>
                {"摸索Unreal Engine 5，並使用Quixel Bridge資源在Unreal Engine內隨意佈景。"}
            </p>
            <br/>
            <br/>
            <p>
                {"製作工具：Unreal Engine 5"}
            </p>
        </article>
    }
}
#[function_component(Work3)]
pub fn work3() -> Html {
    html! {
        <article class="work">
            <h3>{"DOTS架構實作練習"}</h3>
            <p>
                {"使用ECS架構進行程式撰寫，使自己對ECS有了了解，並將程式碼修改為0.50版本，使遊戲可以正常運行。
                也稍微學習了Shader Graph(圖中類似火焰的地方)，並將球加上VFX。"}
            </p>
            <br/>
            <br/>
            <p>
                {"製作工具：Unity"}
            </p>
        </article>
    }
}
#[function_component(Work4)]
pub fn work4() -> Html {
    html! {
        <article class="work">
            <h3>{"John Lemon's Haunted Jaunt 教學專案作品"}</h3>
            <p>
                {"在此教學中，學習到了如何編寫操控角色，環境的燈光調整，與使用Cinemachine來使相機追蹤角色，並使用Post-Processing幫鏡頭加上特效。
                也學習放置靜態與動態的敵人與其導航網格(AI)功能，來控制動態敵人的移動。"}
            </p>
            <br/>
            <br/>
            <p>
                {"製作工具：Unity"}
            </p>
        </article>
    }
}
#[function_component(Work5)]
pub fn work5() -> Html {
    html! {
        <article class="work">
            <h3>{"Survival Shooter 教學專案作品"}</h3>
            <p>
                {"在此教學中，學習到了如何製作UI，利用相機投射到網格並將值傳到角色身上使角色旋轉，並編寫生命值與傷害等功能，並在遊戲中生成敵人與用Audio Mixer調整音效。"}
            </p>
            <br/>
            <br/>
            <p>
                {"製作工具：Unity"}
            </p>
        </article>
    }
}