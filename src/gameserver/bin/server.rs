// This is the server main file.
use log::debug;
use scrabble::{constants::global::PORT, gameserver::{board::{Grid, Grids}, gamestate::BoardState}, Action, Response, ClientEvent, Coordinate, MOVEMENT};
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

    let mut board_state = BoardState::initialize();
    
    // need to update the scrab board for the wins as well. 
    let scrab_board = Grid::new();

    board_state.set_scrab_grid(*scrab_board);

    tokio::spawn(async move {
        match listener.accept().await {
            Ok(mut connection) => {
                debug!("Accepted connection {:?}", connection);
                tokio::spawn(async move {
                    loop {
                        // now the board state will be sufficient to interact with the client
                        // actions and we can send the things we want to the server to the client. 
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
fn handle_movement(mov: MOVEMENT, cur_coords: &mut Coordinate) -> Response  {
    match mov {
        MOVEMENT::UP => {
            if cur_coords.y > 0 {
                cur_coords.y -= 1;
            }
       }, 
        MOVEMENT::DOWN => {
            if cur_coords.y < 14 {
                cur_coords.y += 1; 
            }
        },
        MOVEMENT::RIGHT => {
            if cur_coords.x < 14 {
                cur_coords.x += 1;
            }
        }, 
        MOVEMENT::LEFT => {
            if cur_coords.x > 0 {
                cur_coords.x -= 1;
            }
        }
    }

    Response { box_coordinate: Some(cur_coords.clone()), write_char: (None), win_score: None }
}

// this is the place where we have the client event. 
fn request_handler(board_state: &mut Box<BoardState>) -> Response {
    match board_state.get_action() {
        Action::DIRECTION(mov) => {
            handle_movement(mov, board_state.get_current_coord_mut())
        },
        Action::QUIT => {
            exit(0);
        },
        Action::WRITE(ch) => {
            // calculate win here also. 
            Response {box_coordinate: Some(board_state.get_current_coord().clone()), write_char: Some(ch), win_score: Some(0)}
        }, 
        Action::NONE => {
            Response {box_coordinate: None, write_char: None, win_score: Some(0)}
        }
    }
}

// here the buffer is completely populating the 1024 bytes hence when it finds a null character it
// crashes.
async fn handle_connection(con: &mut Connection, board_state: &mut Box<BoardState>) {
    debug!("handle connection {:?}", con);
    let (stream, _) = con;
    let mut buffer = [0u8; 1024];
    let mut buf_cursor = 0;

    match stream.read(&mut buffer).await {
        Ok(n) => {
            debug!("{} bytes read", n);
        }
        Err(_) => {
            debug!("Failed to read");
        }
    }

    for (buf_sz, b_ch) in buffer.iter().enumerate() {
        if b_ch.to_owned() == b'\0' {
            buf_cursor = buf_sz;
            debug!("Broke the loop at {}", buf_cursor);
            break;
        }
    }

    debug!("String {}", String::from_utf8_lossy(&buffer[0..buf_cursor]));

    if let Ok(c_event) = serde_json::from_slice::<ClientEvent>(&buffer[0..buf_cursor]) {
        board_state.set_client_event(Some(c_event));

        // board state will be updated 
        // let resp = request_handler(board_state);
        let resp = request_handler(board_state);
        debug!("Board state value {:?}", board_state.get_current_coord().clone());
        stream.flush().await.unwrap();

        let trans = serde_json::to_string(&resp).unwrap();
        stream.write_all(trans.as_bytes()).await.unwrap();
        // board will be updated on the background
    }

}
