use crate::wangdenticon;
use yew::{html, Html};

pub fn render_wangdenticon_image(name: &str, gridsize: u8, invert: bool, size: usize) -> Html {
    let image_jpeg = wangdenticon::Wangdenticon::new(gridsize, invert).generate_as_png(name, size);
    let image_base64_encoded = base64::encode(&image_jpeg);
    html! {
        <img alt="Wangdenticon" src={format!("data:image/png;base64,{}", image_base64_encoded)}/>
    }
}
