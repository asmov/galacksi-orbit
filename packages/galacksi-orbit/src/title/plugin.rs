use bevy::prelude::*;
use crate::{color::*, util::*, Mode};
use super::*;

pub fn plugin_title(app: &mut App) {
    app
        .init_state::<MenuScreen>()
        .add_systems(OnEnter(Mode::Title), system_enter_title)
        .add_systems(OnEnter(MenuScreen::Title), system_enter_title_menu)
        .add_systems(OnExit(MenuScreen::Title), system_exit_title_menu)
        .add_systems(Update, (
            system_update_title_mouse,
            system_update_title_button,
        ).run_if(in_state(Mode::Title)));
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
enum MenuScreen {
    #[default]
    Inactive,
    Title,
}

fn system_enter_title(mut menu_screen: ResMut<NextState<MenuScreen>>) {
    menu_screen.set(MenuScreen::Title);
}

fn system_exit_title_menu(query: Query<Entity, With<OnMenuScreen>>, mut commands: Commands) {
    commands.remove_resource::<Selected>();
    despawn_entities(query, commands);
}

const STR_GALACKSI_ORBIT: &'static str = " G A L A C K S I\nO R B I T";
const STR_SIMULATE: &'static str = "simulate";
const STR_QUIT: &'static str = "quit";

fn system_enter_title_menu(
    mut commands: Commands
) {
    let button_style = Node {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextFont {
        font_size: 40.0,
        ..default()
    };

    let border_radius = BorderRadius::new(Val::Px(20.), Val::Px(20.), Val::Px(20.), Val::Px(20.));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                )
                .with_children(|parent| {
                    parent.spawn((
                        Text(STR_GALACKSI_ORBIT.to_string()),
                        TextLayout {
                            justify: JustifyText::Center,
                            ..default()
                        },
                        TextFont {
                            font_size: 80.0,
                            ..default()
                        },
                        TextColor(Palette::rand_bloom()),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }
                    ));
                    parent.spawn((
                        Button,
                        button_text_style.clone(),
                        border_radius.clone(),
                        button_style.clone(),
                        MenuAction::Simulate,
                        Selection(MenuAction::Simulate as usize, Interaction::None)
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text(STR_SIMULATE.to_string()),
                            button_text_style.clone(),
                            TextColor::BLACK
                        ));
                    });
                    parent.spawn((
                        Button,
                        button_text_style.clone(),
                        border_radius.clone(),
                        button_style.clone(),
                        MenuAction::Quit,
                        Selection(MenuAction::Quit as usize, Interaction::None)
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text(STR_QUIT.to_string()),
                            button_text_style.clone(),
                            TextColor::BLACK
                        ));
                    });

                });
        });

    commands.insert_resource(Selected(None));
}

fn system_update_title_mouse(
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

fn system_update_title_button(
    mut interaction_query: Query<
        (
            &Selection,
            &MenuAction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children
        ),
        (Changed<Selection>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut mode: ResMut<NextState<Mode>>,
    mut menu_screen: ResMut<NextState<MenuScreen>>,
    text_query: Query<Entity>,
    mut text_writer: TextUiWriter
) {
    for (selection, menu_action, mut bg_color, mut _border_color, children) in &mut interaction_query {
        let text = text_query.get(children[0]).unwrap();

        let interaction = selection.1;
        match interaction {
            Interaction::Pressed => {
                *text_writer.color(text, 0) = TextColor::WHITE;
                //text.sections[0].style.color = Color::WHITE;
                *bg_color = swatch::MENU_BUTTON_BG_HOVER.into();

                match menu_action {
                    MenuAction::Simulate => {
                        mode.set(Mode::Game);
                        menu_screen.set(MenuScreen::Inactive);
                    },
                    MenuAction::Quit => {
                        app_exit_events.send(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *text_writer.color(text, 0) = TextColor::WHITE;
                //text.sections[0].style.color = Palette::rand_button_text();
                *bg_color = swatch::MENU_BUTTON_BG_HOVER.into();
            }
            Interaction::None => {
                *text_writer.color(text, 0) = TextColor::WHITE;
                //text.sections[0].style.color = Color::BLACK;
                *bg_color = swatch::MENU_BUTTON_BG_NORMAL.into();
            }
        }
    }
}
