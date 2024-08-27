use std::{
    process::{exit, Child},
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use enigo::*;

use rdev::{listen, Event, EventType};

enum State {
    Start,
    Stop,
    End,
}

fn main() {
    const PRESS: EventType = EventType::ButtonPress(rdev::Button::Unknown(1));
    const RELEASE: EventType = EventType::ButtonRelease(rdev::Button::Unknown(1));
    const EXIT: EventType = EventType::KeyPress(rdev::Key::F1);
    let state = Arc::new(Mutex::new(State::Stop));
    let s = Arc::clone(&state);

    let _handle = thread::spawn(move || {
        let mut enigo = Enigo::new(&Settings::default()).expect("can not get enigo");
        loop {
            match *s.lock().unwrap() {
                State::Start => {
                    zhuanquan(&mut enigo);
                }
                State::Stop => (),
                State::End => {
                    break;
                }
            }
            sleep(Duration::from_millis(60));
        }
    });

    let callback = move |event: Event| {
        let s = Arc::clone(&state);

        match event.event_type {
            PRESS => {
                // println!("press");
                *s.lock().unwrap() = State::Start;
            }
            RELEASE => {
                // println!("release");
                *s.lock().unwrap() = State::Stop;
            }
            EXIT => {
                *s.lock().unwrap() = State::End;
                exit(0)
            }
            _ => (),
        }
    };

    if let Err(error) = listen(callback) {
        println!("错误: {:?}", error)
    }
}

fn zhuanquan(enigo: &mut Enigo) {
    enigo.move_mouse(2000, 0, Coordinate::Rel).unwrap();
}
