use yew::prelude::*;

use crate::ui::gallery;

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
pub fn work() -> Html {
    html! {
        <article class="work">
            <h3>{"Unity Visual Effect Graph測試"}</h3>
            <gallery::Gallery2img />
            // <img src="./static/images/VEG Demo01-Chiang-compressed.webp" alt="VEG Demo01-Chiang" class="hover-shadow cursor modal-open veg1" />
            // <div class="thumbnail">
            //     <img src="./static/images/VEG Demo01-Chiang-compressed.webp" alt="VEG Demo01-Chiang" class="hover-shadow cursor modal-open veg1" />
            //     <img src="./static/images/VEG Fireworks-Lite-675p-compressed.webp" alt="VEG Fireworks" class="hover-shadow cursor modal-open veg2" />
            // </div>
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
pub fn work() -> Html {
    let video = format!("<video src='./static/videos/UE5_Demo001 3s-1080p.webm' controls autoplay loop></video>");

    html! {
        <article class="work">
            <h3>{"Unreal Engine 5佈景測試"}</h3>
            {Html::from_html_unchecked(AttrValue::from(video))}
            // <video src="./static/videos/UE5_Demo001 3s-1080p.webm"></video>
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
pub fn work() -> Html {
    html! {
        <article class="work">
            <h3>{"DOTS架構實作練習"}</h3>
            <img class="hover-shadow cursor modal-open" id="dotsPong" src="./static/images/DOTS Pong-compressed.webp" alt="DOTS Pong" />
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
pub fn work() -> Html {
    html! {
        <article class="work">
            <h3>{"John Lemon's Haunted Jaunt 教學專案作品"}</h3>
            <img src="./static/images/John Lemon's Haunted Jaunt.png" alt="John Lemon's Haunted Jaunt 教學專案作品" />
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
pub fn work() -> Html {
    html! {
        <article class="work">
            <h3>{"Survival Shooter 教學專案作品"}</h3>
            <img src="./static/images/Survival Shooter.png" alt="Survival Shooter 教學專案作品" />
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
#[function_component(Work6)]
pub fn work() -> Html {
    html! {
        <article class="work">
            <img class="hover-shadow cursor modal-open" id="blenderLion" src="./static/images/blender lion demo.png" alt="blender作品" />
            <p>{"製作工具：blender"}</p> 
        </article>
    }
}
#[function_component(Work7)]
pub fn work() -> Html {
    html! {
        <article class="work">
            <img class="hover-shadow cursor modal-open" id="smallRoom" src="./static/images/smallroom.jpg" alt="smallroom" />
            <p>{"製作工具：blender"}</p> 
        </article>
    }
}
#[function_component(Work8)]
pub fn work() -> Html {
    html! {
        <article class="work">
            <h3>{"嘗試自己寫作品集網站"}</h3>
            <p>
                {"此作品集網站就是我的作品之一，因為想學會如何撰寫網頁，所以就寫了自己的作品集RWD網站。"}
                <br/>
                {"自主學習html、css、javascript等"}
                <br/>
                {"並且使用webpack進行打包，與使用GitHub Pages發布。"}
                <br/>
                <br/>
                {"使用工具：JavaScript、Webpack、Html、CSS、GitHub、VS Code"}
            </p>
        </article>
    }
}
#[function_component(Work9)]
pub fn work() -> Html {
    html! {
        <article class="work">
            <h3>{"使用Rust改寫作品集網站"}</h3>
            <p>
                {"使用Rust語言與Yew框架重新編寫本作品集網站"}<br/>
                <br/>
                {"使用工具：Rust、Yew、Html、CSS、GitHub、VS Code"}<br/>
                <br/>
                <br/>
                {"目前仍在製作中，可能仍有許多問題在..."}
                <br/>
                <a href="https://github.com/YuChiang-Chang/user_github_pages_yew">{"https://github.com/YuChiang-Chang/user_github_pages_yew"}</a><br/>
                <br/>
                {"2024/01/07 開始製作"}<br/>
                {"2024/01/14 發佈到Github Pages測試 (yuchiang-chang.github.io)"}
            </p>
        </article>
    }
}
#[function_component(Work10)]
pub fn work() -> Html {
    html! {
        <article class="work">
            <h3>{"畢業專題"}</h3>
            <img src="./static/images/fornt-light.png" alt="畢業專題" />
            <p>
                {"以掃地機器人為探討基礎，
                導入深度學習與影像辨識，
                將樹莓派加裝鏡頭，
                使樹莓派可透過鏡頭進行影像辨識，
                並透過加入深度學習使樹莓派可進行物體識別。"}<br/>
                <br/>
                {"使用工具與技術：python、樹莓派、OpenCV、Caffe、CNN"}
            </p>
        </article>
    }
}