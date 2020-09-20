#![allow(non_snake_case)]

mod gameui;
use chess::{Board, ChessMove, Color, Square};
use crossbeam::crossbeam_channel::unbounded;
use std::io;
use std::thread::sleep;
use std::time::Duration;

pub mod parsers;

fn main() {
	println!("Hello, world!");

	let mut rep_board = Board::default();

	let (board_sender, board_receiver) = unbounded();
	let renederwindow_handle = std::thread::spawn(move || {
		let _ = gameui::main(board_receiver);
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

				println!("{:?}", parsers::abstract_move_parser::parse(old_board_rep, new_board_rep, promotion))

			}else{println!("nope")}
		}
	}

	renederwindow_handle.join();
}
