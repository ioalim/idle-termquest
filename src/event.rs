use std::{sync::mpsc::{self, TryRecvError}, thread, time::{Duration, Instant}};

use anyhow::Result;
use crossterm::event::{self, KeyEvent, MouseEvent, Event as CrosstermEvent, KeyEventKind};

use crate::app::App;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick, Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    if event::poll(timeout).expect("Unable to poll for event") {
                        match event::read().expect("Unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                } else {
                                    Ok(())
                                }
                            },
                            CrosstermEvent::Mouse(e) => {
                                sender.send(Event::Mouse(e))
                            },
                            CrosstermEvent::Resize(w, h) => {
                                sender.send(Event::Resize(w, h))
                            },
                            _ => unimplemented!()
                        }
                        .expect("Failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("Failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self, app: &mut App) -> Result<Event> {
        let start_time = Instant::now();
        let mut ok_event = None;
        loop {
            match self.receiver.try_recv() {
                Ok(event) => {
                    //app.log.push_back(format!("{event:?}"));
                    //if app.log.len() > 20 {
                    //    app.log.pop_front();
                    //}
                    ok_event = Some(event)
                },
                Err(TryRecvError::Empty) => {
                    if let Some(event) = ok_event {
                        return Ok(event)
                    } else {
                        return Ok(Event::Tick)
                    }
                },
                Err(TryRecvError::Disconnected) => return Err(anyhow::anyhow!("Event handler disconnected")),
            }
            if start_time.elapsed() >= Duration::from_millis(25) {
                if let Some(event) = ok_event {
                    return Ok(event)
                } else {
                    return Ok(Event::Tick)
                }
            }
        }

        //match res_event {
        //    Ok(event) => result = Ok(event),
        //    Err(TryRecvError::Empty) => result = Ok(Event::Tick),
        //    Err(TryRecvError::Disconnected) => result = Err(anyhow::anyhow!("Event handler disconnected")),
        //}

        //Ok(self.receiver.recv()?)
    }
}
