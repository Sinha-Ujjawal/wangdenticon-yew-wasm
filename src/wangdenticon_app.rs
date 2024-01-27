use crate::{simple_components, wangdenticon};
use wangdenticon::Wangdenticon;
use yew::prelude::{html, Component, Context, Html, Properties};

pub struct App {
    name: String,
    hex_list: [u8; 16],
    fgcolor: [u8; 3],
    bgcolor: [u8; 3],
    gridsize: u8,
    min_grid_size: u8,
    max_grid_size: u8,
    size: usize,
}

pub fn render_wangdenticon_image(
    hex_list: &[u8; 16],
    fgcolor: &[u8; 3],
    bgcolor: &[u8; 3],
    gridsize: u8,
    size: usize,
) -> Html {
    let image = Wangdenticon::new(gridsize).generate_as_png(hex_list, fgcolor, bgcolor, size);
    let image_base64_encoded = base64::encode(image);
    html! {
        <img alt="Wangdenticon" src={format!("data:image/png;base64,{}", image_base64_encoded)}/>
    }
}

impl App {
    fn render_image(&self) -> Html {
        render_wangdenticon_image(
            &self.hex_list,
            &self.fgcolor,
            &self.bgcolor,
            self.gridsize,
            self.size,
        )
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
        let hex_list = md5::compute("").0;
        let fgcolor = [hex_list[0], hex_list[1], hex_list[2]];
        let bgcolor = [0; 3];
        Self {
            name: "".to_owned(),
            hex_list,
            fgcolor,
            bgcolor,
            gridsize: *min_grid_size,
            min_grid_size: *min_grid_size,
            max_grid_size: *max_grid_size,
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
                (self.fgcolor, self.bgcolor) = (self.bgcolor, self.fgcolor);
                true
            }
            Msg::SetName(name) => {
                self.name = name;
                self.hex_list = md5::compute(&self.name).0;
                self.fgcolor = [self.hex_list[0], self.hex_list[1], self.hex_list[2]];
                self.bgcolor = [0; 3];
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

            {simple_components::draw_button(
                ctx,
                "Invert",
                || Msg::ToggleInvert,
            )}
            </>
        }
    }
}
