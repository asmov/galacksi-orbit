//! Modified form from from bevy_console
use std::{
    io::{BufRead, Write},
    sync::{Arc, Mutex},
};

use bevy::{
    app::{App, Update},
    log::tracing_subscriber::{self, EnvFilter, Layer, Registry},
    prelude::{EventWriter, IntoSystemConfigs, ResMut, Resource},
};

use bevy_console::{ConsoleSet, PrintConsoleLine};

/// Buffers logs written by bevy at runtime
#[derive(Resource)]
pub struct BevyLogBuffer(Arc<Mutex<std::io::Cursor<Vec<u8>>>>);

/// Writer implementation which writes into a buffer resource inside the bevy world
pub struct BevyLogBufferWriter(Arc<Mutex<std::io::Cursor<Vec<u8>>>>);

impl Write for BevyLogBufferWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // let lock = self.0.upgrade().unwrap();
        let mut lock = self.0.lock().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to lock buffer: {}", e),
            )
        })?;
        lock.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // let lock = self.0.upgrade().unwrap();
        let mut lock = self.0.lock().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to lock buffer: {}", e),
            )
        })?;
        lock.flush()
    }
}

/// Flushes the log buffer and sends its content to the console
pub fn send_log_buffer_to_console(
    buffer: ResMut<BevyLogBuffer>,
    mut console_lines: EventWriter<PrintConsoleLine>,
) {
    let mut buffer = buffer.0.lock().unwrap();
    // read and clean buffer
    let buffer = buffer.get_mut();
    for line in buffer.lines().map_while(Result::ok) {
        console_lines.send(PrintConsoleLine { line });
    }
    buffer.clear();
}

pub fn make_galacksi_log_layer(
    app: &mut App,
) -> Option<Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync>> {
    let buffer = Arc::new(Mutex::new(std::io::Cursor::new(Vec::new())));
    app.insert_resource(BevyLogBuffer(buffer.clone()));
    app.add_systems(
        Update,
        send_log_buffer_to_console.in_set(ConsoleSet::PostCommands),
    );

    let filter = "galacksi_orbit=info,warn,debug,error".to_string();
    let env_filter = EnvFilter::builder().parse_lossy(filter);

    Some(Box::new(
        tracing_subscriber::fmt::Layer::new()
            .with_target(false)
            .with_ansi(true)
            .with_writer(move || BevyLogBufferWriter(buffer.clone()))
            .with_filter(env_filter)
    ))
}

pub fn make_filtered_log_layer(
    app: &mut App,
    filter: EnvFilter,
) -> Option<Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync>> {
    let buffer = Arc::new(Mutex::new(std::io::Cursor::new(Vec::new())));
    app.insert_resource(BevyLogBuffer(buffer.clone()));
    app.add_systems(
        Update,
        send_log_buffer_to_console.in_set(ConsoleSet::PostCommands),
    );

    Some(Box::new(
        tracing_subscriber::fmt::Layer::new()
            .with_target(false)
            .with_ansi(true)
            .with_writer(move || BevyLogBufferWriter(buffer.clone()))
            .with_filter(filter)
    ))
}
