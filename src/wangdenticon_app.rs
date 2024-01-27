use crate::{simple_components, wangdenticon};
use wangdenticon::Wangdenticon;
use yew::prelude::{html, Component, Context, Html, Properties};
pub struct App {
    name: String,
    hex_list: [u8; 16],
    fgcolor: [u8; 3],
    bgcolor: [u8; 3],
    override_colors: bool,
    gridsize: u32,
    size: u32,
}

pub fn fgcolor_from_hex_list(hex_list: &[u8; 16]) -> [u8; 3] {
    [hex_list[0], hex_list[1], hex_list[2]]
}

pub fn render_wangdenticon_image(
    hex_list: &[u8; 16],
    fgcolor: &[u8; 3],
    bgcolor: &[u8; 3],
    gridsize: u32,
    size: u32,
) -> Html {
    let image = Wangdenticon::new(gridsize).generate_as_png(hex_list, fgcolor, bgcolor, size);
    let image_base64_encoded = base64::encode(image);
    html! {
        <img alt="Wangdenticon" src={format!("data:image/png;base64,{}", image_base64_encoded)}/>
    }
}

impl App {
    fn set_default_colors(&mut self) {
        self.fgcolor = fgcolor_from_hex_list(&self.hex_list);
        self.bgcolor = [0; 3];
    }

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
    SetGridSize(u32),
    Invert,
    SetName(String),
    SetFGColor([u8; 3]),
    SetBGColor([u8; 3]),
    ToggleOverrideColors,
}

#[derive(PartialEq, Properties, Eq)]
pub struct Props {
    pub size: u32,
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let Props { size } = ctx.props();
        let hex_list = md5::compute("").0;
        let fgcolor = fgcolor_from_hex_list(&hex_list);
        let bgcolor = [0; 3];
        Self {
            name: "".to_owned(),
            hex_list,
            fgcolor,
            bgcolor,
            override_colors: false,
            gridsize: 2,
            size: *size,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetGridSize(gridsize) => {
                self.gridsize = gridsize;
                true
            }
            Msg::Invert => {
                if self.override_colors {
                    (self.fgcolor, self.bgcolor) = (self.bgcolor, self.fgcolor);
                }
                true
            }
            Msg::SetName(name) => {
                self.name = name;
                self.hex_list = md5::compute(&self.name).0;
                self.override_colors = false;
                self.set_default_colors();
                true
            }
            Msg::SetFGColor(fgcolor) => {
                if self.override_colors {
                    self.fgcolor = fgcolor;
                }
                true
            }
            Msg::SetBGColor(bgcolor) => {
                if self.override_colors {
                    self.bgcolor = bgcolor;
                }
                true
            }
            Msg::ToggleOverrideColors => {
                self.override_colors = !self.override_colors;
                if !self.override_colors {
                    self.set_default_colors();
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            {self.render_image()}

            <div style="padding: 10px">
            {simple_components::draw_textbox(
                ctx,
                "Name",
                self.name.clone(),
                "Enter your name here...",
                Msg::SetName,
                false,
            )}
            </div>

            <div style="padding: 10px">
            {simple_components::draw_slider(
                ctx,
                "Grid Size",
                self.gridsize,
                2,
                1000,
                0,
                Msg::SetGridSize,
                false,
            )}
            </div>

            <div style="padding: 10px">
            {simple_components::draw_checkbox(
                ctx,
                "Override colors?",
                self.override_colors,
                || Msg::ToggleOverrideColors,
                false,
            )}
            {simple_components::draw_color_chooser(
                ctx,
                "fgcolor",
                &self.fgcolor,
                Msg::SetFGColor,
                !self.override_colors,
            )}

            {simple_components::draw_color_chooser(
                ctx,
                "bgcolor",
                &self.bgcolor,
                Msg::SetBGColor,
                !self.override_colors,
            )}

            <div style="padding-top: 5px">
            {simple_components::draw_button(
                ctx,
                "Invert",
                || Msg::Invert,
                !self.override_colors,
            )}
            </div>
            </div>
            </>
        }
    }
}
