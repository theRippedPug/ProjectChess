use super::parsers::PhysicalPos;
use crossbeam;
#[allow(dead_code)]
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Font, Image, Text, TextFragment};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use std::env;
use std::path;

mod reneder_hepler;

#[derive(Copy, Clone, Debug)]
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
	black_pieces: [graphics::Image; 6],
	white_pieces: [graphics::Image; 6],
	chess_board: graphics::Image,
	live_board: chess::Board,
	line_list: Vec<Vec<na::Point2<f32>>>,
	font: graphics::Font,
	recv: crossbeam::Receiver<chess::Board>,
	mark_recv: crossbeam::Receiver<Vec<Vec<na::Point2<f32>>>>,
}

impl MainState {
	//this function initialises all data in the game
	fn new(
		ctx: &mut Context,
		recv: crossbeam::Receiver<chess::Board>,
		mark_recv: crossbeam::Receiver<Vec<Vec<na::Point2<f32>>>>,
	) -> GameResult<MainState> {
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

		let font = Font::new(ctx, "/CPMonoBold.ttf")?;

		let chess_board = Image::new(ctx, "/chess_pawns/board.png")?;
		//initialise the main state
		let s = MainState {
			black_pieces,
			white_pieces,
			chess_board,
			live_board: chess::Board::default(),
			line_list: Vec::new(),
			font,
			recv,
			mark_recv,
		};
		Ok(s)
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
		if !self.recv.is_empty() {
			self.live_board = self.recv.recv().unwrap();
		}
		if !self.mark_recv.is_empty() {
			self.line_list = self.mark_recv.recv().unwrap();
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
		//draw chess board in the background
		graphics::draw(
			ctx,
			&self.chess_board,
			graphics::DrawParam::new().scale([super::SCALEFACTOR * 2.0, super::SCALEFACTOR * 2.0]),
		)?;

		//graphics::draw(ctx, &self.black_pieces[2], graphics::DrawParam::new())?;
		let simpl_board = reneder_hepler::getMatrix(&self.live_board);
		for (y, row) in simpl_board.iter().enumerate() {
			for (x, maybe_piece) in row.iter().enumerate() {
				//draw the position text

				if maybe_piece.is_some() {
					let piece = maybe_piece.as_ref().unwrap();
					if piece.piece_color == 'b' {
						//println!("({},{},{})",piece.piece_type as usize,x,y);
						graphics::draw(
							ctx,
							&self.black_pieces[piece.piece_type as usize],
							graphics::DrawParam::new()
								.scale([super::SCALEFACTOR, super::SCALEFACTOR])
								.dest([
									500.0 * super::SCALEFACTOR * x as f32
										+ super::SCALEFACTOR * 50.0,
									500.0 * super::SCALEFACTOR * y as f32
										+ super::SCALEFACTOR * 50.0,
								]),
						)?;
					} else {
						graphics::draw(
							ctx,
							&self.white_pieces[piece.piece_type as usize],
							graphics::DrawParam::new()
								.scale([super::SCALEFACTOR, super::SCALEFACTOR])
								.dest([
									500.0 * super::SCALEFACTOR * x as f32
										+ super::SCALEFACTOR * 50.0,
									500.0 * super::SCALEFACTOR * y as f32
										+ super::SCALEFACTOR * 50.0,
								]),
						)?;
					}
				}
				//println!("yeeeee");
				graphics::draw(
					ctx,
					&Text::new(
						TextFragment::new(reneder_hepler::getPosString(x, 7 - y))
							.color(graphics::Color::new(0.0, 0.0, 0.0, 1.0))
							.font(self.font),
					),
					graphics::DrawParam::new().dest([
						500.0 * super::SCALEFACTOR * x as f32 + super::SCALEFACTOR * 50.0,
						500.0 * super::SCALEFACTOR * y as f32 + super::SCALEFACTOR * 50.0,
					]),
				)?;
			}
			//println!("heehaw");
		}
		for point_list in &self.line_list{
		if point_list.len() >= 2 {
			let line_mesh = graphics::MeshBuilder::new()
				.line(
					&point_list,
					5.0,
					graphics::Color::new(0.0, 0.0, 1.0, 0.5),
				)?
				.build(ctx)?;
			graphics::draw(ctx, &line_mesh, graphics::DrawParam::new());
		}}
		graphics::present(ctx)?;
		Ok(())
	}
}

pub fn main(
	recv: crossbeam::channel::Receiver<chess::Board>,
	mark_recv: crossbeam::Receiver<Vec<Vec<na::Point2<f32>>>>,
) -> ggez::GameResult {
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
			width: (super::SCALEFACTOR * 500.0 * 8.0) + super::SCALEFACTOR * 2.0 * 50.0,
			height: (super::SCALEFACTOR * 500.0 * 8.0) + super::SCALEFACTOR * 2.0 * 50.0,
			maximized: false,
			fullscreen_type: ggez::conf::FullscreenType::Windowed,
			borderless: false,
			min_width: 0.0,
			max_width: 0.0,
			min_height: 0.0,
			max_height: 0.0,
			resizable: false,
		});
	//.backend(ggez::conf::Backend::OpenGLES { major: 3, minor: 0 });

	//build a window instance
	let (ctx, event_loop) = &mut cb.build()?;
	//mount Mainstate onto the window
	let state = &mut MainState::new(ctx, recv, mark_recv)?;
	//run the game
	event::run(ctx, event_loop, state)
}
