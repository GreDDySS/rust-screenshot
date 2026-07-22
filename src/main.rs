#![warn(clippy::pedantic)]

use chrono::{DateTime, Utc};
use rdev::{Event, EventType, Key, grab};
use screenshots::Screen;
use std::env;
use std::fs;

const TARGET_DIR: &str = "screens";

fn main() -> std::io::Result<()> {
    // Читаем аргументы командной строки
    // cargo run [OPTIONS] [ARGS]
    // cargo run -- test
    let args: Vec<String> = env::args().collect();

    // Получаем директорию скринов
    let screens_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();

    // Текущая дериктория
    let mut path = env::current_dir()?;
    // Прописываем директорию скринов
    path.push(&screens_dir);
    // Создание директории
    fs::create_dir_all(path)?;

    if let Err(error) = grab(move |e| callback(e, &screens_dir)) {
        println!("Error {error:?}")
    }

    Ok(())
}

fn callback(event: Event, screens_dir: &String) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::CapsLock) => {
            make_screen(screens_dir);
            None
        }
        _ => Some(event),
    }
}

fn make_screen(screens_dir: &String) {
    let screens = Screen::all().unwrap();

    for screen in screens {
        let image = screen.capture().unwrap();

        let now: DateTime<Utc> = Utc::now();

        image
            .save(format!(
                "{}/{}.png",
                screens_dir,
                now.format("%d-%m-%Y")
            ))
            .unwrap();
    }
}
