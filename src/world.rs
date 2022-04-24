pub mod world {

	pub mod region {
		use crate::engine::engine::Coordinate;
		use std::cell::Cell;

		pub struct Region {
			pub width: i32,
			pub height: i32,
			pub position: Coordinate,
			edge_x: Cell<Option<i32>>,
			top_y: Cell<Option<i32>>,
		}

		impl Region {
			pub fn new<X: Into<i32>, Y: Into<i32>>(
				width: X,
				height: Y,
				position: Coordinate,
			) -> Region {
				Region {
					width: width.into(),
					height: height.into(),
					position,
					edge_x: Cell::new(None),
					top_y: Cell::new(None),
				}
			}

			pub fn get_edge_x(&self) -> i32 {
				match self.edge_x.get() {
					Some(x) => x,
					None => {
						let edge_x = self.position.x + self.width - 1;
						self.edge_x.set(Some(edge_x));
						edge_x
					}
				}
			}

			pub fn get_top_y(&self) -> i32 {
				match self.top_y.get() {
					Some(y) => y,
					None => {
						let top_y = self.position.y + self.height - 1;
						self.top_y.set(Some(top_y));
						top_y
					}
				}
			}

			pub fn overlaps_coordinate(&self, coord: Coordinate) -> bool {
				coord.y <= self.get_top_y()
					&& coord.y >= self.position.y
					&& coord.x <= self.get_edge_x()
					&& coord.x >= self.position.x
			}
		}
	}

	pub mod area {
		use crate::engine::engine::Coordinate;
		use crate::render::canvas::Color;
		use crate::world::world::region::Region;
		use rand::Rng;
		use std::collections::HashMap;

		pub enum Hallway {
			STRAIGHT(
				bool, // true if vertical
				i32,  // y position if horizontal, or x position if vertical
				i32,  // starting (left x/bottom y) position
				i32,  // ending (right x/top y) position
			),
			BENT(
				BoxCorner,  // orientation
				Coordinate, // turning point
				i32,        // horizontal distance
				i32,        // vertical distance
			),
		}

		#[derive(Copy, Clone)]
		pub enum BoxCorner {
			BottomRight, // ┘
			BottomLeft,  // └
			TopRight,    // ┐
			TopLeft,     // ┌
		}

		pub struct Area {
			pub map: HashMap<i32, HashMap<i32, Tile>>,
			pub default_fill: Option<WorldObject>,
		}

		impl Area {
			pub fn new(default_fill: Option<WorldObject>) -> Area {
				Area {
					map: HashMap::new(),
					default_fill,
				}
			}

			pub fn place_region(&mut self, region: &Region) {
				self.fill(
					region.position,
					Coordinate::new(region.get_edge_x(), region.get_top_y()),
					Tile::wall(),
				);
				self.fill(
					Coordinate::new(region.position.x + 1, region.position.y + 1),
					Coordinate::new(region.get_edge_x() - 1, region.get_top_y() - 1),
					Tile::new(None),
				);
			}

			pub fn create_hallway(&mut self, region_1: &Region, region_2: &Region) {
				let valid_hallways = self.get_valid_hallways(region_1, region_2);
				self.create_hallway_from_valid(&valid_hallways);
			}

			pub fn create_hallway_from_valid(&mut self, valid_hallways: &Vec<Hallway>) {
				if valid_hallways.len() == 0 {
					return;
				}
				let hallway = valid_hallways
					.get(rand::thread_rng().gen_range(0..valid_hallways.len()))
					.unwrap();
				match *hallway {
					Hallway::STRAIGHT(is_vertical, x_y, start, end) => {
						self.fill(
							Coordinate::new(
								if is_vertical { x_y - 1 } else { start },
								if is_vertical { start } else { x_y - 1 },
							),
							Coordinate::new(
								if is_vertical { x_y + 1 } else { end },
								if is_vertical { end } else { x_y + 1 },
							),
							Tile::wall(),
						);
						self.fill(
							Coordinate::new(
								if is_vertical { x_y } else { start },
								if is_vertical { start } else { x_y },
							),
							Coordinate::new(
								if is_vertical { x_y } else { end },
								if is_vertical { end } else { x_y },
							),
							Tile::new(None),
						);
					}
					Hallway::BENT(orientation, point, d_hor, d_ver) => {
						let x = match orientation {
							BoxCorner::BottomRight | BoxCorner::TopRight => point.x - d_hor,
							BoxCorner::BottomLeft | BoxCorner::TopLeft => point.x + d_hor,
						};
						let y = match orientation {
							BoxCorner::TopRight | BoxCorner::TopLeft => point.y - d_ver,
							BoxCorner::BottomRight | BoxCorner::BottomLeft => point.y + d_ver,
						};

						self.fill(
							Coordinate::new(point.x, point.y - 1),
							Coordinate::new(x, point.y + 1),
							Tile::wall(),
						);
						self.fill(
							Coordinate::new(point.x - 1, point.y),
							Coordinate::new(point.x + 1, y),
							Tile::wall(),
						);

						let (corner_x, corner_y) = match orientation {
							BoxCorner::TopLeft => (-1, 1),
							BoxCorner::TopRight => (1, 1),
							BoxCorner::BottomLeft => (-1, -1),
							BoxCorner::BottomRight => (1, -1),
						};
						self.set_tile(point.x + corner_x, point.y + corner_y, Tile::wall());

						self.fill(point, Coordinate::new(x, point.y), Tile::new(None));
						self.fill(point, Coordinate::new(point.x, y), Tile::new(None));
					}
				};
			}

			pub fn get_valid_hallways(
				&mut self,
				region_1: &Region,
				region_2: &Region,
			) -> Vec<Hallway> {
				let regions = if region_2.position.y > region_1.position.y {
					[region_1, region_2]
				} else {
					[region_2, region_1]
				};
				let mut possible_hallways = vec![];
				let (vertical, horizontal) = if regions[0].get_top_y() >= regions[1].position.y + 2
					&& regions[1].get_top_y() >= regions[0].position.y + 2
				{
					(false, true)
				} else if regions[0].get_edge_x() >= regions[1].position.x + 2
					&& regions[1].get_edge_x() >= regions[0].position.x + 2
				{
					(true, false)
				} else {
					(false, false)
				};
				if vertical || horizontal {
					let (one_t_e, two_y_x, two_t_e, one_y_x, one_x_y, two_x_y, one_e_t, two_e_t) =
						if horizontal {
							(
								regions[0].get_top_y(),
								regions[1].position.y,
								regions[1].get_top_y(),
								regions[0].position.y,
								regions[0].position.x,
								regions[1].position.x,
								regions[0].get_edge_x(),
								regions[1].get_edge_x(),
							)
						} else {
							(
								regions[0].get_edge_x(),
								regions[1].position.x,
								regions[1].get_edge_x(),
								regions[0].position.x,
								regions[0].position.y,
								regions[1].position.y,
								regions[0].get_top_y(),
								regions[1].get_top_y(),
							)
						};
					let starting_xy = if one_y_x > two_y_x { one_y_x } else { two_y_x };
					let ending_xy = if one_t_e > two_t_e { two_t_e } else { one_t_e };
					for y in starting_xy + 1..ending_xy {
						possible_hallways.push(Hallway::STRAIGHT(
							vertical,
							y,
							if one_x_y > two_x_y { two_e_t } else { one_e_t },
							if one_x_y > two_x_y { one_x_y } else { two_x_y },
						));
					}
				} else {
					let possible_orientations = if regions[0].position.x < regions[1].position.x {
						[BoxCorner::TopLeft, BoxCorner::BottomRight]
					} else {
						[BoxCorner::BottomLeft, BoxCorner::TopRight]
					};

					for orientation in possible_orientations {
						let (start_x, end_x, start_y, end_y) = match orientation {
							BoxCorner::BottomLeft | BoxCorner::BottomRight => (
								regions[1].position.x,
								regions[1].get_edge_x(),
								regions[0].position.y,
								regions[0].get_top_y(),
							),
							BoxCorner::TopLeft | BoxCorner::TopRight => (
								regions[0].position.x,
								regions[0].get_edge_x(),
								regions[1].position.y,
								regions[1].get_top_y(),
							),
						};
						for y in start_y + 1..end_y {
							for x in start_x + 1..end_x {
								if regions[1].overlaps_coordinate(Coordinate::new(x, y))
									|| regions[0].overlaps_coordinate(Coordinate::new(x, y))
								{
									continue;
								}
								let (horizontal_distance, vertical_distance) = match orientation {
									BoxCorner::BottomLeft => {
										(regions[0].position.x - x, regions[1].position.y - y)
									}
									BoxCorner::BottomRight => {
										(x - regions[0].get_edge_x(), regions[1].position.y - y)
									}
									BoxCorner::TopRight => {
										(x - regions[1].get_edge_x(), y - regions[0].get_top_y())
									}
									BoxCorner::TopLeft => {
										(regions[1].position.x - x, y - regions[0].get_top_y())
									}
								};
								possible_hallways.push(Hallway::BENT(
									orientation,
									Coordinate::new(x, y),
									horizontal_distance,
									vertical_distance,
								));
							}
						}
					}
				}
				let mut valid_hallways = vec![];
				for hallway in possible_hallways {
					let is_valid = match hallway {
						Hallway::STRAIGHT(is_vertical, x_y, start, end) => {
							if is_vertical {
								self.region_is_empty(
									Coordinate::new(x_y, start + 1),
									Coordinate::new(x_y, end - 1),
								)
							} else {
								self.region_is_empty(
									Coordinate::new(start + 1, x_y),
									Coordinate::new(end - 1, x_y),
								)
							}
						}
						Hallway::BENT(orientation, point, d_hor, d_ver) => {
							self.region_is_empty(
								point,
								Coordinate::new(
									match orientation {
										BoxCorner::BottomRight | BoxCorner::TopRight => {
											point.x - d_hor + 1
										}
										BoxCorner::BottomLeft | BoxCorner::TopLeft => {
											point.x + d_hor - 1
										}
									},
									point.y,
								),
							) && self.region_is_empty(
								point,
								Coordinate::new(
									point.x,
									match orientation {
										BoxCorner::TopRight | BoxCorner::TopLeft => {
											point.y - d_ver + 1
										}
										BoxCorner::BottomRight | BoxCorner::BottomLeft => {
											point.y + d_ver - 1
										}
									},
								),
							)
						}
					};
					if is_valid {
						valid_hallways.push(hallway);
					}
				}
				valid_hallways
			}

			pub fn get_tile_at<X: Into<i32>, Y: Into<i32>>(&self, x: X, y: Y) -> Tile {
				let x_i32 = &x.into();
				let y_i32 = &y.into();

				let x_col = match self.map.get(x_i32) {
					Some(x_val) => x_val,
					None => {
						return match &self.default_fill {
							Some(obj) => Tile {
								contents: vec![*obj],
							},
							None => Tile { contents: vec![] },
						};
					}
				};

				match x_col.get(y_i32) {
					Some(tile) => tile.clone(),
					None => match &self.default_fill {
						Some(obj) => Tile {
							contents: vec![*obj],
						},
						None => Tile { contents: vec![] },
					},
				}
			}

			pub fn set_tile<X: Into<i32>, Y: Into<i32>>(&mut self, x: X, y: Y, t: Tile) {
				let x_i32 = x.into();
				let y_i32 = y.into();

				let x_col = match self.map.get_mut(&x_i32) {
					Some(x_val) => x_val,
					None => {
						self.map.insert(x_i32, HashMap::new());
						self.map.get_mut(&x_i32).unwrap()
					}
				};
				x_col.insert(y_i32, t);
			}

			pub fn region_is_empty(&self, coord_1: Coordinate, coord_2: Coordinate) -> bool {
				let coords = crate::render::canvas::sort_coordinates(coord_1, coord_2);
				for x in coords[0].x..=coords[1].x {
					for y in coords[0].y..=coords[1].y {
						if self.get_tile_at(x, y).contents.len() != 0 {
							return false;
						}
					}
				}
				true
			}

			pub fn fill(&mut self, coord_1: Coordinate, coord_2: Coordinate, tile: Tile) {
				let coords = crate::render::canvas::sort_coordinates(coord_1, coord_2);
				for x in coords[0].x..=coords[1].x {
					for y in coords[0].y..=coords[1].y {
						self.set_tile(x, y, tile.clone());
					}
				}
			}
		}

		#[derive(Clone)]
		pub struct Tile {
			pub contents: Vec<WorldObject>,
		}

		impl Tile {
			pub fn new(tile: Option<WorldObject>) -> Tile {
				Tile {
					contents: match tile {
						Some(obj) => vec![obj],
						None => vec![],
					},
				}
			}

			pub fn wall() -> Tile {
				Tile::new(Some(WorldObject::WALL))
			}

			pub fn contains_wall(&self) -> bool {
				for obj in &self.contents {
					if matches!(obj, WorldObject::WALL) {
						return true;
					}
				}
				false
			}

			pub fn get_char(&self) -> char {
				for obj in &self.contents {
					match obj.get_char() {
						Some(c) => return c,
						None => {}
					}
				}
				' '
			}

			pub fn get_bgcolor(&self) -> Color {
				for obj in &self.contents {
					let color = obj.get_bgcolor();
					if !matches!(color, Color::Black) {
						return color;
					}
				}
				Color::Black
			}
		}

		#[derive(Copy, Clone)]
		pub enum WorldObject {
			PLAYER,
			WALL,
		}

		impl WorldObject {
			fn get_char(&self) -> Option<char> {
				match &self {
					WorldObject::PLAYER => Some('O'),
					WorldObject::WALL => Some(' '),
				}
			}
			fn get_bgcolor(&self) -> Color {
				match &self {
					WorldObject::WALL => Color::White,
					_ => Color::Black,
				}
			}
		}
	}
}
