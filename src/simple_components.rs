use log::info;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Event, Html, InputEvent, KeyboardEvent};

#[allow(clippy::too_many_arguments)]
pub fn draw_slider<
    A: 'static + std::fmt::Display + std::str::FromStr + Copy,
    C: Component<Message = Msg>,
    Msg: 'static,
>(
    ctx: &Context<C>,
    label: &str,
    label_value: A,
    min: A,
    max: A,
    default: A,
    mk_event: fn(A) -> Msg,
    disabled: bool,
) -> Html {
    html! {
        <div>
            <div>
                { format!("{}: {}", label, label_value) }
            </div>
            <input
                type="range"
                oninput={ctx.link().batch_callback(move |e: InputEvent| {
                    let target = e.target();
                    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                    input.map(move |input| mk_event(input.value().parse().unwrap_or(default)))
                })}
                value={format!("{}", label_value)}
                min={format!("{}", min)}
                max={format!("{}", max)}
                disabled={disabled}
            />
        </div>
    }
}

pub fn draw_button<C: Component<Message = Msg>, Msg: 'static>(
    ctx: &Context<C>,
    label: &str,
    mk_event: fn() -> Msg,
    disabled: bool,
) -> Html {
    html! {
        <div>
            <button
                type="checkbox"
                onclick={ctx.link().callback(move |_| mk_event())}
                disabled={disabled}
            >
            {label}
            </button>
        </div>
    }
}

pub fn draw_textbox<C: Component<Message = Msg>, Msg: 'static>(
    ctx: &Context<C>,
    label: &str,
    label_value: String,
    placeholder: &'static str,
    mk_event: fn(String) -> Msg,
    disabled: bool,
) -> Html {
    let link = ctx.link();
    let handle_event = move |e: Event| {
        let target = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        input.map(move |input| mk_event(input.value()))
    };
    let onchange = link.batch_callback(handle_event);
    let onkeyup = link.batch_callback(move |e: KeyboardEvent| handle_event(e.into()));
    html! {
        <div>
            <div> {label} </div>
            <input
                type="textbox"
                placeholder={placeholder}
                value={label_value}
                onchange={onchange}
                onkeyup={onkeyup}
                disabled={disabled}
            />
        </div>
    }
}

pub fn draw_checkbox<C: Component<Message = Msg>, Msg: 'static>(
    ctx: &Context<C>,
    label: &str,
    is_checked: bool,
    mk_event: fn() -> Msg,
    disabled: bool,
) -> Html {
    html! {
        <div>
            <div> {label} </div>
            <input
                type="checkbox"
                oninput={ctx.link().callback(move |_| mk_event())}
                checked={is_checked}
                disabled={disabled}
            />
        </div>
    }
}

fn rgb_to_hex(color: &[u8; 3]) -> String {
    let [r, g, b] = color;
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

fn hex_to_rgb(hex_color: &str) -> Result<[u8; 3], core::num::ParseIntError> {
    let hex_color = hex_color.strip_prefix('#').unwrap_or(hex_color);
    let hex_value = usize::from_str_radix(hex_color, 16)?;
    let r = ((hex_value >> 16) & 0xFF) as u8; // Extract the RR byte
    let g = ((hex_value >> 8) & 0xFF) as u8; // Extract the GG byte
    let b = ((hex_value) & 0xFF) as u8; // Extract the BB byte
    Ok([r, g, b])
}

pub fn draw_color_chooser<C: Component<Message = Msg>, Msg: 'static>(
    ctx: &Context<C>,
    label: &str,
    color: &[u8; 3],
    mk_event: fn([u8; 3]) -> Msg,
    disabled: bool,
) -> Html {
    let link = ctx.link();
    let handle_event = move |e: Event| {
        let target = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        input.map(move |input| {
            let input_value = input.value();
            let color = hex_to_rgb(&input_value).unwrap_or_else(|err| {
                info!(
                    "Could not parse the string {input_value}: {err}",
                    err = err.to_string()
                );
                [0; 3]
            });
            mk_event(color)
        })
    };
    let onchange = link.batch_callback(handle_event);
    html! {
        <div>
            <div> {label} </div>
            <input
                type="color"
                value={rgb_to_hex(color)}
                onchange={onchange}
                disabled={disabled}
            />
        </div>
    }
}
