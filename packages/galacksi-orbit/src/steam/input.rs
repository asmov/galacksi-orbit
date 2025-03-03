use std::sync;
use bevy::prelude::*;
use bevy_steamworks as steamworks;

const ACTIONSET_GAME: &'static str = "Game";
const ACTIONSET_MENU: &'static str = "Menu";

const ACTION_MENU_UP: &'static str = "Menu_Up";
const ACTION_MENU_DOWN: &'static str = "Menu_Down";
const ACTION_MENU_SELECT: &'static str = "Menu_Select";
const ACTION_MENU_CANCEL: &'static str = "Menu_Cancel";

const ACTION_GAME_MOVE: &'static str = "Move";
const ACTION_GAME_AIM: &'static str = "Aim";

pub type SteamHandle = std::os::raw::c_ulonglong;

pub struct MenuInputHandles {
    pub actionset: SteamHandle,
    pub menu_up: SteamHandle,
    pub menu_down: SteamHandle,
    pub menu_select: SteamHandle,
    pub menu_cancel: SteamHandle,
}

pub struct GameInputHandles {
    pub actionset: SteamHandle,
    pub game_move: SteamHandle,
    pub game_aim: SteamHandle,
}

#[derive(Debug, Default)]
pub struct ButtonState {
    pub released: bool,
    pub just_pressed: bool,
    pub pressed: bool,
    pub just_released: bool,
}

#[derive(Debug, Default)]
pub struct JoystickState {
    pub xy: Vec2,
}

