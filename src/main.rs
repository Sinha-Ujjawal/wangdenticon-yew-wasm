use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

mod simple_components;
mod wangdenticon;
mod wangdenticon_app;

const MIN_GRID_SIZE: u8 = 2;
const MAX_GRID_SIZE: u8 = 10;
const SIZE: usize = 255;

#[derive(Clone, Routable, PartialEq, Eq)]
enum Route {
    #[at("/wangdenticon-yew-wasm/")]
    Home,
    #[at("/wangdenticon-yew-wasm/generate/:name")]
    GenerateImageNameOnly { name: String },
    #[at("/wangdenticon-yew-wasm/generate/:name/:gridsize")]
    GenerateImageNameAndGridsize { name: String, gridsize: u8 },
    #[at("/wangdenticon-yew-wasm/generate/:name/:gridsize/:invert")]
    GenerateImageAll {
        name: String,
        gridsize: u8,
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
                <wangdenticon_app::App
                    min_grid_size={MIN_GRID_SIZE}
                    max_grid_size={MAX_GRID_SIZE}
                    size={SIZE}
                />
            }
        }
        Route::GenerateImageNameOnly { name } => {
            let hex_list = md5::compute(name).0;
            let fgcolor = [hex_list[0], hex_list[1], hex_list[2]];
            let bgcolor = [0; 3];
            wangdenticon_app::render_wangdenticon_image(
                &hex_list,
                &fgcolor,
                &bgcolor,
                MIN_GRID_SIZE,
                SIZE,
            )
        }
        Route::GenerateImageNameAndGridsize { name, gridsize } => {
            let hex_list = md5::compute(name).0;
            let fgcolor = [hex_list[0], hex_list[1], hex_list[2]];
            let bgcolor = [0; 3];
            wangdenticon_app::render_wangdenticon_image(
                &hex_list, &fgcolor, &bgcolor, *gridsize, SIZE,
            )
        }
        Route::GenerateImageAll {
            name,
            gridsize,
            invert,
        } => {
            let hex_list = md5::compute(name).0;
            let mut fgcolor = [hex_list[0], hex_list[1], hex_list[2]];
            let mut bgcolor = [0; 3];
            if *invert {
                (fgcolor, bgcolor) = (bgcolor, fgcolor);
            }
            wangdenticon_app::render_wangdenticon_image(
                &hex_list, &fgcolor, &bgcolor, *gridsize, SIZE,
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
