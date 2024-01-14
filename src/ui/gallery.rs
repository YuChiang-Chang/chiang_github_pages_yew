use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub images: Vec<(String, String)>,
}

#[function_component(Gallery)]
pub fn gallery(props: &Props) -> Html {
    html! {
        <div class="gallery">
            { for props.images.iter().map(|(img_src, alt)| {
                html! {
                    <div class="gallery__item">
                        <img src={img_src.clone()} alt={alt.clone()} />
                    </div>
                }
            }) }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props1 {
    #[prop_or_default]
    pub img_src: String,
    #[prop_or_default]
    pub alt: String,
}

#[function_component(Gallery2img)]
pub fn gallery(_props: &Props1) -> Html {
    html! {
        <div class="gallery">
            <img class="gallery__feature" src="./static/images/VEG Demo01-Chiang-compressed.webp" alt="VEG Demo01-Chiang" />
            <div class="gallery__item">
                <img src="./static/images/VEG Demo01-Chiang-compressed.webp" alt="VEG Demo01-Chiang" />
                <img src="./static/images/VEG Fireworks-Lite-675p-compressed.webp" alt="VEG Fireworks" />
            </div>
        </div>
    }
}
