use lazy_static::lazy_static;
use macroquad::prelude::*;
use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    Mutex,
};

lazy_static! {
    static ref IMMEDIATE: Mutex<Vec<Info>> = Mutex::new(Vec::new());
    static ref PERSISTENT: Mutex<Vec<PerEntry>> = Mutex::new(Vec::new());
}

static FRAME_COUNTER: AtomicU32 = AtomicU32::new(0);

pub enum Info {
    Msg(String),
    Rect(f32, f32, f32, f32, Color),
}

pub static ENABLED: AtomicBool = AtomicBool::new(false);

/// Add immediate info for the current frame
pub fn imm(info: Info) {
    if ENABLED.load(Ordering::Acquire) {
        IMMEDIATE.lock().unwrap().push(info);
    }
}

struct PerEntry {
    frame: u32,
    info: Info,
}

/// Add persistent information
pub fn per(info: Info) {
    let mut log = PERSISTENT.lock().unwrap();
    log.push(PerEntry {
        frame: FRAME_COUNTER.load(Ordering::Acquire),
        info,
    });
    if log.len() > 20 {
        log.remove(0);
    }
}

const FONT_SIZE: f32 = 20.0;

pub fn draw_world() {
    if ENABLED.load(Ordering::Acquire) {
        let imms = IMMEDIATE.lock().unwrap();
        for imm in imms.iter() {
            if let Info::Rect(x, y, w, h, c) = *imm {
                draw_rectangle(x, y, w, h, c)
            }
        }
    }
}

/// Draw all info bits, then clear
pub fn draw_overlay() {
    if ENABLED.load(Ordering::Acquire) {
        let infos = IMMEDIATE.lock().unwrap();
        let mut y = FONT_SIZE;
        draw_rectangle(
            0.,
            0.,
            420.,
            (infos.iter().filter(|i| matches!(i, Info::Msg(_))).count() + 2) as f32 * FONT_SIZE,
            Color::from_rgba(0, 0, 0, 100),
        );
        let frame = FRAME_COUNTER.load(Ordering::Acquire);
        draw_text(
            &format!("= Debug (frame {}) =", frame),
            0.,
            y,
            FONT_SIZE,
            WHITE,
        );
        for info in infos.iter() {
            if let Info::Msg(string) = info {
                y += FONT_SIZE;
                draw_text(string, 0., y, FONT_SIZE, WHITE);
            }
        }
        // Draw logs
        let msgs = 10;
        let h = FONT_SIZE * msgs as f32;
        draw_rectangle(
            0.,
            screen_height() - h,
            420.,
            h,
            Color::from_rgba(0, 0, 0, 100),
        );
        let log = PERSISTENT.lock().unwrap();
        for (i, entry) in log.iter().rev().take(msgs).enumerate() {
            if let Info::Msg(string) = &entry.info {
                draw_text(
                    &format!("{}: {}", entry.frame, string),
                    0.,
                    screen_height() - i as f32 * FONT_SIZE,
                    FONT_SIZE,
                    WHITE,
                );
            }
        }
    }
}

pub fn clear_immediates() {
    IMMEDIATE.lock().unwrap().clear();
}

pub fn toggle() {
    let current = ENABLED.load(Ordering::Acquire);
    ENABLED.store(!current, Ordering::Release);
}

pub fn enabled() -> bool {
    ENABLED.load(Ordering::Acquire)
}

pub fn inc_frame() {
    let frame = FRAME_COUNTER.load(Ordering::Acquire);
    FRAME_COUNTER.store(frame + 1, Ordering::Release);
}

#[macro_export]
macro_rules! imm_msg {
    ($x:expr) => {{
        if crate::debug::ENABLED.load(std::sync::atomic::Ordering::Acquire) {
            crate::debug::imm(crate::debug::Info::Msg(format!(
                concat!(stringify!($x), ": {:?}"),
                $x
            )));
        }
        $x
    }};
}

#[macro_export]
macro_rules! per_msg {
    ($($arg:tt)*) => {{
        if crate::debug::ENABLED.load(std::sync::atomic::Ordering::Acquire) {
            crate::debug::per(crate::debug::Info::Msg(format!($($arg)*)));
        }
    }};
}
