use super::*;

pub mod engine {

	pub struct Player {
		pub area: super::area::Area,
		pub location: Coordinate,
		pub canvas: super::canvas::Canvas,
	}

	pub fn draw_area(
		player: &mut Player,
		screen_coord_1: Coordinate,
		screen_coord_2: Coordinate,
	) {
		let screen_coordinates = super::canvas::sort_box_coordinates(
			screen_coord_1,
			screen_coord_2
		);
		let width = screen_coordinates[1].x - screen_coordinates[0].x + 1;
		let height = screen_coordinates[1].y - screen_coordinates[0].y + 1;
		//c_x and c_y are the x and y of the center of the selection on the canvas
		let c_x = (width / 2) + screen_coordinates[0].x;
		let c_y = (height / 2) + screen_coordinates[0].y;
		for a in screen_coordinates[0].x..screen_coordinates[1].x {
			for b in screen_coordinates[0].y..screen_coordinates[1].y {
				let x: i32 = player.location.x - (c_x - a);
				let y: i32 = player.location.y - (c_y - b);
				player.canvas.set(a, b, player.area.get_tile_at(x, y).get_char(), Action::MOVE(Coordinate{x, y}));
			}
		}
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