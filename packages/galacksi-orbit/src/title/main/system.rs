use bevy::prelude::*;
use strum::IntoEnumIterator;
use crate::*;

pub(in crate::title) fn system_enter_title(mut title_screen: ResMut<NextState<TitleScreen>>) {
    title_screen.set(TitleScreen::Main);
}

const STR_GALACKSI_ORBIT: &'static str = " G A L A C K S I\nO R B I T";

pub(in crate::title) fn system_enter_title_main(
    mut commands: Commands
) {
    let button_node = Node {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_font = TextFont {
        font_size: 40.0,
        ..default()
    };

    let button_border_radius = BorderRadius::new(Val::Px(20.), Val::Px(20.), Val::Px(20.), Val::Px(20.));

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

                    for menu_action in TitleMenuAction::iter() {
                        spawn_button(MenuActionEnum::new(menu_action),
                            parent, button_node.clone(), button_font.clone(), button_border_radius);
                    }
                });
        });

    commands.insert_resource(Selected(None));
}

pub(in crate::title) fn event_title_menu_action(
    mut events: EventReader<TitleMenuActionEvent>,
    mut app_exit_events: EventWriter<AppExit>,
    mut mode: ResMut<NextState<Mode>>,
    mut menu_screen: ResMut<NextState<TitleScreen>>,
) {
    for event in events.read() {
        match event {
            TitleMenuActionEvent::Connect => {
                menu_screen.set(TitleScreen::Connect);
            },
            TitleMenuActionEvent::Simulate => {
                mode.set(Mode::Game);
                menu_screen.set(TitleScreen::Inactive);
            },
            TitleMenuActionEvent::Configure => {/*todo*/},
            TitleMenuActionEvent::Review => {/*todo*/},
            TitleMenuActionEvent::Quit => {
                app_exit_events.send(AppExit::Success);
            }
        }
    }
}
