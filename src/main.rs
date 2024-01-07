use yew::prelude::*;

mod ui;
mod components;
mod modules;
mod pages;

use components::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            // <div id="container">
            <navbar::Navbar />
                // <div id="nav">
                //     //  <span id="open-right-navbtn" class="nav-btn" style="cursor: pointer;" href="#about">
                //     //     {"關於我"}
                //     // </span>
                //     <a id="aboutme-btn" href="#about" class="nav-btn">
                //         {"關於我"}
                //     </a> 
                //     <a href="#game" class="nav-btn">
                //         {"遊戲作品"}
                //     </a>
                //     <a href="#model" class="nav-btn">
                //         {"模型作品"}
                //     </a>
                //     <a href="#web" class="nav-btn">
                //         {"網頁作品"}
                //     </a>
                //     // <style>"style.css"</style>
                // </div>
            <profile::Profile />
            //     <div id="about" class="right-sidenav">
            //         <!-- <span style="cursor: pointer;">&Cross;</span> -->
            //         <a href="javascript:void(0)" id="close-about-btn">&Cross;</a>
            //         <div id="about-content">
            //             <h2 style="text-align: center;">關於我</h2>
            //             <p>
            //                 &ensp;&ensp;&ensp;&ensp;我是張佑強，歡迎來到我的作品集網站，我是個有興趣就會開始自學的人。會開始自學做遊戲的原因，是因為在大學期間，接觸了PLC（可程式化邏輯控制器），並自己嘗試編寫教授所教的以外的寫法，知道了自己適合撰寫程式，也取得了機電整合的證照。後續又因為接觸了3D，漸漸的對製作遊戲產生了興趣，並開始自學如何製作遊戲。
            //             </p>
            //             <p>學歷：華梵大學機電工程學系學士</p>
            //             <p>證照：機電整合丙級</p>
            //             <br>
            //             <h2 style="text-align: center;">聯絡我</h2>
            //             <p>
            //                 <!-- <i class="material-icons" style="margin: .1rem;">email</i> -->
            //                 &#9993; 信箱：
            //             </p>
            //             <a href="mailto:yuchiang.chang.taiwan@gmail.com">yuchiang.chang.taiwan@gmail.com</a>
            //         </div>
            //     </div>
            <content::Content />
            //     <div id="content">
            //         <h1 style="text-align: center;">作品集</h1>
            //         <div id="game">
            //             <h2>遊戲作品</h2>
            //             <hr class="h1-hr">
            //             <div class="portfolio">
            //                 <div class="work">
            //                     <!-- <video autoplay>
            //                         <source src="./images/VEG Demo01-Chiang.webm" type="video/webm">
            //                     </video> -->
            //                     <img src="./images/VEG Demo01-Chiang-compressed.webp" alt="VEG Demo01-Chiang" class="hover-shadow cursor modal-open veg1">
            //                     <div class="thumbnail">
            //                         <img src="./images/VEG Demo01-Chiang-compressed.webp" alt="VEG Demo01-Chiang" class="hover-shadow cursor modal-open veg1">
            //                         <img src="./images/VEG Fireworks-Lite-675p-compressed.webp" alt="VEG Fireworks" class="hover-shadow cursor modal-open veg2">
            //                     </div>
            //                     <div class="work-content">
            //                         <h3>Unity Visual Effect Graph測試</h3>
            //                         <p>
                                        // 使用Visual Effect Graph製作粒子特效。
                                        // <br>
                                        // <br>
                                        // 製作工具：Unity
            //                         </p>
            //                     </div>
            //                 </div>
            //                 <div class="work">
            //                     <video src="./videos/UE5_Demo001 3s-1080p.webm" controls autoplay loop></video>
            //                     <div class="work-content">
            //                         <h3>Unreal Engine 5佈景測試</h3>
            //                         <p>
            //                             摸索Unreal Engine 5，並使用Quixel Bridge資源在Unreal Engine內隨意佈景。
            //                             <br><br>
            //                             製作工具：Unreal Engine 5
            //                         </p>
            //                     </div>
            //                 </div>
            //                 <div class="work">
            //                     <img class="hover-shadow cursor modal-open" id="dotsPong" src="./images/DOTS Pong-compressed.webp" alt="DOTS Pong">
            //                     <div class="work-content">
            //                         <h3>DOTS架構實作練習</h3>
            //                         <p>
            //                             使用ECS架構進行程式撰寫，使自己對ECS有了了解，並將程式碼修改為0.50版本，使遊戲可以正常運行。也稍微學習了Shader Graph(圖中類似火焰的地方)，並將球加上VFX。
            //                             <br>
            //                             <br>
            //                             製作工具：Unity
            //                             <br>
            //                             <br>
            //                             <a href="https://www.bilibili.com/video/av88027106">教學連結</a>
            //                         </p>
            //                     </div>
            //                 </div>
            //                 <div class="work">
            //                     <img src="./images/John Lemon's Haunted Jaunt.png" alt="John Lemon's Haunted Jaunt 教學專案作品"
            //                         class="work-img">
            //                     <div class="work-content">
            //                         <h3>John Lemon's Haunted Jaunt 教學專案作品</h3>
            //                         <p>
            //                             遊戲試玩：
            //                             <br>
            //                             <a href="https://yuchiang-chang.github.io/John-Lemons-Haunted-Jaunt-WebGL">
            //                                 John Lemon's Haunted Jaunt 教學專案作品
            //                             </a>
            //                             <br>
            //                             <br>
            //                             在此教學中，學習到了如何編寫操控角色，環境的燈光調整，與使用Cinemachine來使相機追蹤角色，並使用Post-Processing幫鏡頭加上特效。
            //                             也學習放置靜態與動態的敵人與其導航網格(AI)功能，來控制動態敵人的移動。
            //                             <br>
            //                             <br>
            //                             而我也嘗試自己加了一個選單場景，以用於控制遊戲的開始，而非直接進入遊戲場景。使用TextMesh Pro來使文字有更好的畫質。
            //                             <br>
            //                             <br>
            //                             製作工具：Unity
            //                             <br>
            //                             <br>
            //                             <a href="https://learn.unity.com/project/john-lemon-s-haunted-jaunt-3d-beginner">教學連結</a>
            //                         </p>
            //                     </div>
            //                 </div>
            //                 <div class="work">
            //                     <img src="./images/Survival Shooter.png" alt="Survival Shooter 教學專案作品" class="work-img">
            //                     <div class="work-content">
            //                         <h3>Survival Shooter 教學專案作品</h3>
            //                         <p>
            //                             遊戲試玩：
            //                             <br>
            //                             <a href="https://yuchiang-chang.github.io/Survival-Shooter-WebGL">
            //                                 Survival Shooter 教學專案作品
            //                             </a>
            //                             <br>
            //                             <br>
            //                             在此教學中，學習到了如何製作UI，利用相機投射到網格並將值傳到角色身上使角色旋轉，並編寫生命值與傷害等功能，並在遊戲中生成敵人與用Audio Mixer調整音效。
            //                             <br>
            //                             <br>
            //                             也試著自己利用它提供的資源寫暫停的功能。
            //                             <br>
            //                             <br>
            //                             製作工具：Unity
            //                             <br>
            //                             <br>
            //                             <a href="https://learn.unity.com/project/survival-shooter-tutorial">教學連結</a>
            //                         </p>
            //                     </div>
            //                 </div>
            //             </div>
            //         </div>
            //         <div id="model">
            //             <h2>模型作品</h2>
            //             <hr class="h1-hr">
            //             <div class="portfolio">
            //                 <div class="work">
            //                     <img class="hover-shadow cursor modal-open" id="blenderLion" src="./images/blender lion demo.png" alt="blender作品">
            //                     <div class="work-content">
            //                         <p>製作工具：blender</p> 
            //                     </div>
            //                 </div>
            //                 <div class="work">
            //                     <img class="hover-shadow cursor modal-open" id="smallRoom" src="./images/smallroom.jpg" alt="smallroom">
            //                     <div class="work-content">
            //                         <p>製作工具：blender</p>
            //                     </div>
            //                 </div>
            //             </div>
            //         </div>
            //         <div id="web">
            //             <h2>網頁作品</h2>
            //             <hr class="h1-hr">
            //             <div class="portfolio">
            //                 <div class="work">
            //                     <p>
            //                         此作品集網站就是我的作品之一，因為想學會如何撰寫網頁，所以就寫了自己的作品集網站。
            //                         <br>
            //                         並且使用webpack進行打包，與使用GitHub Pages發布。
            //                     </p>
            //                 </div>
            //             </div>
            //         </div>
            //     </div>
            // </div>
            // <div id="myModal" class="modal">
            //     <span id="modal-close" class="cursor">&times;</span>
            //     <div class="modal-content">
            //         <div id="modalSlide">
            //             <div class="arrow">
            //                 <a class="prev">&#10094;</a>
            //                 <a class="next">&#10095;</a>
            //             </div>
            //             <div id="veg-modal">
            //                 <div class="mySlides">
            //                     <img src="./images/VEG Demo01-Chiang-compressed.webp" alt="">
            //                 </div>
            //                 <div class="mySlides">
            //                     <img src="./images/VEG Fireworks-Lite-675p-compressed.webp" alt="">
            //                 </div>
            //                 <div class="column">
            //                     <img class="demo cursor veg1" src="./images/VEG Demo01-Chiang-compressed.webp" alt="VEG Demo01">
            //                     <img class="demo cursor veg2" src="./images/VEG Fireworks-Lite-675p-compressed.webp" alt="VEG Fireworks">
            //                 </div>
            //             </div>
            //         </div>
            //         <img id="modalImg">
            //     </div>
            // </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}