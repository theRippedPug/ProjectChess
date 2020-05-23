//use ggez::{Context};
use shakmaty::{Board,Role,Color,Piece,Square};
use super::{ChessPieceType,ChessPiece};

pub fn getMatrix(bd : &Board) -> [[Option<ChessPiece>;8];8]{
	[
		[
			resolvePiece(bd.piece_at(Square::A8)),resolvePiece(bd.piece_at(Square::B8)),
			resolvePiece(bd.piece_at(Square::C8)),resolvePiece(bd.piece_at(Square::D8)),
			resolvePiece(bd.piece_at(Square::E8)),resolvePiece(bd.piece_at(Square::F8)),
			resolvePiece(bd.piece_at(Square::G8)),resolvePiece(bd.piece_at(Square::H8))
		],
		[
			resolvePiece(bd.piece_at(Square::A7)),resolvePiece(bd.piece_at(Square::B7)),
			resolvePiece(bd.piece_at(Square::C7)),resolvePiece(bd.piece_at(Square::D7)),
			resolvePiece(bd.piece_at(Square::E7)),resolvePiece(bd.piece_at(Square::F7)),
			resolvePiece(bd.piece_at(Square::G7)),resolvePiece(bd.piece_at(Square::H7))
		],
		[
			resolvePiece(bd.piece_at(Square::A6)),resolvePiece(bd.piece_at(Square::B6)),
			resolvePiece(bd.piece_at(Square::C6)),resolvePiece(bd.piece_at(Square::D6)),
			resolvePiece(bd.piece_at(Square::E6)),resolvePiece(bd.piece_at(Square::F6)),
			resolvePiece(bd.piece_at(Square::G6)),resolvePiece(bd.piece_at(Square::H6))
		],
		[
			resolvePiece(bd.piece_at(Square::A5)),resolvePiece(bd.piece_at(Square::B5)),
			resolvePiece(bd.piece_at(Square::C5)),resolvePiece(bd.piece_at(Square::D5)),
			resolvePiece(bd.piece_at(Square::E5)),resolvePiece(bd.piece_at(Square::F5)),
			resolvePiece(bd.piece_at(Square::G5)),resolvePiece(bd.piece_at(Square::H5))
		],
		[
			resolvePiece(bd.piece_at(Square::A4)),resolvePiece(bd.piece_at(Square::B4)),
			resolvePiece(bd.piece_at(Square::C4)),resolvePiece(bd.piece_at(Square::D4)),
			resolvePiece(bd.piece_at(Square::E4)),resolvePiece(bd.piece_at(Square::F4)),
			resolvePiece(bd.piece_at(Square::G4)),resolvePiece(bd.piece_at(Square::H4))
		],
		[
			resolvePiece(bd.piece_at(Square::A3)),resolvePiece(bd.piece_at(Square::B3)),
			resolvePiece(bd.piece_at(Square::C3)),resolvePiece(bd.piece_at(Square::D3)),
			resolvePiece(bd.piece_at(Square::E3)),resolvePiece(bd.piece_at(Square::F3)),
			resolvePiece(bd.piece_at(Square::G3)),resolvePiece(bd.piece_at(Square::H3))
		],
		[
			resolvePiece(bd.piece_at(Square::A2)),resolvePiece(bd.piece_at(Square::B2)),
			resolvePiece(bd.piece_at(Square::C2)),resolvePiece(bd.piece_at(Square::D2)),
			resolvePiece(bd.piece_at(Square::E2)),resolvePiece(bd.piece_at(Square::F2)),
			resolvePiece(bd.piece_at(Square::G2)),resolvePiece(bd.piece_at(Square::H2))
		],
		[
			resolvePiece(bd.piece_at(Square::A1)),resolvePiece(bd.piece_at(Square::B1)),
			resolvePiece(bd.piece_at(Square::C1)),resolvePiece(bd.piece_at(Square::D1)),
			resolvePiece(bd.piece_at(Square::E1)),resolvePiece(bd.piece_at(Square::F1)),
			resolvePiece(bd.piece_at(Square::G1)),resolvePiece(bd.piece_at(Square::H1))
		]
	]
}

fn resolvePiece(pc: Option<Piece>) -> Option<ChessPiece>{
	match pc{
		None => None,
		Some(piece) => Some(getType(piece))
	}

}

fn getType(tp: shakmaty::Piece) -> ChessPiece {
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

	ChessPiece{
		piece_color: colour,
		piece_type: piece
	}
}