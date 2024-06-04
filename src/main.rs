pub mod multimap;
pub mod ui;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .log_level(TraceLogLevel::LOG_WARNING)
        .size(640, 480)
        .title("MUIZ")
        .build();

    while !rl.window_should_close() {
        let mut handle = rl.begin_drawing(&thread);

        handle.clear_background(Color::WHITE);
        handle.draw_text("Hello, caca!", 12, 12, 20, Color::BLACK);
    }
}
