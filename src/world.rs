
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
			//	pub fn new(size: RoomSize, position: Coordinate) -> Room {
			//		let length_width_sum: i32 = match size {
			//			RoomSize::SMALL => 8,
			//			RoomSize::MEDIUM => 16,
			//			RoomSize::LARGE => 32,
			//			RoomSize::CUSTOM(a) => a,
			//		};
			//		let deviation: i32 = (length_width_sum as f32
			//			* (ROOM_SQUARE_DEVIATION_THRESHOLD as f32 * 0.01)) as i32;
			//		if deviation == 0 {
			//			return Room {
			//				width: length_width_sum / 2,
			//				height: length_width_sum / 2,
			//			};
			//		}
			//		let min = (length_width_sum / 2) - deviation;
			//		let max = (length_width_sum / 2) + deviation;
			//		let width = rand::thread_rng().gen_range(min..max);
			//		let height = length_width_sum - width;
			//		if rand::random() {
			//			return Room { width, height };
			//		}
			//		Room {
			//			width: height,
			//			height: width,
			//		}
			//	}
			//}
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
					self.set_tile(x, room.position.y, Tile::new(Some(WorldObject::WALL)));
					self.set_tile(x, room.position.y + room.width, Tile::new(Some(WorldObject::WALL)));
				}
				for y in room.position.y..=room.position.y+room.height {
					self.set_tile(room.position.x, y, Tile::new(Some(WorldObject::WALL)));
					self.set_tile(room.position.x + room.width, y, Tile::new(Some(WorldObject::WALL)));
				}
			}
			
			pub fn create_room_hallway(&mut self, room_1: &Room, room_2: &Room){
				let room_coords_sorted = crate::render::canvas::sort_box_coordinates(
					room_1.position,
					room_2.position
				);
				let rooms = if room_coords_sorted[0] == room_1.position {
					[room_1, room_2]
				} else {
					[room_2, room_1]
				};
				let mut possible_hallways = vec![];
				for right_first in [true, false] {
					let mut one_pos_y = rooms[0].position.y;
					let mut one_height = rooms[0].position.y + rooms[0].height;
					let mut two_pos_x = rooms[1].position.x;
					let mut two_width = rooms[1].position.x + rooms[1].width;
					let mut two_pos_y = rooms[1].position.y;
					let mut one_width = rooms[0].position.x + rooms[0].width;
					
					if !right_first {
						one_pos_y = rooms[0].position.x;
						one_height = rooms[0].position.x + rooms[0].width;
						two_pos_x = rooms[1].position.y;
						two_width = rooms[1].position.y + rooms[1].height;
						two_pos_y = rooms[1].position.x;
						one_width = rooms[0].position.y + rooms[0].height;
					}
					
					for i in one_pos_y+1..one_height {
						for j in two_pos_x+1..two_width {
							if self.region_is_empty(
								Coordinate::new(one_width+1, i-1),
								Coordinate::new(j+1,i+1)
							) && self.region_is_empty(
								Coordinate::new(j-1,i+1),
								Coordinate::new(j+1, two_pos_y-1)
							) {
								possible_hallways.push((
									right_first,
									one_width+1, i-1, j+1, i+1,
									j-1, i+1, j+1, two_pos_y-1,
								));
							}
						}
					}
				}
				let h = possible_hallways.get(rand::thread_rng().gen_range(0..possible_hallways.len())).unwrap();
				if h.0 {
					self.fill(
						Coordinate::new(h.1, h.2),
						Coordinate::new(h.3, h.4),
						Tile::new(Some(WorldObject::WALL))
					);
					self.fill(
						Coordinate::new(h.5, h.6),
						Coordinate::new(h.7, h.8),
						Tile::new(Some(WorldObject::WALL))
					);
					self.fill(
						Coordinate::new(h.1-1, h.2+1),
						Coordinate::new(h.3-1, h.4-1),
						Tile::new(None)
					);
					self.fill(
						Coordinate::new(h.5+1, h.6),
						Coordinate::new(h.7-1, h.8+1),
						Tile::new(None)
					);
				} else {
					self.fill(
						Coordinate::new(h.2, h.1),
						Coordinate::new(h.4, h.3),
						Tile::new(Some(WorldObject::WALL))
					);
					self.fill(
						Coordinate::new(h.6, h.5),
						Coordinate::new(h.8, h.7),
						Tile::new(Some(WorldObject::WALL))
					);
					self.fill(
						Coordinate::new(h.2+1, h.1-1),
						Coordinate::new(h.4-1, h.3-1),
						Tile::new(None)
					);
					self.fill(
						Coordinate::new(h.6, h.5+1),
						Coordinate::new(h.8+1, h.7-1),
						Tile::new(None)
					);
				}
			}

			fn swap_if_true<T>(is_true: bool, tuple:(T,T)) -> (T,T){
				if is_true {
					return (tuple.1, tuple.0);
				}
				tuple
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
		