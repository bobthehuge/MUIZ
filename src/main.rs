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
    fn draw(&self, handle: &mut RaylibDrawHandle, _: &mut EnvMap) {
        handle.draw_rectangle_rec(self.rect, Color::RED)
    }
}

fn on_loop(ui: &mut UI, (_rl, _thread): RaylibContext) {
    ui.events.queue.push(WidgetEvent::drawcall())
}

fn on_event(ui: &mut UI, (rl, thread): RaylibContext, _e: WidgetEvent) {
    let mut draw_handle = rl.begin_drawing(thread);

    draw_handle.clear_background(Color::WHITE);

    ui.widgets.for_each_drawables(|x| {
        x.render(&mut draw_handle, &mut ui.data.globals)
    })
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .log_level(TraceLogLevel::LOG_WARNING)
        .size(640, 480)
        .title("MUIZ")
        .build();

    let mut ui = UIBuilder::init().build();
    let mut rect = Rect::new("rect", &Rectangle::new(50.0, 50.0, 50.0, 50.0));
    rect.show();
    let id = ui.widgets.register_as_drawable(rect);
    ui.widgets.tag(id, String::from("L9"));

    let handlers = UiHandlers {
        on_loop: Box::new(on_loop),
        on_event: Box::new(on_event),
    };

    ui.run((&mut rl, &thread), &handlers);
}
