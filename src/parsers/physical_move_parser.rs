use super::*;

#[derive(Debug, Clone)]
struct PathNode {
	pos: PhysicalPos,
	penalty: u64, // basically the distance but takes other factors into account
	path: Vec<PhysicalPos>,
	penalty_reset: Vec<PhysicalPos>, // the positions that have to be reset
}
impl PartialEq for PathNode {
	fn eq(&self, other: &Self) -> bool {
		self.pos == other.pos
	}
}

lazy_static! {
	static ref reverve_pieces: std::sync::Mutex<i8> = std::sync::Mutex::new(5);
}

pub fn parse(
	board: BoardRep,
	move_list: Vec<AbstractMove>,
) -> Result<Vec<(Vec<PhysicalPos>, Vec<PhysicalPos>)>, ParseError> {
	
	let mut result = Vec::new();

	for mov in move_list {
		//deal with castling
	let physical_move_list: Vec<PhysicalPos> = Vec::new();
	let reset_positions: Vec<PhysicalPos> = Vec::new();

		match mov {
			AbstractMove::Catstling((_p1, _p2)) => {}
			AbstractMove::RegularMove(mv) => {
				match mv.origin {
					PieceOrigin::Existing(start_pos) => {
						match mv.dest {
							PieceDest::OnBoard(end_pos) => {
								let start_phys_pos = (start_pos.0 as f32, start_pos.1 as f32);
								let end_phys_pos = (end_pos.0 as f32, end_pos.1 as f32);
								// Move from board to board
								if start_pos.0 == end_pos.0 || start_pos.1 == end_pos.1 {
									//the move is linear
									result.push((
										vec![start_phys_pos, end_phys_pos],
										Vec::<PhysicalPos>::new(),
									));
								} else if i8::abs(start_pos.0 as i8 - end_pos.0 as i8)
									== i8::abs(start_pos.1 as i8 - end_pos.1 as i8)
								{
									//its a diagonal move
									result.push((
										vec![start_phys_pos, end_phys_pos],
										Vec::<PhysicalPos>::new(),
									));
								}else{
								//ok so this is definitely a horse move
								let delta = (
									end_phys_pos.0 - start_phys_pos.0,
									end_phys_pos.1 - start_phys_pos.1,
								);
								let delta_dir = (delta.0.signum(), delta.1.signum());

								let delta_weight: (f32, f32) = if delta.0.abs() > delta.1.abs() {
									(1.0, 0.0)
								} else {
									(0.0, 1.0)
								};

								let move1 = (
									start_phys_pos.0 + 0.5 * delta_dir.0,
									start_phys_pos.1 + 0.5 * delta_dir.1,
								);
								let move2 = (
									move1.0 + (delta_dir.0 * delta_weight.0),
									move1.1 + (delta_dir.1 * delta_weight.1),
								);
								let move3 =
									(move2.0 + 0.5 * delta_dir.0, move2.1 + 0.5 * delta_dir.1);

								let move_list = vec![start_phys_pos, move1, move2, move3];

								//ok moves done now reset positions

								let mut resetlist = Vec::new();

								//positions where if pawns are present have to be reset
								//NOTE: code was written when among us was peak
								let mut sus_list = Vec::new();
								sus_list.push((start_phys_pos.0 + delta_dir.0, start_phys_pos.1));
								sus_list.push((start_phys_pos.0, start_phys_pos.1 + delta_dir.1));
								sus_list.push((
									start_phys_pos.0 + delta_dir.0,
									start_phys_pos.1 + delta_dir.1,
								));

								sus_list.push((
									start_phys_pos.0 + (2.0 * delta_weight.0 * delta_dir.0),
									start_phys_pos.1 + (2.0 * delta_weight.1 * delta_dir.1),
								));

								for pos in sus_list {
									if board[pos.1 as usize][pos.0 as usize].is_some() {
										resetlist.push(pos);
									}
								}

								result.push((move_list, resetlist));
							}
							}
							PieceDest::Disposed => {
								// From board to bin
								result.push(dijkstra_path(board, start_pos));
							}
						}
					}
					PieceOrigin::Reserve => {
						match mv.dest {
							PieceDest::OnBoard(end_pos) => {
								// Move from pawn reserve to board
								//TODO
								return Err(ParseError {});
							}
							PieceDest::Disposed => {
								// Why would you move a piece from reverve straight to to the bin??
								return Err(ParseError {});
							}
						}
					}
				}
			}
		}
	}
	Ok(result)
}

