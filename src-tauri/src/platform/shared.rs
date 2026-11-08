use std::time::Duration;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use tauri::Window;

use crate::data::Point;

#[derive(Debug)]
pub enum KeyMode {
    Press,
    Hold,
    Release,
}

// For visual recognition: Avoids mouse clicks outside the window by ignoring monster names that are too close to the bottom of the GUI
pub const IGNORE_AREA_BOTTOM: u32 = 110;
//>100 <230 where we get the red announcement for already targetted mob
/// Get the native window id.
pub fn get_window_id(window: &Window) -> Option<u64> {
    #[allow(unused_variables)]
    match window.raw_window_handle() {
        RawWindowHandle::Xlib(handle) => Some(handle.window as u64),
        RawWindowHandle::Win32(handle) => Some(handle.hwnd as u64),
        RawWindowHandle::AppKit(handle) => {
            #[cfg(target_os = "macos")]
            unsafe {
                use std::ffi::c_void;
                let ns_window_ptr = handle.ns_window as *const c_void;
                libscreenshot::platform::macos::macos_helper::ns_window_to_window_id(ns_window_ptr)
                    .map(|id| id as u64)
            }
            #[cfg(not(target_os = "macos"))]
            unreachable!()
        }
        _ => Some(0_u64),
    }
}

pub fn eval_send_key(window: &Window, key: &str, mode: KeyMode) {
    match mode {
        KeyMode::Press => {
            drop(window.eval(format!("keyboardEvent('press', '{0}');", key).as_str()))
        }

        KeyMode::Hold => drop(window.eval(format!("keyboardEvent('hold', '{0}');", key).as_str())),

        KeyMode::Release => {
            drop(window.eval(format!("keyboardEvent('release', '{0}');", key).as_str()))
        }
    }
}

pub fn send_slot_eval(window: &Window, slot_bar_index: usize, k: usize) {
    drop(window.eval(format!("sendSlot({0}, {1})", slot_bar_index, k).as_str()))
}

pub fn eval_mob_click(window: &Window, pos: Point) {
    // Fresh-7 (display-scale fix): `pos` comes from the screenshot, so it is
    // in device pixels, but the click is dispatched as a webview mouse
    // event, whose coordinates are in logical (CSS) pixels. On a display
    // scaled above 100% (e.g. 125%) the two differ by the scale factor, so
    // an unconverted click lands too far down and to the right — usually on
    // empty ground, which made lock-on slow and inconsistent. Divide by the
    // window's scale factor to convert. At 100% scaling the factor is 1.0
    // and the click is unchanged; if the factor cannot be read, fall back
    // to the unconverted position (the old behavior).
    let scale_factor = window.scale_factor().unwrap_or(1.0);
    let (x, y) = if scale_factor > 0.0 && scale_factor != 1.0 {
        (
            (pos.x as f64 / scale_factor).round() as u32,
            (pos.y as f64 / scale_factor).round() as u32,
        )
    } else {
        (pos.x, pos.y)
    };
    drop(
        window.eval(
            format!(
                "mouseEvent('moveClick', {0}, {1}, {{checkMob: true}});",
                x, y
            )
            .as_str(),
        ),
    );
}

pub fn eval_simple_click(window: &Window, pos: Point) {
    drop(window.eval(format!("mouseEvent('moveClick', {0}, {1});", pos.x, pos.y).as_str()));
}

pub fn eval_send_message(window: &Window, text: &str) {
    drop(window.eval(format!("setInputChat({0})", text).as_str()));
}
