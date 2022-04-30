pub mod engine {
	use crate::render::canvas::{sort_coordinates, Canvas, Color};
	use crate::world::world::area::Area;
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
		let screen_center_x = (width / 2) + screen_coordinates[0].x;
		let screen_center_y = (height / 2) + screen_coordinates[0].y;
		for screen_x in screen_coordinates[0].x..screen_coordinates[1].x {
			for screen_y in screen_coordinates[0].y..screen_coordinates[1].y {
				let x: i32 = player.location.x - (screen_center_x - screen_x);
				let y: i32 = player.location.y - (screen_center_y - screen_y);
				let tile = player.area.get_tile_at(x, y);
				let visible = is_visible(player, Coordinate::new(x, y));
				let (char, bg_color) = if Coordinate::new(x, y) == player.location {
					('O', Some(Color::Gray))
				} else if visible {
					player.discovered_area.set_tile(x, y, tile.clone());
					if tile.contents.len() == 0 {
						(' ', Some(Color::Gray))
					} else {
						(tile.get_char(), tile.get_bgcolor())
					}
				} else {
					let bgcolor = player.discovered_area.get_tile_at(x, y).get_bgcolor();
					(
						player.discovered_area.get_tile_at(x, y).get_char(),
						if player.discovered_area.tile_exists(x, y) {
							if matches!(bgcolor, None) {
								Some(Color::DarkGray)
							} else {
								bgcolor
							}
						} else {
							bgcolor
						},
					)
				};
				player.canvas.set(
					screen_x,
					screen_y,
					char,
					match bg_color{
						Some(c) => c,
						None => Color::Black
					},
					if visible && !tile.contains_wall() && Coordinate::new(x, y) != player.location
					{
						Action::Move(Coordinate::new(x, y))
					} else {
						Action::None
					},
				);
			}
		}
	}

	fn is_visible(player: &mut Player, location: Coordinate) -> bool {
		for (x, y) in Bresenham::new(player.location.as_tuple(), location.as_tuple()) {
			let (x, y) = (x as i32, y as i32);
			if player.area.get_tile_at(x, y).contents.len() != 0
				&& Coordinate::new(x, y) != location
			{
				return false;
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
			(
				T::try_from(self.x).ok().unwrap(),
				T::try_from(self.y).ok().unwrap(),
			)
		}
	}

	impl PartialEq for Coordinate {
		fn eq(&self, other: &Coordinate) -> bool {
			self.x == other.x && self.y == other.y
		}
	}

	#[derive(Copy, Clone)]
	pub enum Action {
		None,
		Move(Coordinate),
	}
}
