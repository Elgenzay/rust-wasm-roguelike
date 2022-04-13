
pub mod world {

	pub mod room {
		//use rand::Rng;
		use crate::engine::engine::Coordinate;

		/// Percent of room length_width_sum to deviate from 50 when rolling a
		/// random number to determine room dimensions.
		///
		/// At 0, all rooms are square.
		///
		/// At 50, rooms can be a single unit tall or wide.
		const ROOM_SQUARE_DEVIATION_THRESHOLD: i8 = 20;

		pub struct Room {
			pub width: i32,
			pub height: i32,
			pub position: Coordinate,
		}

		impl Room {
			pub fn overlaps_coordinate(&self, coord: Coordinate) -> bool {
				coord.y < self.position.y + self.height &&
				coord.y > self.position.y &&
				coord.x < self.position.x + self.width &&
				coord.x > self.position.x
			}
		}
	}

	pub mod area {
		use std::collections::HashMap;
		use rand::Rng;
		use crate::engine::engine::Coordinate;
		use crate::room::Room;

		pub struct Area {
			pub map: HashMap<i32, HashMap<i32, Tile>>,
			pub default_fill: Option<WorldObject>,
		}

		impl Area {
			pub fn new(default_fill: Option<WorldObject>) -> Area {
				Area {
					map: HashMap::new(),
					default_fill,
				}
			}

			pub fn place_room(&mut self, room: &Room){
				for x in room.position.x..room.position.x+room.width {
					self.set_tile(x, room.position.y, Tile::wall());
					self.set_tile(x, room.position.y + room.width, Tile::wall());
				}
				for y in room.position.y..=room.position.y+room.height {
					self.set_tile(room.position.x, y, Tile::wall());
					self.set_tile(room.position.x + room.width, y, Tile::wall());
				}
			}
			
