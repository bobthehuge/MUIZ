use crate::events::WidgetEvent;
use crate::ui::{BaseHandler, EnvMap};

use raylib::core::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;

#[derive(Default)]
pub enum DataObj {
    #[default]
    None,
    I32(i32),
    F32(f32),
    String(String),
    Vec(Vec<DataObj>),
    BaseHandler(BaseHandler),
}

pub trait WidgetDrawable: Widget {
    fn visible(&self) -> bool;
    fn show(&mut self);
    fn hide(&mut self);
    fn toggle_visibility(&mut self);
    fn draw(&self, handle: &mut RaylibDrawHandle, env: &mut EnvMap);
    fn render(&self, handle: &mut RaylibDrawHandle, env: &mut EnvMap) {
        if self.visible() {
            self.draw(handle, env)
        }
    }
}

pub trait WidgetCallable: Widget {
    fn is_ready(&self) -> bool;
    fn ready(&mut self);
    fn unready(&mut self);
    fn toggle_ready(&mut self);
    fn call(&self, env: &mut EnvMap) -> WidgetEvent;
}

pub trait WidgetCollidable: WidgetDrawable {
    fn locked(&self) -> bool;
    fn lock(&mut self);
    fn unlock(&mut self);
    fn toggle_lock(&mut self);
    fn interact(
        &mut self,
        handle: &mut RaylibHandle,
        env: &mut EnvMap,
    ) -> WidgetEvent;
}

pub trait Widget {
    fn get_id(&self) -> &str;

    fn try_as_widget(&self) -> Option<&dyn Widget> {
        None
    }

    fn try_as_widget_mut(&mut self) -> Option<&mut dyn Widget> {
        None
    }

    fn try_as_callable(&self) -> Option<&dyn WidgetCallable> {
        None
    }

    fn try_as_callable_mut(&mut self) -> Option<&mut dyn WidgetCallable> {
        None
    }

    fn try_as_collidable(&self) -> Option<&dyn WidgetCollidable> {
        None
    }

    fn try_as_collidable_mut(&mut self) -> Option<&mut dyn WidgetCollidable> {
        None
    }

    fn try_as_drawable(&self) -> Option<&dyn WidgetDrawable> {
        None
    }

    fn try_as_drawable_mut(&mut self) -> Option<&mut dyn WidgetDrawable> {
        None
    }
}
