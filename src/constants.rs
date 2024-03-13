pub mod global {
    pub const PORT: u32 = 8888;


    use std::time::Duration;

    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    use futures::{FutureExt, StreamExt};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Action {
        UP,
        DOWN,
        RIGHT,
        LEFT,
        DUMMY,
        NONE,
    }

    #[derive(Debug)]
    pub enum KeyPress {
        Key(crossterm::event::KeyEvent),
        Error, 
        Tick,
    }


    // Now this will be our stream source. 
    pub struct KeyboardEvent {
        _tx: tokio::sync::mpsc::UnboundedSender<KeyPress>,
        rx: tokio::sync::mpsc::UnboundedReceiver<KeyPress>,
        task: Option<tokio::task::JoinHandle<()>>,
    }

    impl KeyboardEvent {
        // So this will be creating an unbounded channel, the source will be the crossterm
        // keypresses, the reciever will be the place where this is called. 
        // Example: The update function in the game program. 
        pub fn new() -> Self {
            let (ltx, lrx) = tokio::sync::mpsc::unbounded_channel::<KeyPress>();

            let t_rate = Duration::from_millis(250);
            let _tx: tokio::sync::mpsc::UnboundedSender<KeyPress> = ltx.clone(); 

            let task = tokio::spawn(async move{
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

            KeyboardEvent{_tx,rx: lrx, task: Some(task)}
        }

        pub async fn next(&mut self) -> Option<KeyPress> {
            self.rx.recv().await
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ClientEvent {
        pub action: Action,
    }

    impl ClientEvent {
        pub fn default() -> Self {
            ClientEvent {
                action: Action::DUMMY,
            }
        }

        pub fn from_key(keyp: &KeyEvent) -> Self {
            match keyp {
                KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, state: KeyEventState::KEYPAD } => ClientEvent {
                    action: Action::UP,
                },
                _ => ClientEvent::default(),
            }
        }
    }

}
