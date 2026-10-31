//! Qt 6 shell. Widgets are Qt; playback, search and the ten modes stay in Rust.
#![cfg_attr(windows, windows_subsystem = "windows")]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::thread;
use std::time::Duration;

use mineral_gui_core::{BridgeBackend, Session, autotest_step, is_autotest, session_path};
use serde_json::Value;

unsafe extern "C" {
    fn qt_init();
    fn qt_process() -> c_int;
    fn qt_set_snapshot(json: *const c_char);
    fn qt_tick_playback(playing: c_int, pos_ms: i64, dur_ms: i64, peaks: *const f32, peaks_len: c_int);
    fn qt_poll_action(buf: *mut c_char, cap: c_int) -> c_int;
    fn qt_frame_interval_us() -> c_int;
}

fn main() {
    let autotest = is_autotest();
    unsafe { qt_init() };
    let mut session = Session::open(BridgeBackend::launch(), session_path());
    if !autotest {
        session.workspace = "classic".to_owned();
        session.input_mode = "pointer".to_owned();
        session.auto_adapt = false;
        session.cover_mode = "list".to_owned();
    }
    let mut buffer = vec![0u8; 16384];
    let mut last_gen = u64::MAX;
    let mut last_pos_ms = u64::MAX;
    let mut step = 0usize;
    loop {
        if unsafe { qt_process() } == 0 {
            break;
        }
        session.tick();
        if autotest {
            step += 1;
            if let Some(action) = autotest_step(step) {
                session.dispatch_value(&action);
            }
        }
        loop {
            let read = unsafe { qt_poll_action(buffer.as_mut_ptr().cast::<c_char>(), buffer.len() as c_int) };
            if read <= 0 {
                break;
            }
            let bytes = &buffer[..read as usize];
            if let Ok(text) = std::str::from_utf8(bytes) {
                if let Ok(value) = serde_json::from_str::<Value>(text) {
                    session.dispatch_value(&value);
                }
            }
        }
        if session.generation != last_gen {
            last_gen = session.generation;
            last_pos_ms = session.position_ms;
            if let Ok(json) = serde_json::to_string(&session.snapshot()) {
                if let Ok(text) = CString::new(json) {
                    unsafe { qt_set_snapshot(text.as_ptr()) };
                }
            }
        } else if session.playing && session.position_ms != last_pos_ms {
            last_pos_ms = session.position_ms;
            unsafe {
                qt_tick_playback(
                    1,
                    session.position_ms as i64,
                    session.duration_ms.unwrap_or(0) as i64,
                    session.peaks.as_ptr(),
                    session.peaks.len() as c_int,
                );
            }
        }
        if session.quit {
            break;
        }
        let frame_us = unsafe { qt_frame_interval_us() }.clamp(1000, 16667) as u64;
        thread::sleep(Duration::from_micros(frame_us));
    }
    session.close();
    let _ = CStr::from_bytes_with_nul(b"\0");
}
