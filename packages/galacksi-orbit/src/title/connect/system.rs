use bevy::prelude::*;
use crate::*;

pub(in crate::title) fn system_enter_title_connect(mut commands: Commands) {
    commands
        .spawn((
            // fullscreen
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
                    parent
                        .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                        )
                        .with_children(|parent| {
                            parent.spawn(
                                Node {
                                    width: Val::Px(100.0),
                                    height: Val::Px(100.0),
                                    ..default()
                                },
                            );
                            parent.spawn(
                                Node {
                                    width: Val::Px(100.0),
                                    height: Val::Px(100.0),
                                    ..default()
                                }
                            );
                        });
                });
        });

    commands.insert_resource(Selected(None));
}
