use super::engine::*;

pub mod canvas {
	use std::collections::HashMap;

	/// A block of text with a coordinate system
	#[derive(Clone)]
	pub struct Canvas {
		/// A two dimensional array of chars representing the canvas.
		///
		/// The 1st dimension is the horizontal position, with 0 representing to the leftmost column
		///
		/// The 2nd dimension is the vertical position, with 0 representing the bottommost row
		/// ```
		/// // write "hi" on the bottom row
		/// let canvas = Canvas::new();
		/// canvas.map[0][0] = 'h';
		/// canvas.map[1][0] = 'i';
		/// ```
		pub map: HashMap<i32, HashMap<i32, CanvasUnit>>,
		pub width: i32,
		pub height: i32,
	}

	#[derive(Copy, Clone)]
	pub enum Color {
		White,
		Black,
		Gray,
		DarkGray,
	}

	impl Color {
		pub fn as_string(&self) -> String {
			match self {
				Color::White => String::from("white"),
				Color::Black => String::from("black"),
				Color::Gray => String::from("#333"),
				Color::DarkGray => String::from("#111"),
			}
		}
	}

	#[derive(Copy, Clone)]
	pub struct CanvasUnit {
		pub character: char,
		pub bg_color: Color,
		pub on_click: super::engine::Action,
	}

	impl CanvasUnit {
		fn empty() -> CanvasUnit {
			CanvasUnit {
				character: ' ',
				bg_color: Color::Black,
				on_click: super::engine::Action::None,
			}
		}
	}

	impl Canvas {
		/// Return an empty canvas (all spaces)
		pub fn new<W: Into<i32>, H: Into<i32>>(width: W, height: H) -> Canvas {
			Canvas {
				map: HashMap::new(),
				width: width.into(),
				height: height.into(),
			}
		}

		pub fn get<X: Into<i32>, Y: Into<i32>>(&self, x: X, y: Y) -> CanvasUnit {
			let x_i32 = x.into();
			let y_i32 = y.into();
			if self.out_of_bounds(&x_i32, &y_i32) {
				panic!("Canvas coordinate out of bounds");
			}
			let x_col = match self.map.get(&x_i32) {
				Some(val) => val,
				None => {
					return CanvasUnit::empty();
				}
			};
			match x_col.get(&y_i32) {
				Some(val) => *val,
				None => {
					return CanvasUnit::empty();
				}
			}
		}

		pub fn set<X: Into<i32>, Y: Into<i32>>(
			&mut self,
			x: X,
			y: Y,
			c: char,
			bg_color: Color,
			action: super::engine::Action,
		) {
			let x_i32 = &x.into();
			let y_i32 = &y.into();
			let x_col = match self.map.get_mut(x_i32) {
				Some(val) => val,
				None => {
					self.map.insert(*x_i32, HashMap::new());
					self.map.get_mut(&x_i32).unwrap()
				}
			};
			x_col.insert(
				*y_i32,
				CanvasUnit {
					character: c,
					bg_color,
					on_click: action,
				},
			);
		}

		fn out_of_bounds(&self, x: &i32, y: &i32) -> bool {
			x < &0 || y < &0 || x > &(&self.width - 1) || y > &(&self.height - 1)
		}

		/// Print the canvas to the console
		pub fn print(&self) {
			for y in (0..self.height).rev() {
				let mut row_string: String = String::from("");
				for x in 0..self.width {
					let mut b = [0; 3];
					row_string.push_str(self.get(x, y).character.encode_utf8(&mut b));
				}
				println!("{}", row_string);
			}
		}

		/// Fill a selection with a character
		///
		/// # Arguments
		///
		/// * `fill_from` - The first Coordinate of the rectangular selection
		/// * `fill_to` - The second Coordinate of the rectangular selection
		/// * `fill` - The char to fill the selection with
		pub fn fill(
			&mut self,
			fill_from: super::engine::Coordinate,
			fill_to: super::engine::Coordinate,
			fill: char,
			bg_color: Color,
		) {
			let new_coords = sort_coordinates(fill_from, fill_to);
			for x in new_coords[0].x..(new_coords[1].x + 1) {
				for y in new_coords[0].y..(new_coords[1].y + 1) {
					self.set(x, y, fill, bg_color, super::engine::Action::None);
				}
			}
		}