			pub fn create_room_hallway(&mut self, room_1: &Room, room_2: &Room) -> bool {
				enum Hallway{
					STRAIGHT(
						bool, // true if vertical
						i32,  // y position if horizontal, or x position if vertical
						i32,  // starting (left x/bottom y) position
						i32   // ending (right x/top y) position
					),
					BENT (
						BoxCorner,  // orientation
						Coordinate, // turning point
						i32,        // horizontal distance
						i32         // vertical distance
					)
				}
				#[derive(Copy, Clone)]
				enum BoxCorner{
					BottomRight, // ┘
					BottomLeft,  // └
					TopRight,    // ┐
					TopLeft,     // ┌
				}

				let rooms = if room_2.position.y > room_1.position.y {
					[room_1, room_2]
				} else {
					[room_2, room_1]
				};
				let room_1_top = rooms[0].position.y + rooms[0].height;
				let room_2_top = rooms[1].position.y + rooms[1].height;
				let room_2_edge = rooms[1].position.x + rooms[1].width;
				let room_1_edge = rooms[0].position.x + rooms[0].width;
				let mut possible_hallways = vec![];
				let (vertical, horizontal) = if room_1_top >= rooms[1].position.y + 2 && room_2_top >= rooms[0].position.y + 2 {
					(false, true)
				} else if room_1_edge >= rooms[1].position.x + 2 && room_2_edge >= rooms[0].position.x + 2 {
					(true, false)
				} else {
					(false, false)
				};
				if vertical || horizontal {
					let (one_t_e, two_y_x, two_t_e, one_y_x, one_x_y, two_x_y, one_e_t, two_e_t) = if horizontal {
						(room_1_top, rooms[1].position.y, room_2_top, rooms[0].position.y, rooms[0].position.x, rooms[1].position.x, room_1_edge, room_2_edge)
					} else {
						(room_1_edge, rooms[1].position.x, room_2_edge, rooms[0].position.x, rooms[0].position.y, rooms[1].position.y, room_1_top, room_2_top)
					};
					let starting_xy = if one_y_x > two_y_x {
						one_y_x
					} else {
						two_y_x
					};
					let ending_xy = if one_t_e > two_t_e {
						two_t_e
					} else {
						one_t_e
					};
					for y in starting_xy+1..ending_xy {
						possible_hallways.push(Hallway::STRAIGHT(
								vertical,
								y,
								if one_x_y > two_x_y{two_e_t} else {one_e_t},
								if one_x_y > two_x_y{one_x_y} else {two_x_y},
							)
						);
					}
				} else {
					let possible_orientations = if rooms[0].position.x < rooms[1].position.x{
						[
							BoxCorner::TopLeft,
							BoxCorner::BottomRight,
						]
					} else {
						[
							BoxCorner::BottomLeft,
							BoxCorner::TopRight,
						]
					};

					for orientation in possible_orientations {
						let (
							start_x, end_x,
							start_y, end_y
						) = match orientation {
							BoxCorner::BottomLeft | BoxCorner::BottomRight => (
								rooms[1].position.x, room_2_edge,
								rooms[0].position.y, room_1_top,
							),
							BoxCorner::TopLeft | BoxCorner::TopRight => (
								rooms[0].position.x, room_1_edge,
								rooms[1].position.y, room_2_top
							),
						};
						for y in start_y+1..end_y {
							for x in start_x+1..end_x {
								if rooms[1].overlaps_coordinate(Coordinate::new(x,y)) || rooms[0].overlaps_coordinate(Coordinate::new(x,y)) {
									continue;
								}
								let (horizontal_distance, vertical_distance) = match orientation {
									BoxCorner::BottomLeft => (
										rooms[0].position.x - x,
										rooms[1].position.y - y,
									),
									BoxCorner::BottomRight => (
										x - room_1_edge,
										rooms[1].position.y - y,
									),
									BoxCorner::TopRight => (
										x - room_2_edge,
										y - room_1_top,
									),
									BoxCorner::TopLeft => (
										rooms[1].position.x - x,
										y - room_1_top,
									),
								};
								possible_hallways.push(Hallway::BENT(
										orientation,
										Coordinate {x,y},
										horizontal_distance,
										vertical_distance,
									)
								);
							}
						}
					}
				}
				let	mut valid_hallways = vec![];
				for hallway in possible_hallways {
					let is_valid = match hallway {
						Hallway::STRAIGHT(is_vertical, x_y, start, end) => {
							if is_vertical {
								self.region_is_empty(Coordinate::new(x_y, start+1), Coordinate::new(x_y, end-1))
							} else {
								self.region_is_empty(Coordinate::new(start+1, x_y), Coordinate::new(end-1, x_y))
							}
						},
						Hallway::BENT(orientation, point, d_hor, d_ver) => {
							self.region_is_empty(
								point,
								Coordinate::new(
									match orientation {
										BoxCorner::BottomRight | BoxCorner::TopRight => point.x - d_hor + 1,
										BoxCorner::BottomLeft | BoxCorner::TopLeft => point.x + d_hor - 1,
									},
									point.y,
								)
							)
							&&
							self.region_is_empty(
								point,
								Coordinate::new(
									point.x,
									match orientation {
										BoxCorner::TopRight | BoxCorner::TopLeft => point.y - d_ver + 1,
										BoxCorner::BottomRight | BoxCorner::BottomLeft => point.y + d_ver - 1,
									},
								)
							)
						}
					};
					if is_valid {
						valid_hallways.push(hallway);
					}
				}
				if valid_hallways.len() == 0 {
					return false;
				}
				let hallway = valid_hallways.get(rand::thread_rng().gen_range(0..valid_hallways.len())).unwrap();
				match *hallway {
					Hallway::STRAIGHT(is_vertical, x_y, start, end) => {
						self.fill(
							Coordinate::new(
								if is_vertical {x_y-1} else {start},
								if is_vertical {start} else {x_y-1},
							),
							Coordinate::new(
								if is_vertical {x_y+1} else {end},
								if is_vertical {end} else {x_y+1},
							),
							Tile::wall()
						);
						self.fill(
							Coordinate::new(
								if is_vertical {x_y} else {start},
								if is_vertical {start} else {x_y},
							),
							Coordinate::new(
								if is_vertical {x_y} else {end},
								if is_vertical {end} else {x_y},
							),
							Tile::new(None)
						);
					},
					Hallway::BENT(orientation, point, d_hor, d_ver) => {
						let x = match orientation {
							BoxCorner::BottomRight | BoxCorner::TopRight => point.x - d_hor,
							BoxCorner::BottomLeft | BoxCorner::TopLeft => point.x + d_hor,
						};
						let y = match orientation {
							BoxCorner::TopRight | BoxCorner::TopLeft => point.y - d_ver,
							BoxCorner::BottomRight | BoxCorner::BottomLeft => point.y + d_ver,
						};

						self.fill(
							Coordinate::new(
								point.x,
								point.y-1,
							),
							Coordinate::new(
								x,
								point.y+1,
							),
							Tile::wall()
						);
						self.fill(
							Coordinate::new(
								point.x-1,
								point.y,
							),
							Coordinate::new(
								point.x+1,
								y,
							),
							Tile::wall()
						);

						let (corner_x, corner_y) = match orientation {
							BoxCorner::TopLeft => (-1,1),
							BoxCorner::TopRight => (1,1),
							BoxCorner::BottomLeft => (-1,-1),
							BoxCorner::BottomRight => (1,-1),
						};
						self.set_tile(point.x + corner_x, point.y + corner_y, Tile::wall());
						
						self.fill(
							point,
							Coordinate::new(
								x,
								point.y,
							),
							Tile::new(None)
						);
						self.fill(
							point,
							Coordinate::new(
								point.x,
								y,
							),
							Tile::new(None)
						);
					}
				};
				true
			}