impl ButtonState {
    pub fn update(&mut self, steamworks: &Res<steamworks::Client>, controller_handle: SteamHandle, button_handle: SteamHandle) {
        let input = steamworks.input();
        let button = input.get_digital_action_data(controller_handle, button_handle);

        if button.bState {
            self.just_pressed = !self.pressed;
            self.pressed = true;
        } else {
            self.just_released = self.pressed;
            self.pressed = false;
            self.released = true;
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct ControllerInputCollection {
    pub controller_inputs: Vec<ControllerInput>,
}

impl ControllerInputCollection {
    pub fn update(&mut self, steamworks: &Res<steamworks::Client>) {
        let input = steam.input();
        let connected_controller_handles = input.get_connected_controllers();

        for controller_handle in connected_controller_handles {
            let input = self.controller_inputs.iter_mut().find(|i| i.controller_handle() == controller_handle);

            if let Some(input) = input {
                input.update(steam);
            } else {
                self.controller_inputs.push(ControllerInput::Connected(controller_handle));
            }
        }
    }
}

pub trait ControllerHandle {
    fn controller_handle(&self) -> SteamHandle;
}

pub trait InputUpdate {
    fn update(&mut self, steam: &Res<steamworks::Client>);
}

pub enum ControllerInputType {
    Connected,
    Disconnected,
    Menu,
    Game
}

#[derive(Debug)]
pub enum ControllerInput {
    Connected(SteamHandle),
    Disconnected(SteamHandle),
    Menu(SteamHandle, MenuInput),
    Game(SteamHandle, GameInput),
}

impl ControllerInput {
    pub fn set_type(&mut self, input_type: ControllerInputType) {
        match input_type {
            ControllerInputType::Connected | ControllerInputType::Disconnected => panic!("Invalid input type"),
            ControllerInputType::Menu => {
                if let Self::Menu(_, _) = self {
                    return;
                }

                let handle = self.controller_handle();
                *self = Self::Menu(handle, MenuInput::new(handle));
            },
            ControllerInputType::Game => {
                if let Self::Game(_, _) = self {
                    return;
                }

                let handle = self.controller_handle();
                *self = Self::Game(handle, GameInput { controller: handle, movement: JoystickState::default(), aim: JoystickState::default() });
            }
        }
    }
}

impl ControllerHandle for ControllerInput {
    fn controller_handle(&self) -> SteamHandle {
        match self {
            Self::Connected(handle) => *handle,
            Self::Disconnected(handle) => *handle,
            Self::Menu(handle, _) => *handle,
            Self::Game(handle, _) => *handle,
        }
    }
}

impl InputUpdate for ControllerInput {
    fn update(&mut self, steam: &Res<steamworks::Client>) {
        match self {
            Self::Connected(_) => {},
            Self::Disconnected(_) => {},
            Self::Menu(_, input) => input.update(steam),
            Self::Game(_, input) => input.update(steam)
        }
    }
}

#[derive(Default, Debug)]
pub struct MenuInput {
    pub controller: SteamHandle,
    pub up: ButtonState,
    pub down: ButtonState,
    pub select: ButtonState,
    pub cancel: ButtonState,
}

#[derive(Default, Debug)]
pub struct GameInput {
    pub controller: SteamHandle,
    pub movement: JoystickState,
    pub aim: JoystickState
}

impl ControllerHandle for GameInput {
    fn controller_handle(&self) -> SteamHandle {
        self.controller
    }
}

impl InputUpdate for GameInput {
    fn update(&mut self, steam: &Res<steamworks::Client>) {
        let input = steam.input();
        let movement = input.get_analog_action_data(self.controller, 0);
        let aim = input.get_analog_action_data(self.controller, 1);

        self.movement.xy = Vec2::new(movement.x, movement.y);
        self.aim.xy = Vec2::new(aim.x, aim.y);
    }
}

impl MenuInput {
    pub fn new(controller: SteamHandle) -> Self {
        Self {
            controller,
            up: ButtonState::default(),
            down: ButtonState::default(),
            select: ButtonState::default(),
            cancel: ButtonState::default(),
        }
    }
}

impl InputUpdate for MenuInput {
    fn update(&mut self, steam: &Res<steamworks::Client>) {
        let handles = menu_input_handles(&steam);
        self.up.update(steam, self.controller, handles.menu_up);
        self.down.update(steam, self.controller, handles.menu_down);
        self.select.update(steam, self.controller, handles.menu_select);
        self.cancel.update(steam, self.controller, handles.menu_cancel);
    }
}

pub(crate) fn menu_actionset_input_handles(steam: &Res<steamworks::Client>) -> &'static MenuInputHandles {
    static HANDLES: sync::OnceLock<MenuInputHandles> = sync::OnceLock::new();
    HANDLES.get_or_init(|| {
        let steam_input = steam.input();
        let actionset = steam_input.get_action_set_handle(ACTIONSET_MENU);
        assert_ne!(0, actionset, "Failed to access Steam Input action set {}", ACTIONSET_MENU);

        let menu_up = steam_input.get_digital_action_handle(ACTION_MENU_UP);
        assert_ne!(0, menu_up, "Failed to access Steam Input action {}", ACTION_MENU_UP);

        let menu_down = steam_input.get_digital_action_handle(ACTION_MENU_DOWN);
        assert_ne!(0, menu_down, "Failed to access Steam Input action {}", ACTION_MENU_DOWN);

        let menu_select = steam_input.get_digital_action_handle(ACTION_MENU_SELECT);
        assert_ne!(0, menu_select, "Failed to access Steam Input action {}", ACTION_MENU_SELECT);

        let menu_cancel = steam_input.get_digital_action_handle(ACTION_MENU_CANCEL);
        assert_ne!(0, menu_cancel, "Failed to access Steam Input action {}", ACTION_MENU_CANCEL);

        MenuInputHandles {
            actionset,
            menu_up,
            menu_down,
            menu_select,
            menu_cancel,
        }
    })
}

pub(crate) fn game_actionset_input_handles(steam: &Res<steamworks::Client>) -> &'static GameInputHandles {
    static HANDLES: sync::OnceLock<GameInputHandles> = sync::OnceLock::new();
    HANDLES.get_or_init(|| {
        let steam_input = steam.input();
        let actionset = steam_input.get_action_set_handle(ACTIONSET_GAME);
        assert_ne!(0, actionset, "Failed to access Steam Input action set {}", ACTIONSET_MENU);

        let game_move = steam_input.get_analog_action_handle(ACTION_GAME_MOVE);
        assert_ne!(0, game_move, "Failed to access Steam Input action {}", ACTION_GAME_MOVE);

        let game_aim = steam_input.get_analog_action_handle(ACTION_GAME_AIM);
        assert_ne!(0, game_aim, "Failed to access Steam Input action {}", ACTION_GAME_AIM);

        GameInputHandles {
            actionset,
            game_move,
            game_aim,
        }
    })
}
