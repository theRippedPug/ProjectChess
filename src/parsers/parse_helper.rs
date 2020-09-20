//use ggez::{Context};

use super::{ChessPiece, ChessPieceType};
use chess;
use chess::{Board, Color, Piece, Square};

pub fn get_matrix(bd: &Board) -> [[Option<ChessPiece>; 8]; 8] {
	[
		[
			resolve_piece(bd.piece_on(Square::A1), bd.color_on(Square::A1)),
			resolve_piece(bd.piece_on(Square::B1), bd.color_on(Square::B1)),
			resolve_piece(bd.piece_on(Square::C1), bd.color_on(Square::C1)),
			resolve_piece(bd.piece_on(Square::D1), bd.color_on(Square::D1)),
			resolve_piece(bd.piece_on(Square::E1), bd.color_on(Square::E1)),
			resolve_piece(bd.piece_on(Square::F1), bd.color_on(Square::F1)),
			resolve_piece(bd.piece_on(Square::G1), bd.color_on(Square::G1)),
			resolve_piece(bd.piece_on(Square::H1), bd.color_on(Square::H1)),
		],
		[
			resolve_piece(bd.piece_on(Square::A2), bd.color_on(Square::A2)),
			resolve_piece(bd.piece_on(Square::B2), bd.color_on(Square::B2)),
			resolve_piece(bd.piece_on(Square::C2), bd.color_on(Square::C2)),
			resolve_piece(bd.piece_on(Square::D2), bd.color_on(Square::D2)),
			resolve_piece(bd.piece_on(Square::E2), bd.color_on(Square::E2)),
			resolve_piece(bd.piece_on(Square::F2), bd.color_on(Square::F2)),
			resolve_piece(bd.piece_on(Square::G2), bd.color_on(Square::G2)),
			resolve_piece(bd.piece_on(Square::H2), bd.color_on(Square::H2)),
		],
		[
			resolve_piece(bd.piece_on(Square::A3), bd.color_on(Square::A3)),
			resolve_piece(bd.piece_on(Square::B3), bd.color_on(Square::B3)),
			resolve_piece(bd.piece_on(Square::C3), bd.color_on(Square::C3)),
			resolve_piece(bd.piece_on(Square::D3), bd.color_on(Square::D3)),
			resolve_piece(bd.piece_on(Square::E3), bd.color_on(Square::E3)),
			resolve_piece(bd.piece_on(Square::F3), bd.color_on(Square::F3)),
			resolve_piece(bd.piece_on(Square::G3), bd.color_on(Square::G3)),
			resolve_piece(bd.piece_on(Square::H3), bd.color_on(Square::H3)),
		],
		[
			resolve_piece(bd.piece_on(Square::A4), bd.color_on(Square::A4)),
			resolve_piece(bd.piece_on(Square::B4), bd.color_on(Square::B4)),
			resolve_piece(bd.piece_on(Square::C4), bd.color_on(Square::C4)),
			resolve_piece(bd.piece_on(Square::D4), bd.color_on(Square::D4)),
			resolve_piece(bd.piece_on(Square::E4), bd.color_on(Square::E4)),
			resolve_piece(bd.piece_on(Square::F4), bd.color_on(Square::F4)),
			resolve_piece(bd.piece_on(Square::G4), bd.color_on(Square::G4)),
			resolve_piece(bd.piece_on(Square::H4), bd.color_on(Square::H4)),
		],
		[
			resolve_piece(bd.piece_on(Square::A5), bd.color_on(Square::A5)),
			resolve_piece(bd.piece_on(Square::B5), bd.color_on(Square::B5)),
			resolve_piece(bd.piece_on(Square::C5), bd.color_on(Square::C5)),
			resolve_piece(bd.piece_on(Square::D5), bd.color_on(Square::D5)),
			resolve_piece(bd.piece_on(Square::E5), bd.color_on(Square::E5)),
			resolve_piece(bd.piece_on(Square::F5), bd.color_on(Square::F5)),
			resolve_piece(bd.piece_on(Square::G5), bd.color_on(Square::G5)),
			resolve_piece(bd.piece_on(Square::H5), bd.color_on(Square::H5)),
		],
		[
			resolve_piece(bd.piece_on(Square::A6), bd.color_on(Square::A6)),
			resolve_piece(bd.piece_on(Square::B6), bd.color_on(Square::B6)),
			resolve_piece(bd.piece_on(Square::C6), bd.color_on(Square::C6)),
			resolve_piece(bd.piece_on(Square::D6), bd.color_on(Square::D6)),
			resolve_piece(bd.piece_on(Square::E6), bd.color_on(Square::E6)),
			resolve_piece(bd.piece_on(Square::F6), bd.color_on(Square::F6)),
			resolve_piece(bd.piece_on(Square::G6), bd.color_on(Square::G6)),
			resolve_piece(bd.piece_on(Square::H6), bd.color_on(Square::H6)),
		],
		[
			resolve_piece(bd.piece_on(Square::A7), bd.color_on(Square::A7)),
			resolve_piece(bd.piece_on(Square::B7), bd.color_on(Square::B7)),
			resolve_piece(bd.piece_on(Square::C7), bd.color_on(Square::C7)),
			resolve_piece(bd.piece_on(Square::D7), bd.color_on(Square::D7)),
			resolve_piece(bd.piece_on(Square::E7), bd.color_on(Square::E7)),
			resolve_piece(bd.piece_on(Square::F7), bd.color_on(Square::F7)),
			resolve_piece(bd.piece_on(Square::G7), bd.color_on(Square::G7)),
			resolve_piece(bd.piece_on(Square::H7), bd.color_on(Square::H7)),
		],
		[
			resolve_piece(bd.piece_on(Square::A8), bd.color_on(Square::A8)),
			resolve_piece(bd.piece_on(Square::B8), bd.color_on(Square::B8)),
			resolve_piece(bd.piece_on(Square::C8), bd.color_on(Square::C8)),
			resolve_piece(bd.piece_on(Square::D8), bd.color_on(Square::D8)),
			resolve_piece(bd.piece_on(Square::E8), bd.color_on(Square::E8)),
			resolve_piece(bd.piece_on(Square::F8), bd.color_on(Square::F8)),
			resolve_piece(bd.piece_on(Square::G8), bd.color_on(Square::G8)),
			resolve_piece(bd.piece_on(Square::H8), bd.color_on(Square::H8)),
		],
	]
}

fn resolve_piece(piece_type: Option<Piece>, piece_color: Option<Color>) -> Option<ChessPiece> {
	match piece_type {
		None => None,
		Some(piece) => Some(get_type(piece, piece_color.unwrap())),
	}
}

fn get_type(tp: Piece, col: Color) -> ChessPiece {
	let colour: char = match col {
		Color::Black => 'b',
		Color::White => 'w',
	};

	let piece = match tp {
		Piece::Pawn => ChessPieceType::Pawn,
		Piece::Bishop => ChessPieceType::Bishop,
		Piece::Knight => ChessPieceType::Knight,
		Piece::Rook => ChessPieceType::Rook,
		Piece::Queen => ChessPieceType::Queen,
		Piece::King => ChessPieceType::King,
	};

	ChessPiece {
		piece_color: colour,
		piece_type: piece,
	}
}
