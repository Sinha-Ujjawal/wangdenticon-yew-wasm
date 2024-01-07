use crate::{simple_components, wangdenticon_as_img::render_wangdenticon_image};
use yew::prelude::{html, Component, Context, Html, Properties};

pub struct App {
    name: String,
    hex_list: [u8; 16],
    gridsize: u8,
    min_grid_size: u8,
    max_grid_size: u8,
    invert: bool,
    size: usize,
}

impl App {
    fn render_image(&self) -> Html {
        render_wangdenticon_image(&self.hex_list, self.gridsize, self.invert, self.size)
    }
}

pub enum Msg {
    SetGridSize(u8),
    ToggleInvert,
    SetName(String),
}

#[derive(PartialEq, Properties, Eq)]
pub struct Props {
    pub min_grid_size: u8,
    pub max_grid_size: u8,
    pub size: usize,
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let Props {
            min_grid_size,
            max_grid_size,
            size,
        } = ctx.props();
        Self {
            name: "".to_owned(),
            hex_list: md5::compute("").0,
            gridsize: *min_grid_size,
            min_grid_size: *min_grid_size,
            max_grid_size: *max_grid_size,
            invert: false,
            size: *size,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetGridSize(gridsize) => {
                self.gridsize = gridsize;
                true
            }
            Msg::ToggleInvert => {
                self.invert = !self.invert;
                true
            }
            Msg::SetName(name) => {
                self.name = name;
                self.hex_list = md5::compute(&self.name).0;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            {self.render_image()}

            {simple_components::draw_textbox(
                ctx,
                "Name",
                self.name.clone(),
                "Enter your name here...",
                Msg::SetName,
            )}

            {simple_components::draw_slider(
                ctx,
                "Grid Size",
                self.gridsize,
                self.min_grid_size,
                self.max_grid_size,
                0,
                Msg::SetGridSize
            )}

            {simple_components::draw_checkbox(
                ctx,
                "Invert?",
                self.invert,
                || Msg::ToggleInvert,
            )}
            </>
        }
    }
}
