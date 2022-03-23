pub mod canvas {

	pub const CANVAS_WIDTH: usize = 119;
	pub const CANVAS_HEIGHT: usize = 28;

	/// A block of text with a coordinate system
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
		map: [[char; CANVAS_HEIGHT]; CANVAS_WIDTH],
	}

	impl Canvas {

		/// Return an empty canvas (all spaces)
		pub fn new() -> Canvas {
			Canvas {
				map: [[' '; CANVAS_HEIGHT]; CANVAS_WIDTH],
			}
		}

		/// Print the canvas to the console
		pub fn print(&self){
			for y in (0..CANVAS_HEIGHT).rev() {
				let mut row_string: String = String::from("");
				for x in 0..CANVAS_WIDTH {
					let mut b = [0; 3];
					row_string.push_str(self.map[x][y].encode_utf8(&mut b));
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
		pub fn fill(&mut self, fill_from: Coordinate, fill_to: Coordinate, fill: char){
			let new_coords = sort_box_coordinates(fill_from, fill_to);
			for x in new_coords[0].x..(new_coords[1].x + 1) {
				for y in new_coords[0].y..(new_coords[1].y + 1) {
					self.map[x][y] = fill;
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
			draw_from: Coordinate,
			draw_to: Coordinate,
			fills: &str
		){
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
			self.fill(draw_from, Coordinate::new(draw_from.x, draw_to.y), fill_chars[5]);
			self.fill(draw_from, Coordinate::new(draw_to.x, draw_from.y), fill_chars[4]);
			self.fill(Coordinate::new(draw_from.x, draw_to.y), draw_to, fill_chars[4]);
			self.fill(Coordinate::new(draw_to.x, draw_from.y), draw_to, fill_chars[5]);
		
			let new_coords = sort_box_coordinates(draw_from, draw_to);
			self.map[new_coords[0].x][new_coords[0].y] = fill_chars[3];
			self.map[new_coords[1].x][new_coords[1].y] = fill_chars[1];
			self.map[new_coords[0].x][new_coords[1].y] = fill_chars[0];
			self.map[new_coords[1].x][new_coords[0].y] = fill_chars[2];
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
			write_from: Coordinate,
			write_to: Coordinate,
			text: &str
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
					_ => ()
				}
				text_box_characters.push(new_text_box_character);
			}
			let mut char_index = 0;
			let mut word_length = 0;
		
			let container_coords = sort_box_coordinates(write_from, write_to);
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
								},
								SpecialCharacter::Empty => word_length += 1,
								_ => break 'word_char_counting,
							}
						}
					}
					if word_length > width - x && word_length <= width {
						break;
					}
					if matches!(text_box_characters[char_index].special_character, SpecialCharacter::LineBreak){
						char_index += 1;
						word_length = 0;
						if x != 0 {
							break;
						} else {
							continue;
						}
					}
					if !matches!(text_box_characters[char_index].special_character, SpecialCharacter::Empty){
						self.map[container_coords[0].x + x][container_coords[1].y - y] = text_box_characters[char_index].character;
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

	/// A point in 2D space
	#[derive(Copy,Clone,Debug)]
	pub struct Coordinate {
		pub x: usize, // 0: leftmost
		pub y: usize, // 0: bottommost
	}

	impl Coordinate {

		/// Return a Coordinate with the specified position
		/// 
		/// # Arguments
		/// 
		/// * `x` - The X position of the new Coordinate
		/// * `y` - the Y position of the new Coordinate
		pub fn new(x: usize, y: usize) -> Coordinate{
			Coordinate {
				x: x,
				y: y,
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

	fn sort_box_coordinates(coord_one: Coordinate, coord_two: Coordinate) -> [Coordinate; 2]{
		if coord_one.x > coord_two.x && coord_one.y > coord_two.y {
			return [coord_two, coord_one];
		} else if coord_one.x > coord_two.x && coord_one.y <= coord_two.y {
			return [
				Coordinate::new(coord_two.x, coord_one.y),
				Coordinate::new(coord_one.x, coord_two.y),
			];
		} else if coord_one.x <= coord_two.x && coord_one.y > coord_two.y {
			return [
				Coordinate::new(coord_one.x, coord_two.y),
				Coordinate::new(coord_two.x, coord_one.y),
			];
		}
		[coord_one, coord_two]
	}
}