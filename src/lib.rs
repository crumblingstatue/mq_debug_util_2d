pub use gamedebug_core::{
    clear_immediates, enabled, frame, imm, imm_msg, inc_frame, per, per_msg, toggle, Info,
};
use gamedebug_core::{IMMEDIATE, PERSISTENT};
use macroquad::prelude::*;

const FONT_SIZE: f32 = 20.0;

pub fn draw_world() {
    if enabled() {
        let imms = IMMEDIATE.lock().unwrap();
        for imm in imms.iter() {
            if let Info::Rect(x, y, w, h, c) = *imm {
                draw_rectangle(x, y, w, h, color_u8!(c.r, c.g, c.b, c.a))
            }
        }
    }
}

/// Draw all info bits, then clear
pub fn draw_overlay() {
    if enabled() {
        let infos = IMMEDIATE.lock().unwrap();
        let mut y = FONT_SIZE;
        draw_rectangle(
            0.,
            0.,
            420.,
            (infos.iter().filter(|i| matches!(i, Info::Msg(_))).count() + 2) as f32 * FONT_SIZE,
            Color::from_rgba(0, 0, 0, 100),
        );
        let frame = frame();
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
