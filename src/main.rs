mod gameui;
use crossbeam::crossbeam_channel::unbounded;
use shakmaty;
use shakmaty::{Color, Role};

fn main() {
	println!("Hello, world!");

	let board = shakmaty::Board::new();

	let (s, r) = unbounded();
	let handle = std::thread::spawn(move || {
		let _ = gameui::main(r);
	});
	s.send(board.clone()).unwrap();
	println!("{:?}", board);

	handle.join();
}
