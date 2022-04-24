use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

mod engine;
use crate::engine::engine::draw_area;
use crate::engine::engine::Action;
use crate::engine::engine::Coordinate;
use crate::engine::engine::Player;

mod world;
use world::world::area::Area;

mod render;
use render::canvas::Canvas;
use render::canvas::Color;

mod dungeon;
use dungeon::dungeon::*;

use mut_static::MutStatic;

use json::object;
use json::stringify;

lazy_static! {
	pub static ref PLAYER: MutStatic<Player> = {
		MutStatic::from(Player {
			area: new_bsp_dungeon(dungeon::dungeon::DungeonConfig::default()),
			discovered_area: Area::new(None),
			location: Coordinate::new(50, 25),
			canvas: Canvas::new(40, 80),
		})
	};
}

fn canvas_vector_to_string(vec: Vec<Canvas>) -> String {
	let mut canvas_objects = vec![];
	for canvas in vec {
		let mut canvas_vec = vec![];
		for x in 0..canvas.width {
			let mut x_vec = vec![];
			for y in 0..canvas.height {
				let canvas_unit = canvas.get(x, y);
				let mut obj = object!(
					"c" => canvas_unit.character.to_string(),
				);
				if !matches!(canvas_unit.bg_color, Color::Black) {
					obj.insert("bg", canvas_unit.bg_color.as_string()).unwrap();
				}
				x_vec.push(obj);
			}
			canvas_vec.push(x_vec);
		}
		canvas_objects.push(canvas_vec);
	}
	let obj = object!(
		"canvases" => canvas_objects
	);
	stringify(obj)
}

#[wasm_bindgen]
pub fn click(x: i32, y: i32) -> String {
	let mut player = PLAYER.write().unwrap();
	let canvas_unit_at_click = player.canvas.get(x, y);
	match canvas_unit_at_click.on_click {
		Action::Move(coord) => {
			player.location = coord;
		}
		_ => (),
	};
	let edge = player.canvas.width - 1;
	let top = player.canvas.height - 1;
	draw_area(
		&mut player,
		Coordinate::new(1, 1),
		Coordinate::new(edge, top),
	);
	canvas_vector_to_string(vec![player.canvas.clone()])
}
