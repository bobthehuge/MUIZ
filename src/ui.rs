use raylib::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UiStatus {
    #[default]
    UI_NORMAL,
    UI_LOCKED,
    UI_EXIT,
}

#[derive(Default, Debug)]
pub struct UI {
    status: UiStatus,
    components: Mult,
}

#[derive(Default, Debug)]
pub struct UIBuilder {
    status: UiStatus,
}

impl UIBuilder {
    pub fn create() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn status(&mut self, status: UiStatus) -> &mut Self {
        self.status = status;
        self
    }

    pub fn build(&mut self) -> UI {
        UI {
            status: self.status,
        }
    }
}