			pub fn get_tile_at<X: Into<i32>, Y: Into<i32>>(&self, x: X, y: Y) -> Tile {
				let x_i32 = &x.into();
				let y_i32 = &y.into();

				let x_col = match self.map.get(x_i32) {
					Some(x_val) => x_val,
					None => {
						return match &self.default_fill {
							Some(obj) => Tile {contents:vec![*obj]},
							None => Tile {contents:vec![]}
						};
					}
				};

				match x_col.get(y_i32) {
					Some(tile) => tile.clone(),
					None => match &self.default_fill {
						Some(obj) => Tile {contents:vec![*obj]},
						None => Tile {contents:vec![]}
					},
				}
			}

			pub fn set_tile<X: Into<i32>, Y: Into<i32>>(&mut self, x: X, y: Y, t: Tile) {
				let x_i32 = x.into();
				let y_i32 = y.into();

				let x_col = match self.map.get_mut(&x_i32) {
					Some(x_val) => x_val,
					None => {
						self.map.insert(x_i32, HashMap::new());
						self.map.get_mut(&x_i32).unwrap()
					}
				};
				x_col.insert(y_i32, t);
			}

			pub fn region_is_empty(&self, coord_1: Coordinate, coord_2: Coordinate) -> bool{
				let coords = crate::render::canvas::sort_box_coordinates(coord_1, coord_2);
				for x in coords[0].x..=coords[1].x {
					for y in coords[0].y..=coords[1].y {
						if self.get_tile_at(x, y).contents.len() != 0 {
							return false;
						}
					}
				}
				true
			}

			pub fn fill(&mut self, coord_1: Coordinate, coord_2: Coordinate, tile: Tile) {
				let coords = crate::render::canvas::sort_box_coordinates(coord_1, coord_2);
				for x in coords[0].x..=coords[1].x {
					for y in coords[0].y..=coords[1].y {
						self.set_tile(x, y, tile.clone());
					}
				}
			}

		}

		#[derive(Clone)]
		pub struct Tile {
			pub contents: Vec<WorldObject>,
		}

		impl Tile {
			pub fn new(tile: Option<WorldObject>) -> Tile {
				Tile { contents: match tile {
						Some(obj) => vec![obj],
						None => vec![],
					}
				}
			}

			pub fn wall() -> Tile {
				Tile::new(Some(WorldObject::WALL))
			}

			pub fn get_char(&self) -> char {
				for obj in &self.contents {
					match obj.get_char() {
						Some(c) => return c,
						None => {}
					}
				}
				' '
			}
		}

		#[derive(Copy, Clone)]
		pub enum WorldObject {
			PLAYER,
			WALL,
		}

		impl WorldObject {
			fn get_char(&self) -> Option<char> {
				match &self {
					WorldObject::PLAYER => Some('O'),
					WorldObject::WALL => Some('█'),
				}
			}
		}
	}
}


//static MIN_DUNGEON_ROOM_SIZE: i32 = 8;

//			pub fn new_dungeon<W: Into<i32>, H: Into<i32>>(width: W, height: H) -> Area {
//				let width_i32 = width.into();
//				let height_i32 = height.into();
//				let coord_x = 0 - (width_i32 / 2);
//				let coord_y = 0 - (height_i32 / 2);
//				let new_width_1 = rand::thread_rng().gen_range(MIN_DUNGEON_ROOM_SIZE..(width_i32-MIN_DUNGEON_ROOM_SIZE));
//				let new_height_1 = rand::thread_rng().gen_range(MIN_DUNGEON_ROOM_SIZE..(height_i32-MIN_DUNGEON_ROOM_SIZE));
//				let new_width_2 = width_i32 - new_width_1;
//				let new_height_2 = height_i32 - new_height_1;
//			}
//
//			fn split(c_x: i32, c_y: i32, w: i32, h: i32) -> [(i32,i32,i32,i32); 2]{
//
//			}
		