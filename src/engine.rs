pub mod engine {
	use crate::area::Area;
	use crate::canvas::{Canvas, sort_coordinates};
	use bresenham::Bresenham;

	pub struct Player {
		pub area: Area,
		pub discovered_area: Area,
		pub location: Coordinate,
		pub canvas: Canvas,
	}

	pub fn draw_area(player: &mut Player, screen_coord_1: Coordinate, screen_coord_2: Coordinate) {
		let screen_coordinates = sort_coordinates(screen_coord_1, screen_coord_2);
		let width = screen_coordinates[1].x - screen_coordinates[0].x + 1;
		let height = screen_coordinates[1].y - screen_coordinates[0].y + 1;
		//c_x and c_y are the x and y of the center of the selection on the canvas
		let c_x = (width / 2) + screen_coordinates[0].x;
		let c_y = (height / 2) + screen_coordinates[0].y;
		for a in screen_coordinates[0].x..screen_coordinates[1].x {
			for b in screen_coordinates[0].y..screen_coordinates[1].y {
				let x: i32 = player.location.x - (c_x - a);
				let y: i32 = player.location.y - (c_y - b);
				let char = if Coordinate::new(x,y) == player.location {
					'O'
				} else if is_visible(player, Coordinate::new(x,y)) {
					let tile = player.area.get_tile_at(x, y);
					player.discovered_area.set_tile(x, y, tile.clone());
					let char = tile.get_char();
					if char == ' ' {
						'.'
					} else {
						char
					}
				} else {
					player.discovered_area.get_tile_at(x,y).get_char()
				};
				player.canvas.set(
					a,
					b,
					char,
					Action::MOVE(Coordinate::new(x, y)),
				);
			}
		}
	}

	fn is_visible(player: &mut Player, location: Coordinate) -> bool {
		for (x, y) in Bresenham::new(player.location.as_tuple(), location.as_tuple()) {
			let (x, y) = (x as i32, y as i32);
			if player.area.get_tile_at(x,y).contents.len() != 0 && Coordinate::new(x,y) != location {
				return false
			}
		}
		return true;
	}

	/// A point in 2D space
	#[derive(Copy, Clone, Debug)]
	pub struct Coordinate {
		pub x: i32, // 0: leftmost
		pub y: i32, // 0: bottommost
	}

	impl Coordinate {
		/// Return a Coordinate with the specified position
		///
		/// # Arguments
		///
		/// * `x` - The X position of the new Coordinate
		/// * `y` - the Y position of the new Coordinate
		pub fn new<X: Into<i32>, Y: Into<i32>>(x: X, y: Y) -> Coordinate {
			Coordinate {
				x: x.into(),
				y: y.into(),
			}
		}

		pub fn set<X: Into<i32>, Y: Into<i32>>(&mut self, x: X, y: Y) {
			self.x = x.into();
			self.y = y.into();
		}

		pub fn as_tuple<T: TryFrom<i32>>(&self) -> (T, T) {
			(T::try_from(self.x).ok().unwrap(), T::try_from(self.y).ok().unwrap())
		}
	}

	impl PartialEq for Coordinate {
		fn eq(&self, other: &Coordinate) -> bool {
			self.x == other.x && self.y == other.y
		}
	}

	#[derive(Copy, Clone)]
	pub enum Action {
		NONE,
		MOVE(Coordinate),
	}
}
