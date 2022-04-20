#![allow(dead_code)]
//for testing

use std::io;
use std::io::prelude::*;

mod render;
pub use render::canvas;

mod world;
pub use world::world::{area, region};

mod engine;
pub use engine::engine::*;

///pub const CANVAS_WIDTH: i32 = 119;
//pub const CANVAS_HEIGHT: i32 = 28;
pub const CANVAS_WIDTH: i32 = 250;
pub const CANVAS_HEIGHT: i32 = 70;

fn main() {
	let mut main_canvas = canvas::Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
	main_canvas.draw_frame(
		Coordinate::new(0, 0),
		Coordinate::new(CANVAS_WIDTH - 1, CANVAS_HEIGHT - 1),
		"",
	);
	//let mut main_area = area::Area::new(Option::None);
	let main_area = area::Area::new_bsp_dungeon(area::DungeonConfig::default());

	//let mut test_region = region::Region::new(10, 10, Coordinate::new(-22, -12));
	//let mut test_region = region::Region::new(10, 10, Coordinate::new(-22, 0));
	//let mut test_region = region::Region::new(10, 10, Coordinate::new(-22, 12));
	//let mut test_region = region::Region::new(10, 10, Coordinate::new(-22, -12));
	//let mut test_region = region::Region::new(10, 10, Coordinate::new(0, -12));
	//let mut test_region = region::Region::new(10, 10, Coordinate::new(22, -12));
	//let mut test_region = region::Region::new(10, 10, Coordinate::new(0, 12));

//	main_area.place_region(&mut test_region);
//
//	let mut test_region_2 = region::Region::new(10, 10, Coordinate::new(0, 0));
//	main_area.place_region(&mut test_region_2);
//
//	main_area.create_hallway(&mut test_region, &mut test_region_2);

	let mut player = Player {
		area: main_area,
		location: Coordinate::new(50, 25),
		canvas: main_canvas,
	};
	loop {
		// temporary test loop

		draw_area(
			&mut player,
			Coordinate::new(1, 1),
			Coordinate::new(CANVAS_WIDTH - 1, CANVAS_HEIGHT - 1),
		);
		player.canvas.print();

		let stdin = io::stdin();
		let mut input: String = "".to_string();
		for line in stdin.lock().lines() {
			input = line.unwrap().to_string();
			break;
		}
		let str = &input.to_lowercase()[..];
		//command_to_click(&mut player, str);
		match str {
			"exit" => std::process::exit(0),
			"w" => player.location.y = player.location.y + 6,
			"a" => player.location.x = player.location.x - 16,
			"s" => player.location.y = player.location.y - 6,
			"d" => player.location.x = player.location.x + 16,
			_ => (),
		}
	}
}

fn command_to_click(player: &mut Player, command: &str) {
	let mut args = command.split(" ");
	let mut x = 0;
	let mut y = 0;
	let mut rightclick = false;
	for i in 1..4 {
		match args.next() {
			Some(str) => match i {
				1 => match str.parse::<i32>() {
					Ok(v) => {
						x = v;
					}
					Err(_) => return,
				},
				2 => match str.parse::<i32>() {
					Ok(v) => {
						y = v;
					}
					Err(_) => return,
				},
				3 => match str.parse::<i32>() {
					Ok(v) => {
						if v > 0 {
							rightclick = true;
						}
					}
					Err(_) => (),
				},
				_ => (),
			},
			None => {
				if i == 3 {
					()
				} else {
					return;
				}
			}
		}
	}
	click(player, x, y, rightclick);
}

fn click(mut player: &mut Player, x: i32, y: i32, rightclick: bool) {
	println!(
		"{}clicked ({},{})",
		if rightclick { "right" } else { "" },
		x,
		y
	);
	let canvas_unit_at_click = player.canvas.get(x, y);
	match canvas_unit_at_click.on_click {
		Action::MOVE(coord) => {
			player.location = coord;
		}
		_ => (),
	}
}

fn get_input(mut canvas: canvas::Canvas) -> canvas::Canvas {
	let stdin = io::stdin();
	let mut input: String = "".to_string();
	for line in stdin.lock().lines() {
		input = line.unwrap().to_string();
		break;
	}
	if input.to_lowercase() == "exit" {
		std::process::exit(0);
	}
	if input.chars().count() != 0 && &input[0..1] == "/" {
		canvas = issue_command(canvas, &input[..]);
		canvas.print();
		canvas = get_input(canvas);
	}
	return canvas;
}

/// Process commands
///
/// Examples:
///
/// - /fill 20,20 25,25 x
/// - /text 50,20 60,5 the quick brown fox jumps over the lazy dog
/// - /frame 49,21 61,4
/// - /frame 4,4 15,15 abcdef
fn issue_command(mut canvas: canvas::Canvas, command: &str) -> canvas::Canvas {
	let mut base_command = "";
	for s in command.split(" ") {
		base_command = s;
		break;
	}
	match base_command {
		"/fill" | "/frame" | "/text" => {
			let mut i: i8 = -1;
			let mut fill_from = Coordinate::new(0, 0);
			let mut fill_to = Coordinate::new(0, 0);
			let mut fill_char = String::from("");
			for str in command.split(" ") {
				i += 1;
				match i {
					0 => continue,
					1 => {
						fill_from = parse_comma_separated_coordinate_string(&str);
					}
					2 => {
						fill_to = parse_comma_separated_coordinate_string(&str);
					}
					3 => {
						fill_char = String::from(str);
					}
					_ => {
						let mut new_str = String::from(" ");
						new_str.push_str(str);
						fill_char.push_str(&new_str);
					}
				}
			}
			match base_command {
				"/fill" => canvas.fill(fill_from, fill_to, fill_char.chars().nth(0).unwrap_or('?')),
				"/frame" => canvas.draw_frame(fill_from, fill_to, &fill_char[..]),
				"/text" => canvas.write_text(fill_from, fill_to, &fill_char[..]),
				_ => (),
			}
		}
		_ => (),
	}
	canvas
}

fn parse_comma_separated_coordinate_string(string: &str) -> Coordinate {
	let mut coord = Coordinate::new(0, 0);
	let mut i = 0;
	for s in string.split(",") {
		i += 1;
		let result = s.parse::<i32>();
		match result {
			Ok(v) => match i {
				1 => coord.x = v,
				2 => coord.y = v,
				_ => break,
			},
			Err(_) => break,
		}
	}
	coord
}
