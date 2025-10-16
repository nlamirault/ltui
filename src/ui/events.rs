use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::time::Duration;
use tokio::sync::mpsc;

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    Refresh,
    Quit,
}

pub struct EventHandler {
    sender: mpsc::UnboundedSender<AppEvent>,
    receiver: mpsc::UnboundedReceiver<AppEvent>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self { sender, receiver }
    }

    pub async fn next(&mut self) -> Option<AppEvent> {
        self.receiver.recv().await
    }

    pub fn start(&self) {
        let sender = self.sender.clone();
        tokio::spawn(async move {
            loop {
                if event::poll(Duration::from_millis(250)).unwrap() {
                    match event::read().unwrap() {
                        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                            if let Err(_) = sender.send(AppEvent::Key(key_event)) {
                                break;
                            }
                        }
                        _ => {}
                    }
                } else {
                    if let Err(_) = sender.send(AppEvent::Tick) {
                        break;
                    }
                }
            }
        });

        let sender = self.sender.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                if let Err(_) = sender.send(AppEvent::Refresh) {
                    break;
                }
            }
        });
    }
}