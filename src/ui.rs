use raylib::prelude::*;

use crate::widgets::*;
use std::collections::{BinaryHeap, HashMap};

pub type WidgetID = usize;
pub type EnvMap = HashMap<String, DataObj>;
pub type TagMap = HashMap<String, WidgetID>;
pub type WidgetMap = HashMap<WidgetID, Box<dyn Widget>>;
pub type EventHeap = BinaryHeap<WidgetEvent>;
pub type EventHandler = Box<dyn Fn(&mut UI, RaylibContext, WidgetEvent)>;
pub type LoopHandler = Box<dyn Fn(&mut UI, RaylibContext)>;
pub type RaylibContext<'a> = (&'a mut RaylibHandle, &'a RaylibThread);

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UiState {
    #[default]
    UI_IDLE,
    UI_LOCKED,
    UI_DRAW,
    UI_CALL,
    // Keyboard or Mouse inputs
    UI_INTERACT,
    // External inputs ? (idk atm)
    UI_IO,
    UI_EXIT,
}

#[derive(Default)]
pub struct UiData {
    pub globals: EnvMap,
}

#[derive(Default)]
pub struct UiWidgets {
    pub drawables: Vec<WidgetID>,
    pub tags: TagMap,
    pub widgets: WidgetMap,
    indices: usize,
}

#[derive(Default)]
pub struct UiEvents {
    pub queue: EventHeap,
}

#[derive(Default)]
pub struct UI {
    pub status: UiState,
    pub data: UiData,
    pub events: UiEvents,
    pub widgets: UiWidgets,
}

pub struct UiHandlers {
    pub on_loop: LoopHandler,
    pub on_event: EventHandler,
}

impl Default for UiHandlers {
    fn default() -> Self {
        Self {
            on_loop: Box::new(|_, _| {}),
            on_event: Box::new(|_, _, _| {}),
        }
    }
}

impl UI {
    pub fn should_exit(&self) -> bool {
        self.status == UiState::UI_EXIT
    }

    pub fn run(&mut self, (rl, thread): RaylibContext, handlers: &UiHandlers) {
        while !rl.window_should_close() && !self.should_exit() {
            (*handlers.on_loop)(self, (rl, thread));

            if let Some(event) = self.events.queue.pop() {
                (*handlers.on_event)(self, (rl, thread), event);
            }
        }
    }
}

impl UiWidgets {
    pub fn next_id(&mut self) -> WidgetID {
        let id = self.indices;
        self.indices += 1;
        id
    }

    pub fn try_next_id(&mut self) -> Option<WidgetID> {
        let id = self.indices;
        self.indices = self.indices.checked_add(1)?;
        Some(id)
    }

    #[inline]
    pub fn ids(&self) -> usize {
        self.indices
    }

    pub fn tag(&mut self, id: WidgetID, tag: String) -> Option<usize> {
        self.tags.insert(tag, id)
    }

    pub fn get_by_tag(&self, tag: &String) -> Option<&dyn Widget> {
        self.widgets.get(self.tags.get(tag)?)?.try_as_widget()
    }

    pub fn get_mut_by_tag(&mut self, tag: &String) -> Option<&mut dyn Widget> {
        self.widgets
            .get_mut(self.tags.get(tag)?)?
            .try_as_widget_mut()
    }

    pub fn get_drawable_by_tag(
        &self,
        tag: &String,
    ) -> Option<&dyn WidgetDrawable> {
        self.widgets.get(self.tags.get(tag)?)?.try_as_drawable()
    }

    pub fn get_drawable_mut_by_tag(
        &mut self,
        tag: &String,
    ) -> Option<&mut dyn WidgetDrawable> {
        self.widgets
            .get_mut(self.tags.get(tag)?)?
            .try_as_drawable_mut()
    }

    pub fn register(&mut self, w: impl Widget + 'static) -> WidgetID {
        let id = self.next_id();
        self.widgets.insert(id, Box::new(w));
        id
    }

    #[inline]
    pub fn for_each_widgets(&self, mut f: impl FnMut(&dyn Widget)) {
        self.drawables.iter().for_each(|x| {
            f(self.widgets.get(x).unwrap().try_as_widget().unwrap())
        })
    }

    #[inline]
    pub fn for_each_widgets_mut(&mut self, mut f: impl FnMut(&mut dyn Widget)) {
        self.drawables.iter().for_each(|x| {
            f(self
                .widgets
                .get_mut(x)
                .unwrap()
                .try_as_widget_mut()
                .unwrap())
        })
    }

    pub fn register_as_drawable(
        &mut self,
        w: impl WidgetDrawable + 'static,
    ) -> WidgetID {
        let id = self.register(w);
        self.drawables.push(id);
        id
    }

    #[inline]
    pub fn for_each_drawables(&self, mut f: impl FnMut(&dyn WidgetDrawable)) {
        self.drawables.iter().for_each(|x| {
            f(self.widgets.get(x).unwrap().try_as_drawable().unwrap())
        })
    }

    #[inline]
    pub fn for_each_drawables_mut(
        &mut self,
        mut f: impl FnMut(&mut dyn WidgetDrawable),
    ) {
        self.drawables.iter().for_each(|x| {
            f(self
                .widgets
                .get_mut(x)
                .unwrap()
                .try_as_drawable_mut()
                .unwrap())
        })
    }
}

#[derive(Default, Debug)]
pub struct UIBuilder {
    status: UiState,
    globals: usize,
    drawables: usize,
    tags: usize,
    capacity: usize,
}

impl UIBuilder {
    pub fn init() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn status(&mut self, status: UiState) -> &mut Self {
        self.status = status;
        self
    }

    pub fn globals(&mut self, globals: usize) -> &mut Self {
        self.globals = globals;
        self
    }

    pub fn capacity(&mut self, capacity: usize) -> &mut Self {
        if capacity < self.drawables + self.tags {
            panic!("Requested elements don't fit in given capacity")
        }

        self.capacity = capacity;
        self
    }

    pub fn drawables(&mut self, drawables: usize) -> &mut Self {
        self.drawables = drawables;
        self
    }

    pub fn tags(&mut self, tags: usize) -> &mut Self {
        self.tags = tags;
        self
    }

    pub fn build(&mut self) -> UI {
        UI {
            status: self.status,
            data: UiData {
                globals: EnvMap::with_capacity(self.globals),
            },
            events: UiEvents::default(),
            widgets: UiWidgets {
                drawables: Vec::with_capacity(self.drawables),
                tags: TagMap::with_capacity(self.tags),
                widgets: {
                    HashMap::<WidgetID, Box<dyn Widget>>::with_capacity(
                        self.capacity,
                    )
                },
                indices: 0,
            },
        }
    }
}
