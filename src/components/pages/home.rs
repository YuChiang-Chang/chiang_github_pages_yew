use yew::prelude::*;

// use crate::components::*;
use crate::atoms::*;
use crate::organisms::portfolio::*;
use crate::organisms::*;
use crate::organisms::work::*;
// use super::super::templates;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <main>
            <Portfolio
                attr_id="web"
                title="網頁作品"
            >
                <work::Work 
                    title="咖啡豆庫存管理系統"
                    media={Some(work::MediaType::Image { 
                        src: "./static/images/螢幕擷取畫面_12-3-2024_203531_staging.beansinventory.jadeite.com.tw.jpeg".to_string(), 
                        alt: "咖啡豆庫存管理系統".to_string(),
                    })}
                    content="使用 Vue 和 Django 構建的咖啡豆庫存管理系統，支持咖啡豆的增刪改操作，並實現了用戶管理功能。\n\n技術堆疊： 前端：Vue、後端：Django、資料庫：MySQL"
                    // tech_stack="技術堆疊： 前端：Vue、後端：Django、資料庫：MySQL"
                    link_url="https://github.com/YuChiang-Chang/roasted-beans-inventory-system"
                    link_text="https://github.com/YuChiang-Chang/roasted-beans-inventory-system"
                />
                <Work
                    title="待辦事項管理應用"
                    media={Some(work::MediaType::Image { 
                        src: "./static/images/螢幕擷取畫面_11-2-2024_193947_localhost.jpeg".to_string(), 
                        alt: "待辦事項管理應用".to_string(),
                    })}
                >
                    <p>
                        {"
                        一個簡單的待辦事項管理應用，
                        使用React前端框架和Node.js後端實現。
                        允許用戶新增、檢視、刪除待辦事項。
                        "}<br/>
                        <br/>
                        {"技術堆疊：前端：React、
                        後端：Node.js, Express、
                        資料庫：MongoDB"}<br/>
                        <br/>
                        <a href="https://github.com/YuChiang-Chang/todo-api">
                        {"https://github.com/YuChiang-Chang/todo-api"}
                        </a><br/>
                    </p>
                </Work>
                <work::Work
                    title="使用Rust改寫作品集網站"
                    content="使用Rust語言與Yew框架重新編寫本作品集網站\n\n技術堆疊：Rust、Yew、Html、SCSS、GitHub Pages"
                    link_url="https://github.com/YuChiang-Chang/chiang_github_pages_yew"
                    link_text="https://github.com/YuChiang-Chang/chiang_github_pages_yew"
                />
                <work::Work
                    title="作品集網站"
                    // content="此作品集網站就是我的作品之一，因為想學會如何撰寫網頁，所以就寫了自己的作品集RWD網站。\n自主學習html、css、javascript等\n並且使用webpack進行打包，與使用GitHub Pages發布。\n\n使用工具：JavaScript、Webpack、Html、CSS、GitHub、VS Code"
                >
                    <p>
                        {"此作品集網站就是我的作品之一，因為想學會如何撰寫網頁，所以就寫了自己的作品集RWD網站。"}<br/>
                        {"自主學習html、css、javascript等"}<br/>
                        {"並且使用webpack進行打包，與使用GitHub Pages發布。"}<br/>
                        <br/>
                        {"技術堆疊：JavaScript、Webpack、Html、CSS、GitHub Pages"}
                    </p>
                </work::Work>
            </Portfolio>
            <Portfolio
                attr_id="game"
                title="遊戲作品"
            >
                <Work 
                    title="Unity Visual Effect Graph測試"
                    // media={Some(work::MediaType::Gallery {
                    //     feature_image.src: "./static/images/VEG Demo01-Chiang-compressed.webp".to_string(),
                    // })}
                >
                    <gallery::Gallery2img />
                    <p  class="pre-wrap">
                        {"使用Visual Effect Graph製作粒子特效。"}
                    </p>
                    <br/>
                    <br/>
                    <p>
                        {"技術堆疊：Unity"}
                    </p>
                </Work>
                <Work
                    title="Unreal Engine 5佈景測試"
                    media={Some(work::MediaType::Video(
                        "./static/videos/UE5_Demo001 3s-1080p.webm".to_string(),
                    ))}
                >
                    // {Html::from_html_unchecked(AttrValue::from(video))}
                    // <video src="./static/videos/UE5_Demo001 3s-1080p.webm"></video>
                    <p>
                        {"摸索Unreal Engine 5，並使用Quixel Bridge資源在Unreal Engine內隨意佈景。"}
                    </p>
                    <br/>
                    <br/>
                    <p>
                        {"製作工具：Unreal Engine 5"}
                    </p>
                </Work>
                <Work
                    title="DOTS架構實作練習"
                    media={Some(work::MediaType::Image {
                        src: "./static/images/DOTS Pong-compressed.webp".to_string(),
                        alt: "DOTS Pong".to_string(),
                    })}
                >
                    // <img class="hover-shadow cursor modal-open" id="dotsPong" src="./static/images/DOTS Pong-compressed.webp" alt="DOTS Pong" />
                    <p>
                        {"使用ECS架構進行程式撰寫，使自己對ECS有了了解，並將程式碼修改為0.50版本，使遊戲可以正常運行。
                        也稍微學習了Shader Graph(圖中類似火焰的地方)，並將球加上VFX。"}
                    </p>
                    <br/>
                    <br/>
                    <p>
                        {"製作工具：Unity"}
                    </p>
                </Work>
                <Work
                    title="John Lemon's Haunted Jaunt 教學專案作品"
                    media={Some(work::MediaType::Image { 
                        src: "./static/images/John Lemon's Haunted Jaunt.png".to_string(), 
                        alt: "John Lemon's Haunted Jaunt 教學專案作品".to_string(), 
                    })}
                >
                    <p>
                        {"在此教學中，學習到了如何編寫操控角色，環境的燈光調整，與使用Cinemachine來使相機追蹤角色，並使用Post-Processing幫鏡頭加上特效。
                        也學習放置靜態與動態的敵人與其導航網格(AI)功能，來控制動態敵人的移動。"}
                    </p>
                    <br/>
                    <br/>
                    <p>
                        {"製作工具：Unity"}
                    </p>
                </Work>
                <Work
                    title="Survival Shooter 教學專案作品"
                    media={Some(work::MediaType::Image { 
                        src: "./static/images/Survival Shooter.png".to_string(), 
                        alt: "Survival Shooter 教學專案作品".to_string(),
                    })}
                >
                    <p>
                        {"在此教學中，學習到了如何製作UI，利用相機投射到網格並將值傳到角色身上使角色旋轉，並編寫生命值與傷害等功能，並在遊戲中生成敵人與用Audio Mixer調整音效。"}
                    </p>
                    <br/>
                    <br/>
                    <p>
                        {"製作工具：Unity"}
                    </p>
                </Work>
            </Portfolio>
            <Portfolio
                attr_id="model"
                title="模型作品"
            >
                <Work
                    media={Some(work::MediaType::Image { 
                        src: "./static/images/blender lion demo.png".to_string(), 
                        alt: "blender作品".to_string(),
                    })}
                >
                    // <img class="hover-shadow cursor modal-open" id="blenderLion" src="./static/images/blender lion demo.png" alt="blender作品" />
                    <p>{"製作工具：blender"}</p>
                </Work>
                <Work
                    media={Some(work::MediaType::Image { 
                        src: "./static/images/smallroom.jpg".to_string(), 
                        alt: "smallroom".to_string(), 
                    })}
                >
                    // <img class="hover-shadow cursor modal-open" id="smallRoom" src="./static/images/smallroom.jpg" alt="smallroom" />
                    <p>{"製作工具：blender"}</p> 
                </Work>
            </Portfolio>
            <Portfolio
                attr_id="others"
                title="其他"
            >
                <Work
                    title="畢業專題"
                    media={Some(work::MediaType::Image { src: "./static/images/fornt-light.png".to_string(), alt: "畢業專題".to_string() })}
                >
                    <p>
                        {"以掃地機器人為探討基礎，
                        導入深度學習與影像辨識，
                        將樹莓派加裝鏡頭，
                        使樹莓派可透過鏡頭進行影像辨識，
                        並透過加入深度學習使樹莓派可進行物體識別。"}<br/>
                        <br/>
                        {"使用工具與技術：python、樹莓派、OpenCV、Caffe、CNN"}
                    </p>
                </Work>
            </Portfolio>
        </main>
    }
}

