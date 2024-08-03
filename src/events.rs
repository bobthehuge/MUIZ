use crate::widgets::DataObj;
use std::cmp::Ordering;

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum EventType {
    #[default]
    IDLE,
    LOG,
    DRAW,
    CLICK,
    CUSTOM,
    QUIT,
}

#[derive(Default)]
pub struct WidgetEvent(pub EventType, pub DataObj);

impl WidgetEvent {
    pub const DRAW_EVENT: WidgetEvent =
        WidgetEvent(EventType::DRAW, DataObj::None);
}

impl PartialEq for WidgetEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for WidgetEvent {}

impl Ord for WidgetEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0 as u8).cmp(&(other.0 as u8))
    }
}

impl PartialOrd for WidgetEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.0 as u8).cmp(&(other.0 as u8)))
    }
}
