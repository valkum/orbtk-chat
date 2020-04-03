use orbtk::prelude::*;
use serde::{Serialize, Deserialize};

mod left_most_pane;
mod left_pane;
mod right_pane;
mod main_view;
mod message_panel;
mod message_composer;
mod room_view;
mod room_header;
mod room_list;
mod custom_widgets;

use custom_widgets::*;
use left_most_pane::*;
use left_pane::*;
use right_pane::*;
use main_view::*;
use message_panel::*;
use message_composer::*;
use room_view::*;
use room_header::*;
use room_list::*;

use orbtk::theme::DEFAULT_THEME_CSS;

static CSS_EXT: &'static str = include_str!("../res/orbtk-chat.css");

fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(CSS_EXT)
        .build()
}



fn main() {
  Application::from_name("OrbTK-Chat")
        .window(|ctx| {
            Window::create()
                .title("OrbTK Chat Wireframe")
                .size(900.0, 600.0)
                .resizeable(true)
                .borderless(true)
                .theme(get_theme())
                .child(
                    MainView::create().build(ctx)
                )
                .build(ctx)
        })
        .run();

}