// This is a modified dejkstara implementation thats too comple to explain in comments
//checkout the dijkstra file in MDocs folder for detailed explanation
fn dijkstra_path(board: BoardRep, start_pos: AbstractPos) -> (Vec<PhysicalPos>, Vec<PhysicalPos>) {
	let mut board = board.clone();
	board[start_pos.1 as usize][start_pos.0 as usize] = None;

	let mut node_list: Vec<PathNode> = Vec::new();
	node_list.push(PathNode {
		pos: (start_pos.0 as f32, start_pos.1 as f32),
		path: Vec::new(),
		penalty: 0,
		penalty_reset: Vec::new(),
	});
	loop {
		//Step 1 (termination condition):
		if node_list[0].pos == (-1.0, 3.0) {
			let final_node = node_list[0].clone();
			break (final_node.path, final_node.penalty_reset);
		}

		//Step 3:
		let first_node = node_list.remove(0);

		//Step 4:
		node_list.append(&mut find_surround_nodes(&board, first_node));

		//Step 5:
		node_list.sort_unstable_by_key(|a| a.penalty + (a.pos.0 * 4.0) as u64);

		//Step 6:
		deduplicate(&mut node_list);
	}
}

//Step 2:
fn find_surround_nodes(board: &BoardRep, node: PathNode) -> Vec<PathNode> {
	let mut surround_nodes: Vec<PathNode> = Vec::new();

	//if there is space left to explore nodes at the following directions
	let top = if node.pos.1 == 7.0 { false } else { true };
	let botm = if node.pos.1 == 0.0 { false } else { true };
	let left = if node.pos.0 == 0.0 { false } else { true };
	let right = if node.pos.0 == 7.0 { false } else { true };

	//if node is on a line between two squares and not on the corner, disable diagonal search
	// note-to-self the funny liiking oprator below is XOR
	let diagonals_allowed = if (node.pos.0.fract() != 0.0) ^ (node.pos.0.fract() != 0.0) {
		false
	} else {
		true
	};

	if !left {
		let mut new_node = node.clone();
		new_node.path.push(node.pos);
		new_node.path.push((-1.0, node.pos.1));

		new_node.pos = (-1.0, 3.0);

		surround_nodes.push(new_node);
	}
	if top {
		let new_pos = (node.pos.0, node.pos.1 + 0.5);
		if new_pos.1.fract() == 0.0
			&& new_pos.0.fract() == 0.0
			&& board[new_pos.1 as usize][new_pos.0 as usize].is_some()
		{
		} else {
			let mut new_node = node.clone();
			new_node.path.push(node.pos);
			new_node.penalty += 2;

			let (pawn_pen, mut resets) = get_ambient_pieces(board, new_pos);

			new_node.penalty += pawn_pen as u64 * 10;
			new_node.penalty_reset.append(&mut resets);

			new_node.pos = new_pos;

			surround_nodes.push(new_node);
		}
	}
	if botm {
		let new_pos = (node.pos.0, node.pos.1 - 0.5);
		if new_pos.1.fract() == 0.0
			&& new_pos.0.fract() == 0.0
			&& board[new_pos.1 as usize][new_pos.0 as usize].is_some()
		{
		} else {
			let mut new_node = node.clone();
			new_node.path.push(node.pos);
			new_node.penalty += 2;

			let (pawn_pen, mut resets) = get_ambient_pieces(board, new_pos);

			new_node.penalty += pawn_pen as u64 * 10;
			new_node.penalty_reset.append(&mut resets);

			new_node.pos = new_pos;

			surround_nodes.push(new_node);
		}
	}
	if left {
		let new_pos = (node.pos.0 - 0.5, node.pos.1);
		if new_pos.1.fract() == 0.0
			&& new_pos.0.fract() == 0.0
			&& board[new_pos.1 as usize][new_pos.0 as usize].is_some()
		{
		} else {
			let mut new_node = node.clone();
			new_node.path.push(node.pos);
			new_node.penalty += 2;

			let (pawn_pen, mut resets) = get_ambient_pieces(board, new_pos);

			new_node.penalty += pawn_pen as u64 * 10;
			new_node.penalty_reset.append(&mut resets);

			new_node.pos = new_pos;

			surround_nodes.push(new_node);
		}
	}
	if right {
		let new_pos = (node.pos.0 + 0.5, node.pos.1);
		if new_pos.1.fract() == 0.0
			&& new_pos.0.fract() == 0.0
			&& board[new_pos.1 as usize][new_pos.0 as usize].is_some()
		{
		} else {
			let mut new_node = node.clone();
			new_node.path.push(node.pos);
			new_node.penalty += 2;

			let (pawn_pen, mut resets) = get_ambient_pieces(board, new_pos);

			new_node.penalty += pawn_pen as u64 * 10;
			new_node.penalty_reset.append(&mut resets);

			new_node.pos = new_pos;

			surround_nodes.push(new_node);
		}
	}
	if diagonals_allowed {
		if top && right {
			let new_pos = (node.pos.0 + 0.5, node.pos.1 + 0.5);
			if new_pos.1.fract() == 0.0
				&& new_pos.0.fract() == 0.0
				&& board[new_pos.1 as usize][new_pos.0 as usize].is_some()
			{
			} else {
				let mut new_node = node.clone();
				new_node.path.push(node.pos);
				new_node.penalty += 3;

				let (pawn_pen, mut resets) = get_ambient_pieces(board, new_pos);

				new_node.penalty += pawn_pen as u64 * 10;
				new_node.penalty_reset.append(&mut resets);

				new_node.pos = new_pos;

				surround_nodes.push(new_node);
			}
		}
		if botm && left {
			let new_pos = (node.pos.0 - 0.5, node.pos.1 - 0.5);
			if new_pos.1.fract() == 0.0
				&& new_pos.0.fract() == 0.0
				&& board[new_pos.1 as usize][new_pos.0 as usize].is_some()
			{
			} else {
				let mut new_node = node.clone();
				new_node.path.push(node.pos);
				new_node.penalty += 3;

				let (pawn_pen, mut resets) = get_ambient_pieces(board, new_pos);

				new_node.penalty += pawn_pen as u64 * 10;
				new_node.penalty_reset.append(&mut resets);

				new_node.pos = new_pos;

				surround_nodes.push(new_node);
			}
		}
		if botm && right {
			let new_pos = (node.pos.0 + 0.5, node.pos.1 - 0.5);
			if new_pos.1.fract() == 0.0
				&& new_pos.0.fract() == 0.0
				&& board[new_pos.1 as usize][new_pos.0 as usize].is_some()
			{
			} else {
				let mut new_node = node.clone();
				new_node.path.push(node.pos);
				new_node.penalty += 3;

				let (pawn_pen, mut resets) = get_ambient_pieces(board, new_pos);

				new_node.penalty += pawn_pen as u64 * 10;
				new_node.penalty_reset.append(&mut resets);

				new_node.pos = new_pos;

				surround_nodes.push(new_node);
			}
		}
		if top && left {
			let new_pos = (node.pos.0 - 0.5, node.pos.1 + 0.5);
			if new_pos.1.fract() == 0.0
				&& new_pos.0.fract() == 0.0
				&& board[new_pos.1 as usize][new_pos.0 as usize].is_some()
			{
			} else {
				let mut new_node = node.clone();
				new_node.path.push(node.pos);
				new_node.penalty += 3;

				let (pawn_pen, mut resets) = get_ambient_pieces(board, new_pos);

				new_node.penalty += pawn_pen as u64 * 10;
				new_node.penalty_reset.append(&mut resets);

				new_node.pos = new_pos;

				surround_nodes.push(new_node);
			}
		}
	}
	surround_nodes
}

