#![allow(non_snake_case)]

mod gameui;
use crossbeam::crossbeam_channel::unbounded;
use chess;

fn main() {
	println!("Hello, world!");


	let rep_board = chess::Board::default();

	let (s, r) = unbounded();
	let handle = std::thread::spawn(move || {
		let _ = gameui::main(r);
	});
	s.send(rep_board.clone()).unwrap();
	println!("{:?}", rep_board);

	handle.join();
}
