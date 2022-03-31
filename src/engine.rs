use super::*;

pub mod engine {
	pub fn draw_area(
		canvas: &mut super::canvas::Canvas,
		screen_coord_1: super::canvas::Coordinate,
		screen_coord_2: super::canvas::Coordinate,
		area: &super::area::Area,
		area_point: crate::render::canvas::Coordinate,
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
				let x: i32 = area_point.x - (c_x - a);
				let y: i32 = area_point.y - (c_y - b);
				canvas.set(a, b, area.get_tile_at(x, y).get_char());
			}
		}
	}
}