// fn try_get_node(board: &BoardRep, node: PathNode, pos: PhysicalPos) -> PathNode{

// }

fn deduplicate(array: &mut Vec<PathNode>) {
	let mut point_list: Vec<PhysicalPos> = Vec::new();

	array.retain(|x| {
		let x_pos = x.pos.clone();
		if point_list.contains(&x_pos) {
			return false;
		} else {
			point_list.push(x_pos);
			return true;
		}
	});
}

fn get_ambient_pieces(board: &BoardRep, pos: PhysicalPos) -> (u8, Vec<PhysicalPos>) {
	let mut pawn_count: u8 = 0;
	let mut reset_list: Vec<PhysicalPos> = Vec::new();

	//you are on a diagonal, check for pieces on all 4 sides
	if pos.1.fract() != 0.0 && pos.0.fract() != 0.0 {
		if board[pos.1.floor() as usize][pos.0.floor() as usize].is_some() {
			reset_list.push((pos.0.floor(), pos.1.floor()));
			pawn_count += 1;
		}
		if board[pos.1.floor() as usize][pos.0.ceil() as usize].is_some() {
			reset_list.push((pos.0.ceil(), pos.1.floor()));
			pawn_count += 1;
		}
		if board[pos.1.ceil() as usize][pos.0.floor() as usize].is_some() {
			reset_list.push((pos.0.floor(), pos.1.ceil()));
			pawn_count += 1;
		}
		if board[pos.1.ceil() as usize][pos.0.ceil() as usize].is_some() {
			reset_list.push((pos.0.ceil(), pos.1.ceil()));
			pawn_count += 1;
		}
	} else if pos.1.fract().abs() != 0.0 {
		if board[pos.1.floor() as usize][pos.0 as usize].is_some() {
			reset_list.push((pos.0, pos.1.floor()));
			pawn_count += 2;
		}
		if pos.1 < 7.0 {
			if board[pos.1.ceil() as usize][pos.0 as usize].is_some() {
				reset_list.push((pos.0, pos.1.ceil()));
				pawn_count += 2;
			}
		}
	} else if pos.0.fract().abs() != 0.0 {
		if board[pos.1 as usize][pos.0.floor() as usize].is_some() {
			reset_list.push((pos.0.floor(), pos.1));
			pawn_count += 2;
		}
		if pos.0 < 7.0 {
			if board[pos.1 as usize][pos.0.ceil() as usize].is_some() {
				reset_list.push((pos.0.ceil(), pos.1));
				pawn_count += 2;
			}
		}
	}

	(pawn_count, reset_list)
}
