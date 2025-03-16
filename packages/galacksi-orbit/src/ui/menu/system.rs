use bevy::prelude::*;
use crate::*;

pub fn system_exit_menuscreen(query: Query<Entity, With<OnMenuScreen>>, mut commands: Commands) {
    commands.remove_resource::<Selected>();
    despawn_entities(query, commands);
}

pub fn system_update_menu_action_selection(
    mut interaction_query: Query< ( &Interaction, &mut Selection,), (Changed<Interaction>, With<Button>), >,
    mut selected: ResMut<Selected>,
) {
    for (interaction, mut selection) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                selected.0 = Some((selection.0, Interaction::Pressed));
                selection.1 = Interaction::Pressed;
            }
            Interaction::Hovered => {
                selected.0 = Some((selection.0, Interaction::Hovered));
                selection.1 = Interaction::Hovered;
            }
            Interaction::None => {
                selected.0 = Some((selection.0, Interaction::None));
                selection.1 = Interaction::None;
            }
        }
    }
}

pub fn system_update_menu_action_button(
    mut interaction_query: Query<
        (
            &Selection,
            &MenuActionEnum,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children
        ),
        (Changed<Selection>, With<Button>),
    >,
    mut commands: Commands,

    text_query: Query<Entity>,
    mut text_writer: TextUiWriter
) {
    for (selection, menu_action_enum, mut bg_color, mut _border_color, children) in &mut interaction_query {
        let text = text_query.get(children[0]).unwrap();

        let menu_action = menu_action_enum.menu_action();
        let interaction = selection.1;
        match interaction {
            Interaction::Pressed => {
                *text_writer.color(text, 0) = menu_action.text_color_pressed();
                *bg_color = swatch::MENU_BUTTON_BG_HOVER.into();
                menu_action.queue_event(&mut commands);

                /*match menu_action.0 {
                    TitleMenuAction::Connect => {
                        menu_screen.set(TitleScreen::Connect);
                    },
                    TitleMenuAction::Simulate => {
                        mode.set(Mode::Game);
                        menu_screen.set(TitleScreen::Inactive);
                    },
                    TitleMenuAction::Configure => {/*todo*/},
                    TitleMenuAction::Review => {/*todo*/},
                    TitleMenuAction::Quit => {
                        app_exit_events.send(AppExit::Success);
                    }
                }*/
            }
            Interaction::Hovered => {
                *text_writer.color(text, 0) = menu_action.text_color_hover();
                *bg_color = swatch::MENU_BUTTON_BG_HOVER.into();
            }
            Interaction::None => {
                *text_writer.color(text, 0) = menu_action.text_color();
                *bg_color = swatch::MENU_BUTTON_BG_NORMAL.into();
            }
        }
    }
}
