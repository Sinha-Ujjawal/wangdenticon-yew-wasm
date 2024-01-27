use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

mod simple_components;
mod wangdenticon;
mod wangdenticon_app;

const DEFAULT_SIZE: u32 = 255;

#[derive(Clone, Routable, PartialEq, Eq)]
enum Route {
    #[at("/wangdenticon-yew-wasm/")]
    Home,
    #[at("/wangdenticon-yew-wasm/generate/:name")]
    GenerateImageNameOnly { name: String },
    #[at("/wangdenticon-yew-wasm/generate/:name/:gridsize")]
    GenerateImageNameAndGridsize { name: String, gridsize: u32 },
    #[at("/wangdenticon-yew-wasm/generate/:name/:gridsize/:invert")]
    GenerateImageAll {
        name: String,
        gridsize: u32,
        invert: bool,
    },
    #[not_found]
    #[at("/wangdenticon-yew-wasm/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! {
                <wangdenticon_app::App size={DEFAULT_SIZE}/>
            }
        }
        Route::GenerateImageNameOnly { name } => {
            let hex_list = md5::compute(name).0;
            let fgcolor = wangdenticon_app::fgcolor_from_hex_list(&hex_list);
            let bgcolor = [0; 3];
            wangdenticon_app::render_wangdenticon_image(
                &hex_list,
                &fgcolor,
                &bgcolor,
                2,
                DEFAULT_SIZE,
            )
        }
        Route::GenerateImageNameAndGridsize { name, gridsize } => {
            let hex_list = md5::compute(name).0;
            let fgcolor = wangdenticon_app::fgcolor_from_hex_list(&hex_list);
            let bgcolor = [0; 3];
            wangdenticon_app::render_wangdenticon_image(
                &hex_list,
                &fgcolor,
                &bgcolor,
                *gridsize,
                DEFAULT_SIZE,
            )
        }
        Route::GenerateImageAll {
            name,
            gridsize,
            invert,
        } => {
            let hex_list = md5::compute(name).0;
            let mut fgcolor = wangdenticon_app::fgcolor_from_hex_list(&hex_list);
            let mut bgcolor = [0; 3];
            if *invert {
                (fgcolor, bgcolor) = (bgcolor, fgcolor);
            }
            wangdenticon_app::render_wangdenticon_image(
                &hex_list,
                &fgcolor,
                &bgcolor,
                *gridsize,
                DEFAULT_SIZE,
            )
        }
        Route::NotFound => {
            html! {
                <h1>
                    {"Not found: 404"}
                </h1>
            }
        }
    }
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
