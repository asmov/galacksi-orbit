use crate::steam;

pub fn setup_steam_game(
    mut commands: Commands,
) {
    steam::setup_steam_game_input(steam);
    commands.insert_resource(ControllerInputCollection::default());
}

fn teardown_steam_game(query: Query<Entity, With<OnGameScreen>>, mut commands: Commands) {
    commands.remove_resource::<ControllerInputCollection>();
}
