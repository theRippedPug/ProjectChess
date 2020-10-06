#![allow(non_snake_case)]

mod gameui;
use chess::{Board, ChessMove, Color, Square};
use crossbeam::crossbeam_channel::unbounded;
use ggez::nalgebra as na;
use std::io;
use std::thread::sleep;
use std::time::Duration;

#[macro_use]
extern crate lazy_static;

pub mod parsers;

pub const SCALEFACTOR: f32 = 0.15;

fn main() {
	println!("Hello, world!");

	let mut rep_board = Board::default();

	let (board_sender, board_receiver) = unbounded();
	let (marking_sender, marking_receiver) = unbounded();

	let renederwindow_handle = std::thread::spawn(move || {
		let something = gameui::main(board_receiver, marking_receiver);
		println!("{:?}", something);
	});
	board_sender.send(rep_board.clone()).unwrap();
	println!("{:?}", rep_board);

	//let m = ChessMove::new(Square::D2, Square::D4, None);

	// sleep(Duration::from_secs(15));

	// let new_board = rep_board.make_move_new(m);

	// board_sender.send(new_board.clone()).unwrap();
	// println!("{:?}", rep_board);

	loop {
		let mut bakra = String::new();
		let _ = io::stdin().read_line(&mut bakra).unwrap();
		let bakra = bakra.trim();

		if bakra == "end" {
			break;
		} else if bakra.chars().count() == 4 {
			println!("{}", bakra[0..2].to_string());
			println!("{}", bakra[2..4].to_string());

			let mov = ChessMove::new(
				Square::from_string(bakra[0..2].to_string()).unwrap(),
				Square::from_string(bakra[2..4].to_string()).unwrap(),
				None,
			);

			if rep_board.legal(mov.clone()) {
				let old_board = rep_board.clone();
				let promotion = mov.get_promotion().is_some();
				rep_board = rep_board.make_move_new(mov);
				let new_board = rep_board.clone();
				board_sender.send(rep_board.clone());

				let old_board_rep = parsers::parse_helper::get_matrix(&old_board);
				let new_board_rep = parsers::parse_helper::get_matrix(&new_board);

				let abstract_moves = parsers::physical_move_parser::parse(
					old_board_rep,
					parsers::abstract_move_parser::parse(old_board_rep, new_board_rep, promotion)
						.unwrap(),
				)
				.unwrap();

				let (mut path, mut reset) = (Vec::new(), Vec::new());

				let mut path_list = Vec::new();

				for i in abstract_moves{
					path.append(&mut i.0.clone());
					reset.push(i.1);
					path_list.push(i.0);
				}

				let mut game_draw_path = Vec::new();
				for path in path_list{
				let mut path_display = Vec::new();
				for i in &path {
					if i.0 >= 0.0 && i.1 >= 0.0 {
						path_display.push(na::Point2::new(
							500.0 * SCALEFACTOR * i.0 as f32
								+ SCALEFACTOR * 50.0 + 250.0 * SCALEFACTOR,
							500.0 * SCALEFACTOR * (7.0 - i.1) as f32
								+ SCALEFACTOR * 50.0 + 250.0 * SCALEFACTOR,
						));
					}
				}
				game_draw_path.push(path_display);
			}

				let _ = marking_sender.send(game_draw_path);
				println!(
					"{:?}\n{:?}\n{:?}",
					parsers::abstract_move_parser::parse(old_board_rep, new_board_rep, promotion),
					path,
					reset
				)
			} else {
				println!("nope")
			}
		}
	}

	renederwindow_handle.join();
}
