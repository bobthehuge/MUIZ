use crate::widgets::*;

use raylib::core::drawing::RaylibDrawHandle;
use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UiState {
    #[default]
    UI_IDLE,
    UI_LOCKED,
    UI_DRAW,
    UI_CALL,
    UI_INTERACT,
    UI_EXIT,
}

pub type Index = usize;

#[derive(Default)]
pub struct UI {
    pub status: UiState,
    pub drawables: Vec<Index>,
    pub tags: HashMap<String, Index>,
    pub widgets: HashMap<Index, Box<dyn Widget>>,
    indices: usize,
}

impl UI {
    pub fn should_exit(&self) -> bool {
        self.status == UiState::UI_EXIT
    }

    pub fn next_index(&mut self) -> Index {
        let idx = self.indices;
        self.indices += 1;
        idx
    }

    pub fn try_next_index(&mut self) -> Option<Index> {
        let idx = self.indices;
        self.indices = self.indices.checked_add(1)?;
        Some(idx)
    }

    #[inline]
    pub fn indices(&self) -> usize {
        self.indices
    }

    pub fn tag_index(&mut self, i: Index, tag: String) -> Option<usize> {
        self.tags.insert(tag, i)
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

    pub fn register(&mut self, w: impl Widget + 'static) -> Index {
        let idx = self.next_index();
        self.widgets.insert(idx, Box::new(w));
        idx
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
    ) -> Index {
        let idx = self.register(w);
        self.drawables.push(idx);
        idx
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
    drawables: usize,
    tags: usize,
    capacity: usize,
}

impl UIBuilder {
    pub fn create() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn status(&mut self, status: UiState) -> &mut Self {
        self.status = status;
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
            drawables: Vec::with_capacity(self.drawables),
            tags: HashMap::<String, Index>::with_capacity(self.tags),
            widgets: {
                HashMap::<Index, Box<dyn Widget>>::with_capacity(self.capacity)
            },
            indices: 0,
        }
    }
}
