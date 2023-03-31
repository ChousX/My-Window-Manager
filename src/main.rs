mod consts;
mod hooks;
use consts::*;
use hooks::Feh;
use penrose::{
    builtin::{
        actions::{exit, log_current_state, modify_with, send_layout_message, spawn},
        layout::{
            messages::{ExpandMain, IncMain, ShrinkMain},
            transformers::{Gaps, ReflectHorizontal},
            MainAndStack,
        },
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        layout::LayoutStack,
        Config, WindowManager,
    },
    map, stack,
    x11rb::RustConn,
    Result,
};

use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
            map_keys: |k: &str| k.to_string();

            "M-j" => modify_with(|cs| cs.focus_down()),
            "M-k" => modify_with(|cs| cs.focus_up()),
            "M-S-j" => modify_with(|cs| cs.swap_down()),
            "M-S-k" => modify_with(|cs| cs.swap_up()),
            "M-S-q" => modify_with(|cs| cs.kill_focused()),
            "M-Tab" => modify_with(|cs| cs.toggle_tag()),
            "M-bracketright" => modify_with(|cs| cs.next_screen()),
            "M-bracketleft" => modify_with(|cs| cs.previous_screen()),
            "M-grave" => modify_with(|cs| cs.next_layout()),
            "M-S-grave" => modify_with(|cs| cs.previous_layout()),
            "M-S-Up" => send_layout_message(|| IncMain(1)),
            "M-S-Down" => send_layout_message(|| IncMain(-1)),
            "M-S-Right" => send_layout_message(|| ExpandMain),
            "M-S-Left" => send_layout_message(|| ShrinkMain),
    //        "M-semicolon" => spawn("dmenu_run"),
            "M-e" => spawn("emacsclient --create-frame --alternate-editor nvim"),
            "M-d" => spawn("rofi -show combi"),
            "M-f" => spawn("firefox"),
            "f1" => spawn("firefox"),
            "M-S-Return" => spawn("alacritty"),
            "M-A-Escape" => exit(),
        };

    for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

fn layouts() -> LayoutStack {
    let max_main = 1;
    let ratio = 0.6;
    let ratio_step = 0.1;
    let outer_px = 5;
    let inner_px = 5;

    stack!(
        MainAndStack::side(max_main, ratio, ratio_step),
        ReflectHorizontal::wrap(MainAndStack::side(max_main, ratio, ratio_step)),
        MainAndStack::bottom(max_main, ratio, ratio_step)
    )
    .map(|layout| Gaps::wrap(layout, outer_px, inner_px))
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let config = Config {
        default_layouts: layouts(),
        startup_hook: Some(Box::new(Feh::Center)),
        ..Config::default()
    };

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let wm = WindowManager::new(config, key_bindings, HashMap::new(), conn)?;

    wm.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let res = parse_keybindings_with_xmodmap(raw_key_bindings());

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
