pub mod ui;
pub mod widgets;

use crate::ui::*;
use crate::widgets::*;
use raylib::prelude::*;

pub struct Rect {
    id: String,
    visible: bool,
    rect: Rectangle,
}

impl Rect {
    pub fn new(id: &str, rect: &Rectangle) -> Self {
        Self {
            id: String::from(id),
            visible: false,
            rect: *rect,
        }
    }
}

impl Widget for Rect {
    fn get_id(&self) -> &str {
        &self.id
    }
    fn try_as_widget(&self) -> Option<&dyn Widget> {
        Some(self as _)
    }
    fn try_as_widget_mut(&mut self) -> Option<&mut dyn Widget> {
        Some(self as _)
    }
    fn try_as_drawable(&self) -> Option<&dyn WidgetDrawable> {
        Some(self as _)
    }
    fn try_as_drawable_mut(&mut self) -> Option<&mut dyn WidgetDrawable> {
        Some(self as _)
    }
}

impl WidgetDrawable for Rect {
    fn visible(&self) -> bool {
        self.visible
    }
    fn show(&mut self) {
        self.visible = true
    }
    fn hide(&mut self) {
        self.visible = false
    }
    fn toggle_visibility(&mut self) {
        self.visible ^= true
    }
    fn draw(&self, handle: &mut RaylibDrawHandle) {
        handle.draw_rectangle_rec(self.rect, Color::RED)
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .log_level(TraceLogLevel::LOG_WARNING)
        .size(640, 480)
        .title("MUIZ")
        .build();

    let mut ui = UIBuilder::create().build();
    let mut rect = Rect::new("rect", &Rectangle::new(50.0, 50.0, 50.0, 50.0));
    rect.show();
    let id = ui.register_as_drawable(rect);
    ui.tag_index(id, String::from("L9"));

    while !rl.window_should_close() && !ui.should_exit() {
        let mut handle = rl.begin_drawing(&thread);
        handle.clear_background(Color::WHITE);
        ui.for_each_drawables(|x| x.render(&mut handle))
    }
}