		///Draw a frame within a selection
		///
		/// # Arguments
		///
		/// * `draw_from` - The first Coordinate of the rectangular selection
		/// * `draw_to` - The second Coordinate of the rectangular selection
		/// * `fills` - An empty string to use the default frame, or a string containing fill characters for the
		/// top left corner, top right corner, bottom right corner, bottom left corner, top/bottom wall, and left/right wall, in that order.
		/// For the default frame, this would be "┌┐┘└─│"
		///
		/// ```
		/// let canvas = Canvas::new();
		/// canvas.draw_frame(
		/// 	Coordinate::new(10,10),
		/// 	Coordinate::new(20,20),
		///		String::from("╔╗╝╚═║"),
		///	);
		/// ```
		pub fn draw_frame(
			&mut self,
			draw_from: super::engine::Coordinate,
			draw_to: super::engine::Coordinate,
			fills: &str,
		) {
			let mut fill_chars = [' '; 6];
			let mut i = 0;
			for char in fills.chars() {
				if i == 6 {
					panic!("Too many characters passed to draw_frame()");
				}
				fill_chars[i] = char;
				i += 1;
			}
			if i == 0 {
				fill_chars[0] = '┌';
				fill_chars[1] = '┐';
				fill_chars[2] = '┘';
				fill_chars[3] = '└';
				fill_chars[4] = '─';
				fill_chars[5] = '│';
			}
			self.fill(
				draw_from,
				super::engine::Coordinate::new(draw_from.x, draw_to.y),
				fill_chars[5],
				Color::Black,
			);
			self.fill(
				draw_from,
				super::engine::Coordinate::new(draw_to.x, draw_from.y),
				fill_chars[4],
				Color::Black,
			);
			self.fill(
				super::engine::Coordinate::new(draw_from.x, draw_to.y),
				draw_to,
				fill_chars[4],
				Color::Black,
			);
			self.fill(
				super::engine::Coordinate::new(draw_to.x, draw_from.y),
				draw_to,
				fill_chars[5],
				Color::Black,
			);
			let new_coords = sort_coordinates(draw_from, draw_to);
			self.set(
				new_coords[0].x,
				new_coords[0].y,
				fill_chars[3],
				Color::Black,
				super::engine::Action::None,
			);
			self.set(
				new_coords[1].x,
				new_coords[1].y,
				fill_chars[1],
				Color::Black,
				super::engine::Action::None,
			);
			self.set(
				new_coords[0].x,
				new_coords[1].y,
				fill_chars[0],
				Color::Black,
				super::engine::Action::None,
			);
			self.set(
				new_coords[1].x,
				new_coords[0].y,
				fill_chars[2],
				Color::Black,
				super::engine::Action::None,
			);
		}

		/// Write strings of word wrapped text to the canvas, bound by the specified selection
		///
		/// # Arguments
		///
		/// * `write_from` - The first Coordinate of the rectangular selection
		/// * `write_to` - The second Coordinate of the rectangular selection
		/// * `text` - The string to write. \n are respected and \t are empty characters that don't overwrite the character behind them.
		///
		/// ```
		/// let canvas = Canvas::new();
		///	canvas.write_text(
		///		Coordinate::new(10,10),
		///		Coordinate::new(20,9),
		///		String::from("hello world!"),
		///	);
		/// ```
		pub fn write_text(
			&mut self,
			write_from: super::engine::Coordinate,
			write_to: super::engine::Coordinate,
			text: &str,
		) {
			let text_chars = text.chars();
			let mut text_box_characters = Vec::new();
			for text_char in text_chars {
				let mut new_text_box_character = TextBoxCharacter {
					character: text_char,
					special_character: SpecialCharacter::None,
				};
				match text_char {
					'\n' => new_text_box_character.special_character = SpecialCharacter::LineBreak,
					'\t' => new_text_box_character.special_character = SpecialCharacter::Empty,
					_ => (),
				}
				text_box_characters.push(new_text_box_character);
			}
			let mut char_index = 0;
			let mut word_length = 0;
			let container_coords = sort_coordinates(write_from, write_to);
			let width = container_coords[1].x - container_coords[0].x + 1;
			let height = container_coords[1].y - container_coords[0].y + 1;
			for y in 0..height {
				for x in 0..width {
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
								}
								SpecialCharacter::Empty => word_length += 1,
								_ => break 'word_char_counting,
							}
						}
					}
					if word_length > width - x && word_length <= width {
						break;
					}
					if matches!(
						text_box_characters[char_index].special_character,
						SpecialCharacter::LineBreak
					) {
						char_index += 1;
						word_length = 0;
						if x != 0 {
							break;
						} else {
							continue;
						}
					}
					if !matches!(
						text_box_characters[char_index].special_character,
						SpecialCharacter::Empty
					) {
						self.set(
							container_coords[0].x + x,
							container_coords[1].y - y,
							text_box_characters[char_index].character,
							Color::Black,
							super::engine::Action::None,
						);
					}
					if word_length > 0 {
						word_length -= 1;
					}
					char_index += 1;
					if char_index >= text_box_characters.len() {
						return;
					}
				}
			}
		}
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

	/// Given 2 coordinates that represent any 2 corners of a box, returns an array T where
	/// T[0] is the bottom left of the box and T[1] is the top right of the box.
	pub fn sort_coordinates(
		coord_one: super::engine::Coordinate,
		coord_two: super::engine::Coordinate,
	) -> [super::engine::Coordinate; 2] {
		if coord_one.x > coord_two.x && coord_one.y > coord_two.y {
			return [coord_two, coord_one];
		} else if coord_one.x > coord_two.x && coord_one.y <= coord_two.y {
			return [
				super::engine::Coordinate::new(coord_two.x, coord_one.y),
				super::engine::Coordinate::new(coord_one.x, coord_two.y),
			];
		} else if coord_one.x <= coord_two.x && coord_one.y > coord_two.y {
			return [
				super::engine::Coordinate::new(coord_one.x, coord_two.y),
				super::engine::Coordinate::new(coord_two.x, coord_one.y),
			];
		}
		[coord_one, coord_two]
	}
}
