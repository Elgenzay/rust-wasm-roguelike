#![allow(dead_code)]
//for testing

use std::io;
use std::io::prelude::*;

mod render;
pub use render::canvas;

mod world;
pub use world::world::{room,area};

fn main() {

//	let new_room = room::Room::new(room::RoomSize::LARGE);
//	println!("width: {}\nheight: {}", new_room.width, new_room.height);
	

//	let mut main_canvas = canvas::Canvas::new();
//	main_canvas.draw_frame(
//		canvas::Coordinate::new(0,0),
//		canvas::Coordinate::new(canvas::CANVAS_WIDTH-1, canvas::CANVAS_HEIGHT-1),
//		"",
//	);
//	
//	main_canvas.print();
//	loop {
//		main_canvas = get_input(main_canvas);
//		main_canvas.print();
//	}
}

/*
fn draw_area(
	canvas: canvas::Canvas,
	screen_coord_1: canvas::Coordinate,
	screen_coord_2: canvas::Coordinate,
	area: area::Area,
	area_point: crate::render::canvas::Coordinate
) -> canvas::Canvas {
	//let screen_coordinates = canvas::sort_box_coordinates(screen_coord_1, screen_coord_2);
	canvas::Canvas::new()
}
*/



fn get_input(mut canvas: canvas::Canvas) -> canvas::Canvas{
	let stdin = io::stdin();
	let mut input: String = "".to_string();
	for line in stdin.lock().lines() {
		input = line.unwrap().to_string();
		break;
	}
	if input.to_lowercase() == "exit"{
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
fn issue_command(mut canvas: canvas::Canvas, command: &str) -> canvas::Canvas{
	let mut base_command = "";
	for s in command.split(" ") {
		base_command = s;
		break;
	}
	match base_command {
		"/fill" | "/frame" | "/text" => {
			let mut i:i8 = -1;
			let mut fill_from = canvas::Coordinate::new(0,0);
			let mut fill_to = canvas::Coordinate::new(0,0);
			let mut fill_char = String::from("");
			for str in command.split(" ") {
				i += 1;
				match i {
					0 => continue,
					1 => {
						fill_from = parse_comma_separated_coordinate_string(&str);
					},
					2 => {
						fill_to = parse_comma_separated_coordinate_string(&str);
					},
					3 => {
						fill_char = String::from(str);
					}
					_ => {
						let mut new_str = String::from(" ");
						new_str.push_str(str);
						fill_char.push_str(&new_str);
					},
				}
			}
			match base_command {
				"/fill" => canvas.fill(fill_from, fill_to, fill_char.chars().nth(0).unwrap_or('?')),
				"/frame" => canvas.draw_frame(fill_from, fill_to, &fill_char[..]),
				"/text" => canvas.write_text(fill_from, fill_to, &fill_char[..]),
				_ => (),
			}
		},
		_ => ()
	}
	canvas
}

fn parse_comma_separated_coordinate_string(string: &str) -> canvas::Coordinate{
	let mut coord = canvas::Coordinate::new(0,0);
	let mut i = 0;
	for s in string.split(",") {
		i += 1;
		let result = s.parse::<i32>();
		match result {
			Ok(v) => {
				match i {
					1 => coord.x = v as usize,
					2 => coord.y = v as usize,
					_ => break,
				}
			},
			Err(_) => break,
		}
	}
	coord
}