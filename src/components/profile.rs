use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <div id="about" class="profile">
            // <a href="javascript:void(0)" id="close-about-btn">{Html::from_html_unchecked(AttrValue::from("&Cross;"))}</a>
            <a href="javascript:void(0)" class="profile-close-button">
                <span class="material-symbols-rounded">{"close"}</span>
            </a>
            // <span class="material-icons md-dark">{"face"}</span>
            // <div id="about-content">
            <div>
                <h2 style="text-align: center;">{"關於我"}</h2>
                <p>{"雲端建築師兼網路巫師，可自主學習替公司建置雲端、網站、設備等，並正在提升Rust魔法中~"}</p>
                <p>
                    {Html::from_html_unchecked(AttrValue::from("&ensp;&ensp;&ensp;&ensp;"))}{"我是張佑強，歡迎來到我的作品集網站，我是個有興趣就會開始自學的人。在大學期間，接觸了PLC（可程式化邏輯控制器），並自己嘗試編寫教授所教的以外的寫法，知道了自己適合撰寫程式，也取得了機電整合的證照。後續又因為接觸了3D，漸漸的對製作遊戲產生了興趣，也開始自學如何製作遊戲。"}
                </p>
                <p>{"學歷：華梵大學機電工程學系學士"}</p>
                <p>{"證照：機電整合丙級"}</p>
                <br/>
                <h2 style="text-align: center;">{"聯絡我"}</h2>
                <p>{Html::from_html_unchecked(AttrValue::from("&#9993;"))}{" 信箱："}</p>
                // <p><span class="material-symbols-rounded">{"mail"}</span>{"&#9993; 信箱："}</p>
                <a href="mailto:yuchiang.chang.taiwan@gmail.com">{"yuchiang.chang.taiwan@gmail.com"}</a>
            </div>
        </div>
    }
}