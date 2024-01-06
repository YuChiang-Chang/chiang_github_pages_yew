use yew::prelude::*;
// use stylist::yew::styled_component;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="nav">
            <ul>
                <li>
                    <a id="aboutme-btn" href="#about" class="nav-btn">
                        {"關於我"}
                    </a> 
                </li>
                <li>
                    <a href="#game" class="nav-btn">
                        {"遊戲作品"}
                    </a>
                </li>
                <li>
                    <a href="#model" class="nav-btn">
                        {"模型作品"}
                    </a>
                </li>
                <li>
                    <a href="#web" class="nav-btn">
                        {"網頁作品"}
                    </a>
                </li>
                // <li>
                // </li>
            </ul>
            // <style>"style.css"</style>
        </nav>
    }
}