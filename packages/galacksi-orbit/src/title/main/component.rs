use bevy::prelude::*;
use crate::ui::*;

#[derive(Component, strum::EnumCount, strum::EnumIter, Copy, Clone, Eq, PartialEq)]
pub enum TitleMenuAction {
    Connect = 0,
    Simulate = 1,
    Configure = 2,
    Review = 3,
    Quit = 4
}

#[derive(Event)]
pub enum TitleMenuActionEvent {
    Connect,
    Simulate,
    Configure,
    Review,
    Quit
}

impl TitleMenuAction {
    pub const CONNECT: MenuAction = MenuAction::new(0, "connect", |w| send_event(w, TitleMenuActionEvent::Connect));
    pub const SIMULATE: MenuAction = MenuAction::new(1, "simulate", |w| send_event(w, TitleMenuActionEvent::Simulate));
    pub const CONFIGURE: MenuAction = MenuAction::new(2, "configure", |w| send_event(w, TitleMenuActionEvent::Configure));
    pub const REVIEW: MenuAction = MenuAction::new(3, "review", |w| send_event(w, TitleMenuActionEvent::Review));
    pub const QUIT: MenuAction = MenuAction::new(4, "quit", |w| send_event(w, TitleMenuActionEvent::Quit));
}

impl MenuActionEnumTrait for TitleMenuAction {
    fn menu_action(&self) -> &'static MenuAction {
        match self {
            Self::Connect => &Self::CONNECT,
            Self::Simulate => &Self::SIMULATE,
            Self::Configure => &Self::CONFIGURE,
            Self::Review => &Self::REVIEW,
            Self::Quit => &Self::QUIT,
        }
    }
}
