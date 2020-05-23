use shakmaty;
use gameui;

pub fn getType(tp: shakmaty::Piece) -> (char, gameui::ChessPieceType){
	let colour:char = match tp.color {
		Color::Black => 'b',
		Color::White => 'w'
	};

	let piece = match tp.role {
		Role::Pawn => gameui::ChessPieceType::Pawn,
		Role::Bishop => gameui::ChessPieceType::Bishop,
		Role::Knight => gameui::ChessPieceType::Knight,
		Role::Rook => gameui::ChessPieceType::Rook,
		Role::Queen => gameui::ChessPieceType::Queen,
		Role::King => gameui::ChessPieceType::King,
	};

	(colour,piece)
}