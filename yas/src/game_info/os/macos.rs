use crate::game_info::{GameInfo, Platform, ResolutionFamily, UI};
use crate::utils;
use crate::positioning::{Rect, Size};
use anyhow::Result;

pub fn get_game_info(_window_names: &[&str]) -> Result<GameInfo> {
    let (pid, ui) = utils::get_pid_and_ui();

    let (rect, _window_title) = unsafe { utils::find_window_by_pid(pid).unwrap() };

    Ok(GameInfo {
        window: rect,
        resolution_family: ResolutionFamily::new(rect.to_rect_usize().size()).unwrap(),
        is_cloud: false,
        ui,
        platform: Platform::MacOS
    })
}
