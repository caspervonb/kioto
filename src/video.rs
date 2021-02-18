//! Video-system manipulation operations.
//!

use crate::sys;
use std::ffi;

pub type Color = sys::Color;

impl Color {
    pub const GRAY: Color = Color {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
    };
    pub const DARKGRAY: Color = Color {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
    };
    pub const YELLOW: Color = Color {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
    };
    pub const GOLD: Color = Color {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
    };
    pub const ORANGE: Color = Color {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
    };
    pub const PINK: Color = Color {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
    };
    pub const RED: Color = Color {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
    };
    pub const MAROON: Color = Color {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
    };
    pub const LIME: Color = Color {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
    };
    pub const DARKGREEN: Color = Color {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
    };
    pub const SKYBLUE: Color = Color {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
    };
    pub const DARKBLUE: Color = Color {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
    };
    pub const PURPLE: Color = Color {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
    };
    pub const VIOLET: Color = Color {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
    };
    pub const DARKPURPLE: Color = Color {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
    };
    pub const BEIGE: Color = Color {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
    };
    pub const BROWN: Color = Color {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
    };
    pub const DARKBROWN: Color = Color {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const BLANK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const RAYWHITE: Color = Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
    };
}

/// Clear the screen to the given background color.
///
/// # Examples
///
/// ```no_run
/// use kioto::video;
///
/// video::clear_background(video::Color::BLACK);
/// ```
pub fn clear_background(color: Color) {
    unsafe {
        sys::clear_background(color);
    }
}

/// Draw a line with the given color.
///
/// # Examples
///
/// ```no_run
/// use kioto::video;
///
/// video::draw_line(0, 0, 100, 100, video::Color::WHITE);
/// ```
pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
    unsafe {
        sys::draw_line(x1, y1, x2, y2, color);
    }
}

/// Draw a circle filled with the given color.
///
/// # Examples
///
/// ```no_run
/// use kioto::video;
///
/// video::draw_circle(0, 0, 100.0, video::Color::WHITE);
/// ```
pub fn draw_circle(x: i32, y: i32, radius: f32, color: Color) {
    unsafe {
        sys::draw_circle(x, y, radius, color);
    }
}

/// Draw a rectangle filled with the given color.
///
/// # Examples
///
/// ```no_run
/// use kioto::video;
///
/// video::draw_rectangle(0, 0, 100, 100, video::Color::WHITE);
/// ```
pub fn draw_rectangle(x: i32, y: i32, width: i32, height: i32, color: Color) {
    unsafe {
        sys::draw_rectangle(x, y, width, height, color);
    }
}

/// Draws the given text string with the given color.
///
/// # Examples
///
/// ```no_run
/// use kioto::video;
///
/// video::draw_text("Hello, world!", 0, 0, 32, video::Color::WHITE);
/// ```
pub fn draw_text(text: &str, x: i32, y: i32, size: i32, color: Color) {
    let text = ffi::CString::new(text).unwrap();

    unsafe {
        sys::draw_text(text.as_ptr(), x, y, size, color);
    }
}
