// This is the server main file.
use log::{debug, info};
use scrabble::{
    constants::global::PORT, gameserver::{
        board::{Grid, Grids, Sack, SackTiles},
        gamestate::BoardState, server_player::{self, ServerPlayer},
    }, players::Player, Action, ClientEvent, Coordinate, Response, MOVEMENT
};
use std::net::SocketAddr;
use std::process::exit;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

type Connection = (TcpStream, SocketAddr);
#[tokio::main]
async fn main() {
    // we will need to initialize the scrabble board state.
    // Scrabble board state will include the values related to the values entered on the board.
    env_logger::init();
    println!("Running the server");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .expect("Failed to bind to the port");

    // sack will be instantiated here.
    // when the players connect for the first time then the values from the sack will be given to
    // them

    let mut sack: SackTiles = Sack::new_sack();

    sack.shuffle_sack();
    debug!("{:?}", sack);
    let mut board_state = BoardState::initialize();

    // need to update the scrab board for the wins as well.
    let scrab_board = Grid::new();

    board_state.set_scrab_grid(*scrab_board).await;

    let _serve_thread = tokio::spawn(async move {
        match listener.accept().await {
            Ok(mut connection) => {
                board_state.add_player(ServerPlayer::new()).await;
                debug!("Accepted connection {:?}", connection);
                tokio::spawn(async move {
                    loop {
                        // now the board state will be sufficient to interact with the client
                        // actions and we can send the things we want to the server to the client.
                        //
                        // The problem happens to be here, the connection is not being made by the
                        // tcp stream.
                        handle_connection(&mut connection, &mut board_state).await;
                    }
                });
            }
            Err(e) => {
                println!("Failed to accept the client because {}", e);
            }
        }
    });

    loop {}
}

// the current coordinates will be changed here.
fn handle_movement(mov: MOVEMENT, cur_coords: &mut Coordinate) -> Response {
    match mov {
        MOVEMENT::UP => {
            if cur_coords.y > 0 {
                cur_coords.y -= 1;
            }
        }
        MOVEMENT::DOWN => {
            if cur_coords.y < 14 {
                cur_coords.y += 1;
            }
        }
        MOVEMENT::RIGHT => {
            if cur_coords.x < 14 {
                cur_coords.x += 1;
            }
        }
        MOVEMENT::LEFT => {
            if cur_coords.x > 0 {
                cur_coords.x -= 1;
            }
        }
    }

    Response {
        player_turn: scrabble::PLAYER::Player1,
        box_coordinate: Some(*cur_coords),
        write_char: (None),
        win_score: None,
    }
}

// this is the place where we have the client event.
async fn request_handler(board_state: &mut Box<BoardState>) -> Response {
    match board_state.get_action() {
        Action::DIRECTION(mov) => 
            handle_movement(mov, board_state.get_current_coord_mut()), Action::QUIT => {
            exit(0);
        }
        Action::WRITE(ch) => {
            // check if the player can actually write the specific values.
            Response {
                player_turn: scrabble::PLAYER::Player1,
                box_coordinate: Some(*board_state.get_current_coord()),
                write_char: Some(ch),
                win_score: Some(0),
            }
        }
        Action::END => {
            Response {
                player_turn: scrabble::PLAYER::Player1,
                box_coordinate: Some(*board_state.get_current_coord()),
                write_char: None,
                win_score: Some(0),
            }

        },
        _ => Response {
            player_turn: scrabble::PLAYER::Player1,
            box_coordinate: None,
            write_char: None,
            win_score: None,
        },
    }
}

// here the buffer is completely populating the 1024 bytes hence when it finds a null character it
// crashes.
async fn handle_connection(con: &mut Connection, board_state: &mut Box<BoardState>) {
    info!("Handling the connection");
    // board_state.describe_players().await; 
    let (stream, _) = con;
    let mut buffer = [0u8; 1024];
    let mut buf_cursor = 0;

    let n = stream.read(&mut buffer).await;

    match n {
        Ok(m) => {
            log::info!("Successful reading {}", m);
        }
        Err(e) => {
            log::error!("Failed to read {}", e);
        }
    }

    for (buf_sz, buf_ch) in buffer.iter().enumerate() {
        if *buf_ch == b'\0' {
            buf_cursor = buf_sz;
            break;
        }
    }

    debug!("String {}", String::from_utf8_lossy(&buffer[0..buf_cursor]));

    if let Ok(c_event) = serde_json::from_slice::<ClientEvent>(&buffer[0..buf_cursor]) {
        if c_event != ClientEvent::default() {
            board_state.set_client_event(Some(c_event));
        }

        let resp = request_handler(board_state).await;
        log::info!("Response generated from the request handler {:?}", resp);
        // board state will be updated
        debug!(
            "Board state value {:?}",
            board_state.get_current_coord().clone()
        );
        stream.flush().await.unwrap();

        let trans = serde_json::to_string(&resp).unwrap();
        stream.write_all(trans.as_bytes()).await.unwrap();
        // board will be updated on the background
    }
}
