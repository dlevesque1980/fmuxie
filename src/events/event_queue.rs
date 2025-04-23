// event_queue.rs
use crate::AppEvent;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::time::Duration;
use crossterm::event::{self, Event as CEvent};

pub struct EventQueue {
    pub sender: Sender<AppEvent>,
    pub receiver: Receiver<AppEvent>,
}

impl EventQueue {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        let input_tx = tx.clone();
        thread::spawn(move || {
            loop {
                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let CEvent::Key(key) = event::read().unwrap() {
                        input_tx.send(AppEvent::Input(key)).unwrap();
                    }
                }
                input_tx.send(AppEvent::Tick).ok();
            }
        });

        EventQueue { sender: tx, receiver: rx }
    }
}