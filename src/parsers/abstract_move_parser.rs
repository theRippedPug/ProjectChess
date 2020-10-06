// use chess::{Board, ChessMove, Color, Square};
use super::{
	AbstractMove, AbstractPos, BoardRep, ChessPieceType, ParseError, PieceCoord, PieceDest,
	PieceOrigin, RegularMove,
};

pub fn get_diff(board1: &BoardRep, board2: &BoardRep) -> Vec<PieceCoord> {
	let mut mismatch_list = Vec::<PieceCoord>::new();
	for i in 0..8 {
		for j in 0..8 {
			if board1[i][j] != board2[i][j] {
				mismatch_list.push((j, i))
			}
		}
	}
	mismatch_list
}

// NOTE: pos means position, board1 and board2 refer to inital and final
// configs of the board
pub fn parse(
	board1: BoardRep,
	board2: BoardRep,
	move_is_promotion: bool,
) -> Result<Vec<AbstractMove>, ParseError> {
	let diffs = get_diff(&board1, &board2);

	match diffs.len() {
		3 => {
			let mut finalvec = Vec::new();
			//it's el passaunt move
			// we need to find the pawn that moved
			// first iterate through the different squares on both boards (so nested loop)
			// till you find where the piece that moved went.
			'outer: for i in &diffs {
				// this check is to ensure the algorithm does'nt conclude that
				// it was the empty square that moved, we wanna get a piece ONLY
				if board1[i.1][i.0].is_some() {
					for j in &diffs {
						// so after this conditional, we have the same picece
						// in two different boards,BINGO! Now we add the generate
						// the move for the piece that does the capturing
						if board1[i.1][i.0] == board2[j.1][j.0] {
							finalvec.push(AbstractMove::RegularMove(RegularMove {
								origin: PieceOrigin::Existing((i.0 as u8, i.1 as u8)),
								dest: PieceDest::OnBoard((j.0 as u8, j.1 as u8)),
							}));
							// now we know piece at i(in board1) is the pawn that captures
							// time to find the one that gets captured
							for k in &diffs {
								// NOTE:the following conditional is written with respect
								// to board 1 positions
								// we know this piece is not the pice that captures
								// (so its not at i) also it's not an empty
								if k != i && board1[k.1][k.0].is_some() {
									// now we generate the remove the piece that gets captured
									finalvec.push(AbstractMove::RegularMove(RegularMove {
										origin: PieceOrigin::Existing((k.0 as u8, k.1 as u8)),
										dest: PieceDest::Disposed,
									}));
								}
							}
							break 'outer;
						}
					}
				}
			}
			Ok(finalvec)
		}
		2 => {
			if board1[diffs[0].1][diffs[0].0].is_none() || board1[diffs[1].1][diffs[1].0].is_none()
			{
				if move_is_promotion {
					//simple promotion
					// if piece at pos1 is the pawn, remove it and get one from reserve to occupy it's place
					if board1[diffs[0].1][diffs[0].0].unwrap().piece_type == ChessPieceType::Pawn {
						Ok(vec![
							AbstractMove::RegularMove(RegularMove {
								origin: PieceOrigin::Existing((diffs[0].0 as u8, diffs[0].1 as u8)),
								dest: PieceDest::Disposed,
							}),
							AbstractMove::RegularMove(RegularMove {
								origin: PieceOrigin::Reserve,
								dest: PieceDest::OnBoard((diffs[0].0 as u8, diffs[0].1 as u8)),
							}),
						])
					} else {
						Ok(vec![
							AbstractMove::RegularMove(RegularMove {
								origin: PieceOrigin::Existing((diffs[1].0 as u8, diffs[1].1 as u8)),
								dest: PieceDest::Disposed,
							}),
							AbstractMove::RegularMove(RegularMove {
								origin: PieceOrigin::Reserve,
								dest: PieceDest::OnBoard((diffs[1].0 as u8, diffs[1].1 as u8)),
							}),
						])
					}
				} else {
					//relocation
					// if pos 1 is empty in board1, move piece at pos2 to pos1 and vice versa
					if board1[diffs[0].1][diffs[0].0].is_none() {
						Ok(vec![AbstractMove::RegularMove(RegularMove {
							origin: PieceOrigin::Existing((diffs[1].0 as u8, diffs[1].1 as u8)),
							dest: PieceDest::OnBoard((diffs[0].0 as u8, diffs[0].1 as u8)),
						})])
					} else {
						Ok(vec![AbstractMove::RegularMove(RegularMove {
							origin: PieceOrigin::Existing((diffs[0].0 as u8, diffs[0].1 as u8)),
							dest: PieceDest::OnBoard((diffs[1].0 as u8, diffs[1].1 as u8)),
						})])
					}
				}
			} else if board1[diffs[0].1][diffs[0].0].unwrap().piece_color
				== board1[diffs[1].1][diffs[1].0].unwrap().piece_color
			{
				//castling
				Ok(vec![AbstractMove::Catstling((
					(diffs[0].0 as u8, diffs[0].1 as u8),
					(diffs[1].0 as u8, diffs[1].1 as u8),
				))])
			} else {
				if move_is_promotion {
					//cappture + promotion
					// same as promotion but first get rid of the piece thats no a pawn
					if board1[diffs[0].1][diffs[0].0].unwrap().piece_type == ChessPieceType::Pawn {
						Ok(vec![
							AbstractMove::RegularMove(RegularMove {
								origin: PieceOrigin::Existing((diffs[1].0 as u8, diffs[1].1 as u8)),
								dest: PieceDest::Disposed,
							}),
							AbstractMove::RegularMove(RegularMove {
								origin: PieceOrigin::Existing((diffs[0].0 as u8, diffs[0].1 as u8)),
								dest: PieceDest::Disposed,
							}),
							AbstractMove::RegularMove(RegularMove {
								origin: PieceOrigin::Reserve,
								dest: PieceDest::OnBoard((diffs[0].0 as u8, diffs[0].1 as u8)),
							}),
						])
					} else {
						Ok(vec![
							AbstractMove::RegularMove(RegularMove {
								origin: PieceOrigin::Existing((diffs[0].0 as u8, diffs[0].1 as u8)),
								dest: PieceDest::Disposed,
							}),
							AbstractMove::RegularMove(RegularMove {
								origin: PieceOrigin::Existing((diffs[1].0 as u8, diffs[1].1 as u8)),
								dest: PieceDest::Disposed,
							}),
							AbstractMove::RegularMove(RegularMove {
								origin: PieceOrigin::Reserve,
								dest: PieceDest::OnBoard((diffs[1].0 as u8, diffs[1].1 as u8)),
							}),
						])
					}
				} else {
					// capture only

					//logic is literally the same as en passaunt logic
					let mut finalvec = Vec::new();

					'outer2: for i in &diffs {
						if board1[i.1][i.0].is_some() {
							for j in &diffs {
								if board1[i.1][i.0] == board2[j.1][j.0] {
									finalvec.push(AbstractMove::RegularMove(RegularMove {
										origin: PieceOrigin::Existing((i.0 as u8, i.1 as u8)),
										dest: PieceDest::OnBoard((j.0 as u8, j.1 as u8)),
									}));
									for k in &diffs {
										if k != i && board1[k.1][k.0].is_some() {
											finalvec.push(AbstractMove::RegularMove(RegularMove {
												origin: PieceOrigin::Existing((
													k.0 as u8, k.1 as u8,
												)),
												dest: PieceDest::Disposed,
											}));
										}
									}
									break 'outer2;
								}
							}
						}
					}
					Ok(finalvec)
				}
			}
		}
		_ => Err(ParseError {}),
	}
}
