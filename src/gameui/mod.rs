use crossbeam;
#[allow(dead_code)]
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Image;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use shakmaty;
use shakmaty::{Color, Role};

use std::env;
use std::path;

mod reneder_hepler;

pub const SCALEFACTOR: f32 = 0.15;


#[derive(Copy,Clone,Debug)]
pub enum ChessPieceType {
	Pawn = 0,
	Bishop = 1,
	Knight = 2,
	Rook = 3,
	Queen = 4,
	King = 5,
}

//struct that describes position of pawn on chessboard
//x goes from 1 to 8 -and- y from 'a' to 'h'
#[derive(Debug)]
pub struct PiecePos {
	x: i16,
	y: char,
}

//describes type and position of a piece
//the type goes from 0 to 5 in and this order(
//	pawn,bishop,knight,rook,queen,king
// )

#[derive(Debug)]
pub struct ChessPiece {
	piece_color: char,
	piece_type: ChessPieceType,
}
//main state:- all data in the game recides here
struct MainState {
	pos_x: f32,
	black_pieces: [graphics::Image; 6],
	white_pieces: [graphics::Image; 6],
	chess_board: graphics::Image,
	live_board: shakmaty::Board,
	recv: crossbeam::Receiver<shakmaty::Board>,
	piece_reg: Vec<ChessPiece>,
}

impl MainState {
	//this function initialises all data in the game
	fn new(ctx: &mut Context, recv: crossbeam::Receiver<shakmaty::Board>) -> GameResult<MainState> {
		//load all black pieces from hard disc
		let black_pieces = [
			Image::new(ctx, "/chess_pawns/chess-pawn-black.png")?,
			Image::new(ctx, "/chess_pawns/chess-bishop-black.png")?,
			Image::new(ctx, "/chess_pawns/chess-knight-black.png")?,
			Image::new(ctx, "/chess_pawns/chess-rook-black.png")?,
			Image::new(ctx, "/chess_pawns/chess-queen-black.png")?,
			Image::new(ctx, "/chess_pawns/chess-king-black.png")?,
		];
		//load all white pieces hrom hard disc
		let white_pieces = [
			Image::new(ctx, "/chess_pawns/chess-pawn-white.png")?,
			Image::new(ctx, "/chess_pawns/chess-bishop-white.png")?,
			Image::new(ctx, "/chess_pawns/chess-knight-white.png")?,
			Image::new(ctx, "/chess_pawns/chess-rook-white.png")?,
			Image::new(ctx, "/chess_pawns/chess-queen-white.png")?,
			Image::new(ctx, "/chess_pawns/chess-king-white.png")?,
		];

		let chess_board = Image::new(ctx, "/chess_pawns/board.png")?;
		//initialise the main state
		let s = MainState {
			pos_x: 0.0,
			black_pieces,
			white_pieces,
			chess_board,
			live_board: shakmaty::Board::new(),
			recv,
			piece_reg: Vec::new(),
		};
		Ok(s)
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
		self.pos_x = self.pos_x % 800.0 + 1.0;
		Ok(())
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
		//draw chess board in the background
		graphics::draw(
			ctx,
			&self.chess_board,
			graphics::DrawParam::new().scale([SCALEFACTOR * 2.0, SCALEFACTOR * 2.0]),
		)?;

		let circle = graphics::Mesh::new_circle(
			ctx,
			graphics::DrawMode::fill(),
			na::Point2::new(self.pos_x, 380.0),
			100.0,
			2.0,
			graphics::WHITE,
		)?;
		//graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
		//graphics::draw(ctx, &self.black_pieces[2], graphics::DrawParam::new())?;
		let simpl_board = reneder_hepler::getMatrix(&self.live_board);
		for (y,row) in simpl_board.iter().enumerate(){
			for (x,maybe_piece) in row.iter().enumerate(){
				if maybe_piece.is_some() {
					let piece = maybe_piece.as_ref().unwrap();
					if piece.piece_color== 'b'{
						//println!("({},{},{})",piece.piece_type as usize,x,y);
						graphics::draw(ctx,
							&self.black_pieces[piece.piece_type as usize],
							graphics::DrawParam::new()
							.scale([SCALEFACTOR,SCALEFACTOR])
							.dest([ 500.0 * SCALEFACTOR * x as f32 + SCALEFACTOR * 50.0,500.0 * SCALEFACTOR * y as f32 + SCALEFACTOR * 50.0])
					)?;
					}else{
						graphics::draw(ctx,
							&self.white_pieces[piece.piece_type as usize],
							graphics::DrawParam::new()
							.scale([SCALEFACTOR,SCALEFACTOR])
							.dest([ 500.0 * SCALEFACTOR * x as f32 + SCALEFACTOR * 50.0 ,500.0 * SCALEFACTOR * y as f32 + SCALEFACTOR * 50.0])
						)?;
					}
				}
				//println!("yeeeee");
			}
			//println!("heehaw");
		}
		graphics::present(ctx)?;
		Ok(())
	}
}

pub fn main(recv: crossbeam::channel::Receiver<shakmaty::Board>) -> ggez::GameResult {
	//mount resource directory
	let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
		let mut path = path::PathBuf::from(manifest_dir);
		path.push("assets");
		path
	} else {
		path::PathBuf::from("./assets")
	};

	//build window tith the following charecteristics
	let cb = ggez::ContextBuilder::new("super_simple", "ggez")
		.add_resource_path(resource_dir)
		.window_mode(ggez::conf::WindowMode {
			width: (SCALEFACTOR * 500.0 * 8.0) + SCALEFACTOR * 2.0 * 50.0,
			height: (SCALEFACTOR * 500.0 * 8.0) + SCALEFACTOR * 2.0 * 50.0,
			maximized: false,
			fullscreen_type: ggez::conf::FullscreenType::Windowed,
			borderless: false,
			min_width: 0.0,
			max_width: 0.0,
			min_height: 0.0,
			max_height: 0.0,
			resizable: false,
		});

	//build a window instance
	let (ctx, event_loop) = &mut cb.build()?;
	//mount Mainstate onto the window
	let state = &mut MainState::new(ctx, recv)?;
	//run the game
	event::run(ctx, event_loop, state)
}

pub fn getType(tp: shakmaty::Piece) -> (char, ChessPieceType) {
	let colour: char = match tp.color {
		Color::Black => 'b',
		Color::White => 'w',
	};

	let piece = match tp.role {
		Role::Pawn => ChessPieceType::Pawn,
		Role::Bishop => ChessPieceType::Bishop,
		Role::Knight => ChessPieceType::Knight,
		Role::Rook => ChessPieceType::Rook,
		Role::Queen => ChessPieceType::Queen,
		Role::King => ChessPieceType::King,
	};

	(colour, piece)
}
