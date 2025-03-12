use bevy::prelude::*;
use bevy_console::{ConsoleCommand, reply};
use clap::Parser;
use crate::*;

/// Entity designator
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum What {
    Entity(usize),
    Me,
    LocalPlayer(u8),
    Player(String)
}

impl TryFrom<&str> for What {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        const ERRSTR: &str = "Invalid paramenter for `what`";
        const ERRIDSTR: &str = "Invalid ID for `what`";

        if value == "me" {
            return Ok(What::Me);
        } else if let Ok(id) = value.parse::<usize>() {
            return Ok(What::Entity(id));
        }

        let (prefix, value) = value.split_once(':').ok_or(ERRSTR)?;
        match (prefix, value) {
            ("entity" | "ent" | "e", id) => {
                let id = id.parse::<usize>().map_err(|_| ERRIDSTR)?;
                Ok(What::Entity(id))
            },
            ("local_player" | "localplayer" | "local" | "l", num) => {
                let num = num.parse::<u8>().map_err(|_| ERRIDSTR)?;
                Ok(What::LocalPlayer(num))
            },
            ("player" | "p", name) => {
                Ok(What::Player(name.to_string()))
            },
            _ => Err(ERRSTR)
        }
    }
}

impl std::fmt::Display for What {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            What::Entity(id) => write!(f, "entity({id})"),
            What::Me => write!(f, "me"),
            What::LocalPlayer(num) => write!(f, "local_player({num})"),
            What::Player(name) => write!(f, "player({name})"),
        }
    }
}

fn parse_what(arg: &str) -> Result<What, String> {
    What::try_from(arg)
        .map_err(|err| err.to_string())
}

fn validate_in_game<CMD>(
    console_cmd: &mut ConsoleCommand<CMD>,
    mode: &Res<State<Mode>>
) -> Option<CMD> {
    let cmd = if let Some(Ok(cmd)) = console_cmd.take() { cmd } else {
        return None;
    };

    if !mode.is_game() {
        reply!(console_cmd, "error: not in game");
        return None;
    }

    Some(cmd)
}

/// Teleports an entity to the specified coordinates
#[derive(Parser, ConsoleCommand)]
#[command(name = "teleport")]
pub struct TeleportCmd {
    /// The entity to teleport
    #[arg(value_parser = parse_what)]
    what: What,
    x: u32,
    y: u32
}

pub fn teleport_cmd(
    mut console: ConsoleCommand<TeleportCmd>,
    mut local_players: Query<(&game::LocalPlayer, &mut game::Motion)>,
    mode: Res<State<Mode>>
) {
    let cmd = if let Some(c) = validate_in_game(&mut console, &mode) {c} else {
        return;
    };

    reply!(console, "teleporting {} to ({},{})", cmd.what, cmd.x, cmd.y);

    match cmd.what {
        What::Me => {
            let player = local_players.iter_mut().find(|(lp, _)| lp.num == 0);
            if let Some((_, mut motion)) = player {
                motion.position = Vec2::new(cmd.x as f32, cmd.y as f32);
                motion.velocity = Vec2::ZERO;
            }
        },
        _ => reply!(console, "error: not impelmented")
    }
}
