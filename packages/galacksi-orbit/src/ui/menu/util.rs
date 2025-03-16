use bevy::prelude::*;
use crate::ui::*;

pub fn spawn_button(
    menu_action_enum: MenuActionEnum,
    parent: &mut ChildBuilder,
    button_node: Node,
    button_font: TextFont,
    button_border_radius: BorderRadius
) {
    let menu_action = menu_action_enum.0.menu_action();

    parent.spawn((
        Button,
        button_node,
        button_border_radius,
        menu_action_enum,
        Selection(menu_action.index(), Interaction::None)
    ))
    .with_children(|parent| {
        parent.spawn((
            menu_action.text(),
            button_font,
            menu_action.text_color()
        ));
    });
}

pub fn send_event<E: Event>(world: &mut World, event: E) {
    world.send_event(event);
}
