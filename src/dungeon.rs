pub mod dungeon {

	use crate::engine::engine::Coordinate;
	use crate::world::world::area::Area;
	use crate::world::world::region::Region;
	use rand::Rng;

	pub fn new_bsp_dungeon(config: DungeonConfig) -> Area {
		let mut dungeon = SubDungeon::new(Region::new(
			config.dungeon_width,
			config.dungeon_height,
			Coordinate::new(0, 0),
		));
		let mut area = Area::new(None);
		dungeon.new_bsp_dungeon_recursive(&mut area, &config, 0, SplitDirection::Random);
		area
	}

	struct SubDungeon {
		children: Option<Box<[SubDungeon; 2]>>,
		region: Region,
		room: Option<Region>,
	}

	enum SplitDirection {
		Vertical,
		Horizontal,
		Random,
	}

	impl SubDungeon {
		fn new(region: Region) -> SubDungeon {
			SubDungeon {
				children: None,
				region,
				room: None,
			}
		}

		fn new_bsp_dungeon_recursive(
			&mut self,
			area: &mut Area,
			config: &DungeonConfig,
			iteration: i8,
			split_direction: SplitDirection,
		) {
			let vertical = match split_direction {
				SplitDirection::Vertical => true,
				SplitDirection::Horizontal => false,
				SplitDirection::Random => rand::random(),
			};
			let (region_length, min_room_length) = if vertical {
				(self.region.height, config.min_room_height)
			} else {
				(self.region.width, config.min_room_width)
			};
			let min_child_length = (region_length as f32
				* (((100.0 - config.subdungeon_random_split_range as f32) / 2.0) * 0.01))
				as i32;
			if min_child_length < min_room_length {
				if matches!(split_direction, SplitDirection::Random) {
					if vertical {
						self.new_bsp_dungeon_recursive(
							area,
							config,
							iteration,
							SplitDirection::Horizontal,
						);
					} else {
						self.new_bsp_dungeon_recursive(
							area,
							config,
							iteration,
							SplitDirection::Vertical,
						);
					}
				}
				return;
			}
			let rand = rand::thread_rng()
				.gen_range(min_child_length + 1..=region_length - min_child_length);
			let new_regions: (Region, Region) = if vertical {
				(
					Region::new(self.region.width, rand - 1, self.region.position),
					Region::new(
						self.region.width,
						self.region.height - rand,
						Coordinate::new(self.region.position.x, self.region.position.y + rand),
					),
				)
			} else {
				(
					Region::new(rand - 1, self.region.height, self.region.position),
					Region::new(
						self.region.width - rand,
						self.region.height,
						Coordinate::new(self.region.position.x + rand, self.region.position.y),
					),
				)
			};
			self.children = Some(Box::new([
				SubDungeon {
					children: None,
					region: new_regions.0,
					room: None,
				},
				SubDungeon {
					children: None,
					region: new_regions.1,
					room: None,
				},
			]));
			let iteration = iteration + 1;
			if iteration < config.max_split_iterations {
				match &mut self.children {
					Some(children) => {
						for i in 0..2 {
							children[i].new_bsp_dungeon_recursive(
								area,
								config,
								iteration,
								SplitDirection::Random,
							);
						}
						if !matches!(children[0].children, None) {
							let child_2_rooms = children[1].get_rooms();
							let mut hallway_groups = vec![];
							for room_1 in children[0].get_rooms() {
								for room_2 in &child_2_rooms {
									let valid_hallways = area.get_valid_hallways(&room_1, &room_2);
									if valid_hallways.len() != 0 {
										hallway_groups.push(valid_hallways);
									}
								}
							}
							area.create_hallway_from_valid(
								&hallway_groups
									[rand::thread_rng().gen_range(0..hallway_groups.len())],
							);
						}
					}
					None => (),
				}
			} else {
				match &mut self.children {
					Some(children) => {
						for i in 0..2 {
							// FULL REGION DEBUGGING
							//	children[i].room = Some(Region::new(
							//		children[i].region.width,
							//		children[i].region.height,
							//		Coordinate::new(
							//			children[i].region.position.x,
							//			children[i].region.position.y,
							//		),
							//	));

							let mut width = rand::thread_rng()
								.gen_range(config.min_room_width..=children[i].region.width);
							let mut height = rand::thread_rng()
								.gen_range(config.min_room_height..=children[i].region.height);
							if height > width * 3 {
								height = width * 3;
							}
							if width > height * 3 {
								width = height * 3;
							}
							let pos_x = if children[i].region.position.x
								== children[i].region.get_edge_x() - width + 1
							{
								children[i].region.position.x
							} else {
								rand::thread_rng().gen_range(
									children[i].region.position.x
										..=children[i].region.get_edge_x() - width,
								)
							};
							let pos_y = if children[i].region.position.y
								== children[i].region.get_top_y() - height + 1
							{
								children[i].region.position.y
							} else {
								rand::thread_rng().gen_range(
									children[i].region.position.y
										..=children[i].region.get_top_y() - height,
								)
							};
							children[i].room =
								Some(Region::new(width, height, Coordinate::new(pos_x, pos_y)));
							area.place_region(&children[i].room.as_ref().unwrap());
						}
						area.create_hallway(
							&children[0].room.as_ref().unwrap(),
							&children[1].room.as_ref().unwrap(),
						);
					}
					None => (),
				}
			}
		}

		fn get_rooms(&self) -> Vec<&Region> {
			let mut rooms = vec![];

			match &self.room {
				Some(room) => rooms.push(room.clone()),
				None => (),
			}
			match &self.children {
				Some(children) => {
					for i in 0..2 {
						let mut vec = children[i].get_rooms();
						rooms.append(&mut vec);
					}
				}
				None => (),
			}
			rooms
		}
	}

	pub struct DungeonConfig {
		dungeon_width: i32,
		dungeon_height: i32,
		subdungeon_random_split_range: i8,
		max_split_iterations: i8,
		min_room_width: i32,
		min_room_height: i32,
	}

	impl DungeonConfig {
		pub fn default() -> DungeonConfig {
			DungeonConfig {
				dungeon_width: 150,
				dungeon_height: 50,
				subdungeon_random_split_range: 25,
				max_split_iterations: 4,
				min_room_width: 6,
				min_room_height: 6,
			}
		}
	}
}
