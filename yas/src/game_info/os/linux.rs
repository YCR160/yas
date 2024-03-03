use crate::common::positioning::Rect;
use crate::game_info::{GameInfo, Resolution, UI};
use anyhow::Result;

pub fn get_game_info(_window_names: &[&str]) -> Result<GameInfo> {
    let window_id = unsafe {
        String::from_utf8_unchecked(
            std::process::Command::new("sh")
                .arg("-c")
                .arg(r#" xwininfo|grep "Window id"|cut -d " " -f 4 "#)
                .output()
                .unwrap()
                .stdout,
        )
    };
    let window_id = window_id.trim_end_matches("\n");

    let position_size = unsafe {
        String::from_utf8_unchecked(
            std::process::Command::new("sh")
                .arg("-c")
                .arg(&format!(r#" xwininfo -id {window_id}|cut -f 2 -d :|tr -cd "0-9\n"|grep -v "^$"|sed -n "1,2p;5,6p" "#))
                .output()
                .unwrap()
                .stdout,
        )
    };

    let mut info = position_size.split("\n");

    let left: f64 = info.next().unwrap().parse().unwrap();
    let top: f64 = info.next().unwrap().parse().unwrap();
    let width: f64 = info.next().unwrap().parse().unwrap();
    let height: f64 = info.next().unwrap().parse().unwrap();

    let rect = Rect::new(left, top, width, height);

    Ok(GameInfo {
        window: rect,
        resolution: Resolution::new(rect.size()),
        is_cloud: false,
        ui: UI::Desktop,
    })
}
