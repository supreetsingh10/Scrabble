pub mod gameserver; 
pub mod client;
pub mod constants;



use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum MOVEMENT {
    UP, 
    DOWN,
    RIGHT,
    LEFT
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum Action {
    DIRECTION(MOVEMENT),
    QUIT,
    WRITE,
    NONE,
}

pub enum KeyPress {
    Key(crossterm::event::KeyEvent),
    Error, 
    Tick,
}


// Now this will be our stream source. 
pub struct KeyboardEvent {
    _tx: tokio::sync::mpsc::UnboundedSender<KeyPress>,
    rx: tokio::sync::mpsc::UnboundedReceiver<KeyPress>,
    // task: Option<tokio::task::JoinHandle<()>>,
}



/// TODO
// This will need elements such as Tile, which will have the information related to the value
// added. 
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub struct ClientEvent {
    pub action: Action,
}

impl ClientEvent 
{
    pub fn default() -> Self {
        ClientEvent {
            action: Action::NONE,
        }
    }

    pub fn from_key(keyp: &KeyEvent) -> Self {
        log::debug!("Key pressed {:?}", keyp);
        match keyp {
            KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, state: KeyEventState::NONE} => ClientEvent {
                action: Action::DIRECTION(MOVEMENT::UP),
            },
            KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, state: KeyEventState::NONE} => ClientEvent {
                action: Action::DIRECTION(MOVEMENT::DOWN),
            },
            KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, state: KeyEventState::NONE} => ClientEvent {
                action: Action::DIRECTION(MOVEMENT::RIGHT),
            },
            KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, state: KeyEventState::NONE} => ClientEvent {
                action: Action::DIRECTION(MOVEMENT::LEFT),
            },
            KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, state: KeyEventState::NONE} => ClientEvent { 
                action: Action::QUIT 
            },
            _ => ClientEvent::default(),
        }
    }
}

use std::time::Duration;
impl KeyboardEvent {
    // So this will be creating an unbounded channel, the source will be the crossterm
    // keypresses, the reciever will be the place where this is called. 
    // Example: The update function in the game program. 
    pub fn new() -> Self {
        let (ltx, lrx) = tokio::sync::mpsc::unbounded_channel::<KeyPress>();

        let t_rate = Duration::from_millis(250);
        let _tx: tokio::sync::mpsc::UnboundedSender<KeyPress> = ltx.clone(); 

        let _ = tokio::spawn(async move{
            let mut reader = crossterm::event::EventStream::new(); 
            let mut inter = tokio::time::interval(t_rate);

            loop {
                let delay = inter.tick();
                let crossterm_event = reader.next().fuse(); 

                // select uses both the async functions. 
                tokio::select! {
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(evt)) => {
                                match evt {
                                    crossterm::event::Event::Key(key) => {
                                        if key.kind == crossterm::event::KeyEventKind::Press {
                                            // process the key here. 
                                            ltx.send(KeyPress::Key(key)).unwrap();
                                        }
                                    }
                                    _ => {

                                    }
                                }
                            },
                            Some(Err(_)) => {
                                ltx.send(KeyPress::Error).unwrap();

                            }
                            None => {

                            }
                        }
                    }, 
                    _ = delay => {
                        // panic here for some reason. 
                        // maybe because I was pushing this too quick . 
                        // understand the select macro a bit better. 
                        ltx.send(KeyPress::Tick).unwrap();
                    },
                }
            }
        });

        KeyboardEvent{_tx,rx: lrx /*, task: Some(task)*/}
    }

    pub async fn next(&mut self) -> Option<KeyPress> {
        self.rx.recv().await
    }
}

