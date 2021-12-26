use std::io;
use std::io::prelude::*;

const CANVAS_WIDTH: usize = 119;
const CANVAS_HEIGHT: usize = 28;

#[derive(Copy,Clone,Debug)]
struct Coordinate {
	x: usize, // 0: leftmost
	y: usize, // 0: bottommost
}

struct TextBox {
	width: i8,
	height: i8,
	anchor_point: Coordinate,
	text: String,
}

enum SpecialCharacter {
	None,
	LineBreak,
	Empty,
}

struct TextBoxCharacter {
	character: char,
	special_character: SpecialCharacter,
}

fn main() {
	let mut canvas: [[char; CANVAS_HEIGHT]; CANVAS_WIDTH] = [[' '; CANVAS_HEIGHT]; CANVAS_WIDTH];
	canvas[0][0] = '└';
	canvas[CANVAS_WIDTH - 1][0] = '┘';
	canvas[0][CANVAS_HEIGHT - 1] = '┌';
	canvas[CANVAS_WIDTH - 1][CANVAS_HEIGHT - 1] = '┐';

	canvas = draw_frame(
		canvas,
		Coordinate {
			x:0,
			y:0,
		},
		Coordinate {
			x:CANVAS_WIDTH-1,
			y:CANVAS_HEIGHT-1,
		},
		String::from(""),
	);
	
	/*
	let write_text = TextBox {
		width: 25,
		height: 14,
		anchor_point: Coordinate {
			x: 4,
			y: 26,
		},
		text: String::from("this\n is an example of speex, an audio compression codec specifically tuned for the reproduction of human speech. the quick brown fox jumps over the lazy dog.\nEmpty space: [\t\t\t]"),
	};
	canvas = write_textbox(canvas, write_text);
	*/
	

//	canvas = fill(
//		canvas,
//
//		Coordinate {
//			x:8,
//			y:8,
//		},
//
//		Coordinate {
//			x:4,
//			y:4,
//		},
//
//
//
//		'x',
//	);
//	canvas = draw_frame(
//		canvas,
//
//
//		Coordinate {
//			x:4,
//			y:4,
//		},
//
//		Coordinate {
//			x:40,
//			y:20,
//		},
//
//
//		String::from(""),
//	);
	
	output_canvas(canvas);
	loop {
		canvas = get_input(canvas);
		output_canvas(canvas);
	}
}


fn get_input(mut canvas: [[char; CANVAS_HEIGHT]; CANVAS_WIDTH]) -> [[char; CANVAS_HEIGHT]; CANVAS_WIDTH]{
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
		canvas = issue_command(canvas, input);
		output_canvas(canvas);
		canvas = get_input(canvas);
	}
	return canvas;
}

fn issue_command(mut canvas: [[char; CANVAS_HEIGHT]; CANVAS_WIDTH], command: String) -> [[char; CANVAS_HEIGHT]; CANVAS_WIDTH]{
	let mut base_command = "";
	for s in command.split(" ") {
		base_command = s;
		break;
	}
	match base_command {
		"/fill" | "/frame" => {
			let mut i:i8 = -1;
			let mut fill_from = Coordinate {
				x: 0,
				y: 0,
			};
			let mut fill_to = Coordinate {
				x: 0,
				y: 0,
			};
			let mut fill_char = "";
			for str in command.split(" ") {
				i += 1;
				let string = String::from(str);
				match i {
					0 => continue,
					1 => {
						fill_from = parse_comma_separated_coordinate_string(string);
					},
					2 => {
						fill_to = parse_comma_separated_coordinate_string(string);
					},
					3 => {
						fill_char = str;
					}
					_ => break,
				}
			}
			if base_command == "/fill" {
				canvas = fill(canvas, fill_from, fill_to, fill_char.chars().nth(0).unwrap_or('?'));
			} else {
				canvas = draw_frame(canvas, fill_from, fill_to, String::from(fill_char));
			}
		},
		_ => ()
	}
	canvas
}

/*
fn get_input_i64() -> i64 {
	let out: i64 = loop {
		let result = get_input().parse::<i64>();
		match result {
			Ok(v) => break v,
			Err(e) => {println!("Error: {}", e)},
		}
	};
	out
}
*/

