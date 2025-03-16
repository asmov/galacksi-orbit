pub use bevy::prelude::*;
pub use crate::*;

pub trait MenuActionEnumTrait: 'static + Send + Sync {
    fn menu_action(&self) -> &'static MenuAction;
}

#[derive(Debug, Copy, Clone)]
pub struct MenuAction {
    pub index: usize,
    pub text: &'static str,
    pub disabled: bool,
    send_event_fn: fn(&mut World),
}

impl MenuAction {
    pub const fn new(index: usize, text: &'static str, send_event_fn: fn(&mut World)) -> Self {
        Self {
            index,
            text,
            disabled: false,
            send_event_fn,
        }
    }

    pub const fn new_disabled(index: usize, text: &'static str, send_event_fn: fn(&mut World)) -> Self {
        Self {
            index,
            text,
            disabled: true,
            send_event_fn,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn text_color(&self) -> TextColor {
        if self.disabled() {
            TextColor(palette::LESS_DARK_GRAY)
        } else {
            TextColor::WHITE
        }
    }

    pub fn text_color_hover(&self) -> TextColor {
        if self.disabled() {
            TextColor(palette::DARK_GRAY)
        } else {
            TextColor(palette::BLOOM_CYAN)
        }
    }

    pub fn text_color_pressed(&self) -> TextColor {
        if self.disabled() {
            TextColor(palette::DARK_GRAY)
        } else {
            TextColor(palette::BLOOM_GREEN)
        }
    }

    pub fn text(&self) -> Text {
        Text(self.text.to_string())
    }

    pub fn queue_event(&self, commands: &mut Commands) {
        commands.queue(self.send_event_fn);
    }
}
