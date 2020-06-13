//use ggez::{Context};
use super::{ChessPiece, ChessPieceType};
use chess;
use chess::{Board, Color, Piece, Square};

pub fn getMatrix(bd: &Board) -> [[Option<ChessPiece>; 8]; 8] {
	[
		[
			resolvePiece(bd.piece_on(Square::A8), bd.color_on(Square::A8)),
			resolvePiece(bd.piece_on(Square::B8), bd.color_on(Square::B8)),
			resolvePiece(bd.piece_on(Square::C8), bd.color_on(Square::C8)),
			resolvePiece(bd.piece_on(Square::D8), bd.color_on(Square::D8)),
			resolvePiece(bd.piece_on(Square::E8), bd.color_on(Square::E8)),
			resolvePiece(bd.piece_on(Square::F8), bd.color_on(Square::F8)),
			resolvePiece(bd.piece_on(Square::G8), bd.color_on(Square::G8)),
			resolvePiece(bd.piece_on(Square::H8), bd.color_on(Square::H8)),
		],
		[
			resolvePiece(bd.piece_on(Square::A7), bd.color_on(Square::A7)),
			resolvePiece(bd.piece_on(Square::B7), bd.color_on(Square::B7)),
			resolvePiece(bd.piece_on(Square::C7), bd.color_on(Square::C7)),
			resolvePiece(bd.piece_on(Square::D7), bd.color_on(Square::D7)),
			resolvePiece(bd.piece_on(Square::E7), bd.color_on(Square::E7)),
			resolvePiece(bd.piece_on(Square::F7), bd.color_on(Square::F7)),
			resolvePiece(bd.piece_on(Square::G7), bd.color_on(Square::G7)),
			resolvePiece(bd.piece_on(Square::H7), bd.color_on(Square::H7)),
		],
		[
			resolvePiece(bd.piece_on(Square::A6), bd.color_on(Square::A6)),
			resolvePiece(bd.piece_on(Square::B6), bd.color_on(Square::B6)),
			resolvePiece(bd.piece_on(Square::C6), bd.color_on(Square::C6)),
			resolvePiece(bd.piece_on(Square::D6), bd.color_on(Square::D6)),
			resolvePiece(bd.piece_on(Square::E6), bd.color_on(Square::E6)),
			resolvePiece(bd.piece_on(Square::F6), bd.color_on(Square::F6)),
			resolvePiece(bd.piece_on(Square::G6), bd.color_on(Square::G6)),
			resolvePiece(bd.piece_on(Square::H6), bd.color_on(Square::H6)),
		],
		[
			resolvePiece(bd.piece_on(Square::A5), bd.color_on(Square::A5)),
			resolvePiece(bd.piece_on(Square::B5), bd.color_on(Square::B5)),
			resolvePiece(bd.piece_on(Square::C5), bd.color_on(Square::C5)),
			resolvePiece(bd.piece_on(Square::D5), bd.color_on(Square::D5)),
			resolvePiece(bd.piece_on(Square::E5), bd.color_on(Square::E5)),
			resolvePiece(bd.piece_on(Square::F5), bd.color_on(Square::F5)),
			resolvePiece(bd.piece_on(Square::G5), bd.color_on(Square::G5)),
			resolvePiece(bd.piece_on(Square::H5), bd.color_on(Square::H5)),
		],
		[
			resolvePiece(bd.piece_on(Square::A4), bd.color_on(Square::A4)),
			resolvePiece(bd.piece_on(Square::B4), bd.color_on(Square::B4)),
			resolvePiece(bd.piece_on(Square::C4), bd.color_on(Square::C4)),
			resolvePiece(bd.piece_on(Square::D4), bd.color_on(Square::D4)),
			resolvePiece(bd.piece_on(Square::E4), bd.color_on(Square::E4)),
			resolvePiece(bd.piece_on(Square::F4), bd.color_on(Square::F4)),
			resolvePiece(bd.piece_on(Square::G4), bd.color_on(Square::G4)),
			resolvePiece(bd.piece_on(Square::H4), bd.color_on(Square::H4)),
		],
		[
			resolvePiece(bd.piece_on(Square::A3), bd.color_on(Square::A3)),
			resolvePiece(bd.piece_on(Square::B3), bd.color_on(Square::B3)),
			resolvePiece(bd.piece_on(Square::C3), bd.color_on(Square::C3)),
			resolvePiece(bd.piece_on(Square::D3), bd.color_on(Square::D3)),
			resolvePiece(bd.piece_on(Square::E3), bd.color_on(Square::E3)),
			resolvePiece(bd.piece_on(Square::F3), bd.color_on(Square::F3)),
			resolvePiece(bd.piece_on(Square::G3), bd.color_on(Square::G3)),
			resolvePiece(bd.piece_on(Square::H3), bd.color_on(Square::H3)),
		],
		[
			resolvePiece(bd.piece_on(Square::A2), bd.color_on(Square::A2)),
			resolvePiece(bd.piece_on(Square::B2), bd.color_on(Square::B2)),
			resolvePiece(bd.piece_on(Square::C2), bd.color_on(Square::C2)),
			resolvePiece(bd.piece_on(Square::D2), bd.color_on(Square::D2)),
			resolvePiece(bd.piece_on(Square::E2), bd.color_on(Square::E2)),
			resolvePiece(bd.piece_on(Square::F2), bd.color_on(Square::F2)),
			resolvePiece(bd.piece_on(Square::G2), bd.color_on(Square::G2)),
			resolvePiece(bd.piece_on(Square::H2), bd.color_on(Square::H2)),
		],
		[
			resolvePiece(bd.piece_on(Square::A1), bd.color_on(Square::A1)),
			resolvePiece(bd.piece_on(Square::B1), bd.color_on(Square::B1)),
			resolvePiece(bd.piece_on(Square::C1), bd.color_on(Square::C1)),
			resolvePiece(bd.piece_on(Square::D1), bd.color_on(Square::D1)),
			resolvePiece(bd.piece_on(Square::E1), bd.color_on(Square::E1)),
			resolvePiece(bd.piece_on(Square::F1), bd.color_on(Square::F1)),
			resolvePiece(bd.piece_on(Square::G1), bd.color_on(Square::G1)),
			resolvePiece(bd.piece_on(Square::H1), bd.color_on(Square::H1)),
		],
	]
}

fn resolvePiece(pieceType: Option<Piece>, pieceColor: Option<Color>) -> Option<ChessPiece> {
	match pieceType {
		None => None,
		Some(piece) => Some(getType(piece, pieceColor.unwrap())),
	}
}

fn getType(tp: Piece, col: Color) -> ChessPiece {
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
