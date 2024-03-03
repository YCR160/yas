use crate::game_info::{GameInfo, Resolution};
use crate::utils::{find_window_by_pid, get_pid_and_ui};
use anyhow::Result;

pub fn get_game_info(_window_names: &[&str]) -> Result<GameInfo> {
    let (pid, ui) = get_pid_and_ui();

    let (rect, window_title) = unsafe { find_window_by_pid(pid).unwrap() };

    Ok(GameInfo {
        window: rect,
        resolution: Resolution::new(rect.size()),
        is_cloud: false,
        ui,
    })
}
