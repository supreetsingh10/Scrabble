pub mod client;
pub mod constants;
pub mod gameserver;
pub mod players;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use futures::{FutureExt, StreamExt};
use gameserver::board::ScrabTile;
use serde::{Deserialize, Serialize};

type PlayerNo = u32;
// sending the coordinate.
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    // this box coordinate will be used to highglight the box and as well as will be used to write
    // the values to it.
    pub box_coordinate: Option<Coordinate>,
    pub player_turn: PlayerNo,
    // if None is recieved that means the letter typed does not exist for the given player.
    pub write_char: Option<ScrabTile>,
    pub win_score: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}

impl Coordinate {
    pub fn new() -> Self {
        Coordinate { x: 0, y: 0 }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum MOVEMENT {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum Action {
    DIRECTION(MOVEMENT),
    // Super will be working as a key that will be ending the game round if it is on, and starting
    // the game round if it is off.
    END,
    QUIT,
    WRITE(char),
    WAITING,
    NONE,
}

#[derive(Debug, Clone, Copy)]
pub enum KeyPress {
    Key(crossterm::event::KeyEvent),
    ERROR,
    NONE,
    TICK,
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

impl Default for ClientEvent {
    fn default() -> Self {
        ClientEvent {
            action: Action::NONE,
        }
    }
}

impl ClientEvent {
    pub fn from_key(keyp: &KeyEvent) -> Self {
        //log::debug!("Key pressed {:?}", keyp);

        for ch in b'a'..=b'z' {
            if KeyCode::Char(ch as char) == keyp.code {
                return ClientEvent {
                    action: Action::WRITE(ch as char),
                };
            }
        }

        for ch in b'A'..=b'Z' {
            if KeyCode::Char(ch as char) == keyp.code {
                return ClientEvent {
                    action: Action::WRITE(ch as char),
                };
            }
        }

        match keyp {
            KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => ClientEvent {
                action: Action::DIRECTION(MOVEMENT::UP),
            },
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => ClientEvent {
                action: Action::END,
            },
            KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => ClientEvent {
                action: Action::DIRECTION(MOVEMENT::DOWN),
            },
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => ClientEvent {
                action: Action::DIRECTION(MOVEMENT::RIGHT),
            },
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => ClientEvent {
                action: Action::DIRECTION(MOVEMENT::LEFT),
            },
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => ClientEvent {
                action: Action::QUIT,
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
    // the event handler has to be redone. We do not need async eventstream.
    pub fn new() -> Self {
        let (ltx, lrx) = tokio::sync::mpsc::unbounded_channel::<KeyPress>();

        let t_rate = Duration::from_millis(10);
        let _tx: tokio::sync::mpsc::UnboundedSender<KeyPress> = ltx.clone();

        let eve_thread = tokio::spawn(async move {
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
                                            println!("key pressed {:?}", &key);
                                            match ltx.send(KeyPress::Key(key)) {
                                                Ok(()) => log::info!("Sending successful"),
                                                Err(e) => log::error!("Sending failed {}", e),
                                            }
                                        }
                                    }
                                    _ => {
                                        match ltx.send(KeyPress::NONE) {
                                            Ok(()) => log::info!("Some other random event occured successful"),
                                            Err(e) => log::error!("Random failed {}", e),
                                        }
                                        // for all the other crossterm events.
                                    }
                                }
                            },
                            Some(Err(_)) => {
                                log::error!("Error came");
                                ltx.send(KeyPress::ERROR).unwrap();
                            }
                            None => {
                            }
                        }
                    },
                    _ = delay => {
                        // do not send tick as it is not required.
                        if let Err(e) = ltx.send(KeyPress::TICK) {
                            panic!("failed to tick {}", e);
                        }
                    },
                }
            }
        });

        KeyboardEvent { _tx, rx: lrx }
    }

    pub async fn next(&mut self) -> Option<KeyPress> {
        self.rx.recv().await
    }
}
