use std::os::raw::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: c_uchar,
    pub g: c_uchar,
    pub b: c_uchar,
    pub a: c_uchar,
}

#[link(name = "raylib")]
extern "C" {
    #[link_name = "InitWindow"]
    pub fn init_video(width: c_int, height: c_int, title: *const c_char);

    #[link_name = "CloseWindow"]
    pub fn close_video();

    #[link_name = "IsWindowReady"]
    pub fn is_video_ready() -> bool;

    #[link_name = "ClearBackground"]
    pub fn clear_background(color: Color);

    #[link_name = "DrawRectangle"]
    pub fn draw_rectangle(x: c_int, y: c_int, width: c_int, height: c_int, color: Color);

    #[link_name = "DrawText"]
    pub fn draw_text(text: *const c_char, x: c_int, y: c_int, size: c_int, color: Color);
}

#[link(name="raylib")]
extern "C" {
    #[link_name="BeginDrawing"]
    pub fn begin_frame();

    #[link_name="EndDrawing"]
    pub fn end_frame();
}

#[link(name="raylib")]
extern "C" {
    #[link_name="IsKeyPressed"]
    pub fn is_key_pressed(i32: c_int) -> bool;

    #[link_name="IsKeyReleased"]
    pub fn is_key_released(i32: c_int) -> bool;

    #[link_name="IsKeyDown"]
    pub fn is_key_down(i32: c_int) -> bool;

    #[link_name="IsKeyUp"]
    pub fn is_key_up(i32: c_int) -> bool;

    #[link_name="GetKeyPressed"]
    pub fn get_next_key() -> c_int;
}
