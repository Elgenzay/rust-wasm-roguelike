
mod engine;
pub use crate::engine::canvas::*;

fn main() {
	let mut main_canvas = engine::canvas::Canvas::new();
	main_canvas.draw_frame(
		engine::canvas::Coordinate::new(0,0),
		engine::canvas::Coordinate::new(engine::canvas::CANVAS_WIDTH-1, engine::canvas::CANVAS_HEIGHT-1),
		String::from(""),
	);
	
	main_canvas.print();
	loop {
		main_canvas = engine::canvas::get_input(main_canvas);
		main_canvas.print();
	}
}