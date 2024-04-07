use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <div id="profile">
            // <a href="javascript:void(0)" id="close-about-btn">{Html::from_html_unchecked(AttrValue::from("&Cross;"))}</a>
            // <a href="javascript:void(0)" class="profile-close-button">
            //     <span class="material-symbols-rounded">{"close"}</span>
            // </a>
            // <span class="material-icons md-dark">{"face"}</span>
            // <div id="about-content">
            // <div>
                <h2 style="text-align: center;">{"關於我"}</h2>
                // <p>{"雲端建築師兼網路巫師，可自主學習替公司建置雲端、網站、設備等，並正在提升Rust魔法中~"}</p>
                <p class="pre-wrap">
                    // {Html::from_html_unchecked(AttrValue::from("&ensp;&ensp;&ensp;&ensp;"))}{"我是張佑強，歡迎來到我的作品集網站，我是個有興趣就會開始自學的人。在大學期間，接觸了PLC（可程式化邏輯控制器），並自己嘗試編寫教授所教的以外的寫法，知道了自己適合撰寫程式，也取得了機電整合的證照。後續又因為接觸了3D，漸漸的對製作遊戲產生了興趣，也開始自學如何製作遊戲。"}
                    {"    我是張佑強，歡迎來到我的作品集網站"}
                </p>
                <p>{"學歷：華梵大學機電工程學系學士"}</p>
                <p>{"證照：機電整合丙級"}</p>
                <br/>
                <h2 style="text-align: center;">{"聯絡我"}</h2>
                <p>{Html::from_html_unchecked(AttrValue::from("&#9993;"))}{" 信箱："}</p>
                // <p><span class="material-symbols-rounded">{"mail"}</span>{"&#9993; 信箱："}</p>
                <a href="mailto:yuchiang.chang.taiwan@gmail.com">{"yuchiang.chang.taiwan@gmail.com"}</a>
            // </div>
        </div>
    }
}

// #[function_component(About)]
// pub fn about() -> Html {
    // let about_width = "20%";
    // let adaptive_width = 1024;
    // let focus_color = "#222222";

    // let about = yew::use_node_ref(|| {
    //     html! {
    //         <div id="about" class="about-container">
    //             <div class="about-content">
    //                 </div>
    //         </div>
    //     }
    // });

    // let is_open = yew::use_state(|| false);

    // 滑鼠懸停效果
    // yew::use_effect_with(|| {
    //     let about_me_btn = yew::use_ref(|| document.getElementById("aboutme-btn"));

    //     fn hover_handler(ref mut about_me_btn: &mut HtmlElement) {
    //         about_me_btn.style.backgroundColor = focus_color;
    //     }

    //     fn leave_handler(ref mut about_me_btn: &mut HtmlElement) {
    //         about_me_btn.style.backgroundColor = about.get().style.backgroundColor.unwrap();
    //     }

    //     about_me_btn.add_event_listener("mouseover", hover_handler);
    //     about_me_btn.add_event_listener("mouseout", leave_handler);

    //     move || ()
    // });

    // 開啟側邊欄
    // fn open_about() {
    //     about.get().style.left = "calc(50% + 30%)";
    //     about.get().style.width = about_width;
    //     about.get().style.display = "block";
    //     about.get().parent().unwrap().style.marginRight = about_width;
    //     is_open.set(true);
    // }

    // 關閉側邊欄
    // fn close_about() {
    //     about.get().style.left = "100%";
    //     about.get().style.width = "0";
    //     about.get().style.display = "none";
    //     about.get().parent().unwrap().style.marginRight = "0";
    //     is_open.set(false);
    // }

    // 點擊事件
    // fn handle_click(event: yew::MouseEvent) {
    //     if window.innerWidth > adaptive_width {
    //         if is_open.get() {
    //             close_about();
    //         } else {
    //             open_about();
    //         }
    //     } else {
    //         close_about();
    //     }
    // }

    // html! {
    //     <div class="about-container">
    //         <button id="aboutme-btn" class="about-btn" onclick=handle_click>關於我</button>
    //         {
    //             if is_open.get() {
    //                 about.get()
    //             } else {
    //                 html!{}
    //             }
    //         }
    //     </div>
    // }
// }