fn parse_comma_separated_coordinate_string(string: String) -> Coordinate{
	let mut coord = Coordinate {
		x: 0,
		y: 0,
	};
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

fn write_textbox(mut canvas: [[char; CANVAS_HEIGHT]; CANVAS_WIDTH], text_box: TextBox) -> [[char; CANVAS_HEIGHT]; CANVAS_WIDTH]{
	let anchor_x = text_box.anchor_point.x;
	let anchor_y = text_box.anchor_point.y;
	let text_chars = text_box.text.chars();
	let mut text_box_characters = Vec::new();
	for text_char in text_chars {
		let mut new_text_box_character = TextBoxCharacter {
			character: text_char,
			special_character: SpecialCharacter::None,
		};
		match text_char {
			'\n' => new_text_box_character.special_character = SpecialCharacter::LineBreak,
			'\t' => new_text_box_character.special_character = SpecialCharacter::Empty,
			_ => ()
		}
		text_box_characters.push(new_text_box_character);
	}
	let mut char_index = 0;
	let mut word_length = 0;
	'text_box_loop: for y in 0..text_box.height {
		for x in 0..text_box.width {
			if word_length == 0 {
				word_length = 0;
				'word_char_counting: for i in char_index..text_box_characters.len() {
					match text_box_characters[i].special_character {
						SpecialCharacter::None => {
							if text_box_characters[i].character == ' ' {
								if x == 0 {
									while text_box_characters[char_index].character == ' ' {
										char_index += 1;
									}
								}
								break 'word_char_counting;
							}
							word_length += 1;
						},
						SpecialCharacter::Empty => word_length += 1,
						_ => break 'word_char_counting,
					}
				}
			}
			if word_length > text_box.width - x && word_length < text_box.width {
				break;
			}
			if matches!(text_box_characters[char_index].special_character, SpecialCharacter::LineBreak){
				char_index += 1;
				break;
			}
			if !matches!(text_box_characters[char_index].special_character, SpecialCharacter::Empty){
				canvas[anchor_x + x as usize][anchor_y - y as usize] = text_box_characters[char_index].character;
			}
			if word_length > 0 {
				word_length -= 1;
			}
			char_index += 1;
			if char_index >= text_box_characters.len() {
				break 'text_box_loop;
			}
		}
	}
	canvas
}

fn fill(mut canvas: [[char; CANVAS_HEIGHT]; CANVAS_WIDTH], fill_from: Coordinate, fill_to: Coordinate, fill: char) -> [[char; CANVAS_HEIGHT]; CANVAS_WIDTH]{
	let new_coords = sort_box_coordinates(fill_from, fill_to);
	for x in new_coords[0].x..(new_coords[1].x + 1) {
		for y in new_coords[0].y..(new_coords[1].y + 1) {
			canvas[x][y] = fill;
		}
	}
	canvas
}

fn draw_frame(mut canvas: [[char; CANVAS_HEIGHT]; CANVAS_WIDTH], draw_from: Coordinate, draw_to: Coordinate, fills: String) -> [[char; CANVAS_HEIGHT]; CANVAS_WIDTH]{
	let mut fill_chars = [' '; 6];
	let mut i = 0;
	for char in fills.chars() {
		if i == 6 {
			panic!("Too many characters passed to draw_frame()");
		}
		fill_chars[i] = char;
		i += 1;
	}
	if i == 0{
		fill_chars[0] = '┌';
		fill_chars[1] = '┐';
		fill_chars[2] = '┘';
		fill_chars[3] = '└';
		fill_chars[4] = '─';
		fill_chars[5] = '│';
	}
	canvas = fill(canvas, draw_from, Coordinate{
		x: draw_from.x,
		y: draw_to.y,
	}, fill_chars[5]);
	canvas = fill(canvas, draw_from, Coordinate{
		x: draw_to.x,
		y: draw_from.y,
	}, fill_chars[4]);
	canvas = fill(canvas, Coordinate{
		x: draw_from.x,
		y: draw_to.y,
	}, draw_to, fill_chars[4]);
	canvas = fill(canvas, Coordinate{
		x: draw_to.x,
		y: draw_from.y,
	}, draw_to, fill_chars[5]);

	let new_coords = sort_box_coordinates(draw_from, draw_to);
	canvas[new_coords[0].x][new_coords[0].y] = fill_chars[3];
	canvas[new_coords[1].x][new_coords[1].y] = fill_chars[1];
	canvas[new_coords[0].x][new_coords[1].y] = fill_chars[0];
	canvas[new_coords[1].x][new_coords[0].y] = fill_chars[2];
	canvas
}

fn sort_box_coordinates(coord_one: Coordinate, coord_two: Coordinate) -> [Coordinate; 2]{
	if coord_one.x > coord_two.x && coord_one.y > coord_two.y {
		return [coord_two, coord_one];
	} else if coord_one.x > coord_two.x && coord_one.y <= coord_two.y {
		return [
			Coordinate {
				x: coord_two.x,
				y: coord_one.y,
			},
			Coordinate {
				x: coord_one.x,
				y: coord_two.y,
			},
		];
	} else if coord_one.x <= coord_two.x && coord_one.y > coord_two.y {
		return [
			Coordinate {
				x: coord_one.x,
				y: coord_two.y,
			},
			Coordinate {
				x: coord_two.x,
				y: coord_one.y,
			},
		];
	}
	[coord_one, coord_two]
}

fn output_canvas(canvas:[[char; CANVAS_HEIGHT]; CANVAS_WIDTH]){
	for y in (0..CANVAS_HEIGHT).rev() {
		let mut row_string: String = String::from("");
		for x in 0..CANVAS_WIDTH {
			let mut b = [0; 3];
			row_string.push_str(canvas[x][y].encode_utf8(&mut b));
		}
		println!("{}", row_string);
	}
}
