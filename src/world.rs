pub mod world {

	pub mod room {
		use rand::Rng;

		/// Percent of room length_width_sum to deviate from 50 when rolling a
		/// random number to determine room dimensions.
		///
		/// At 0, all rooms are square.
		///
		/// At 50, rooms can be a single unit tall or wide.
		const ROOM_SQUARE_DEVIATION_THRESHOLD: i8 = 20;

		pub enum RoomSize {
			LARGE,
			MEDIUM,
			SMALL,
			CUSTOM(i32),
		}

		pub struct Room {
			pub width: i32,
			pub height: i32,
		}

		impl Room {
			pub fn new(size: RoomSize) -> Room {
				let length_width_sum: i32 = match size {
					RoomSize::SMALL => 8,
					RoomSize::MEDIUM => 16,
					RoomSize::LARGE => 32,
					RoomSize::CUSTOM(a) => a,
				};
				let deviation: i32 = (length_width_sum as f32
					* (ROOM_SQUARE_DEVIATION_THRESHOLD as f32 * 0.01)) as i32;
				if deviation == 0 {
					return Room {
						width: length_width_sum / 2,
						height: length_width_sum / 2,
					};
				}
				let min = (length_width_sum / 2) - deviation;
				let max = (length_width_sum / 2) + deviation;
				let width = rand::thread_rng().gen_range(min..max);
				let height = length_width_sum - width;
				if rand::random() {
					return Room { width, height };
				}
				Room {
					width: height,
					height: width,
				}
			}
		}
	}

	pub mod area {
		use std::collections::HashMap;

		pub struct Area {
			pub map: HashMap<i32, HashMap<i32, Tile>>,
		}

		impl Area {
			pub fn new() -> Area {
				Area {
					map: HashMap::new(),
				}
			}

			pub fn get_tile_at<X: Into<i32>, Y: Into<i32>>(&self, x: X, y: Y) -> Tile {
				let x_i32 = &x.into();
				let y_i32 = &y.into();

				let x_col = match self.map.get(x_i32) {
					Some(x_val) => x_val,
					None => {
						return Tile { contents: vec![] };
					}
				};

				match x_col.get(y_i32) {
					Some(tile) => tile.clone(),
					None => Tile { contents: vec![] },
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
		}

		#[derive(Clone)]
		pub struct Tile {
			pub contents: Vec<WorldObject>,
		}

		impl Tile {
			fn new() -> Tile {
				Tile { contents: vec![] }
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
