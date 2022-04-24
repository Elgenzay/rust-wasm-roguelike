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
use dungeon::dungeon::Dungeon;
use dungeon::dungeon::DungeonConfig;

use mut_static::MutStatic;

use json::object;
use json::stringify;

use bresenham::Bresenham;

lazy_static! {
	pub static ref PLAYER: MutStatic<Player> = {
		let dungeon = Dungeon::new(DungeonConfig::default());
		MutStatic::from(Player {
			area: dungeon.area,
			discovered_area: Area::new(None),
			location: dungeon.spawn_point,
			canvas: Canvas::new(50, 110),
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
				if matches!(canvas_unit.on_click, Action::Move(_)) {
					obj.insert("m", true).unwrap();
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
	let edge = player.canvas.width - 1;
	let top = player.canvas.height - 1;
	let mut canvases = vec![];
	match canvas_unit_at_click.on_click {
		Action::Move(coord) => {
			for (x, y) in Bresenham::new(player.location.as_tuple(), coord.as_tuple()) {
				player.location = Coordinate::new(x as i32, y as i32);
				draw_area(
					&mut player,
					Coordinate::new(1, 1),
					Coordinate::new(edge, top),
				);
				canvases.push(player.canvas.clone());
			}
			player.location = coord;
			draw_area(
				&mut player,
				Coordinate::new(1, 1),
				Coordinate::new(edge, top),
			);
			canvases.push(player.canvas.clone()); //TODO use closure
		}
		_ => {
			draw_area(
				&mut player,
				Coordinate::new(1, 1),
				Coordinate::new(edge, top),
			);
			canvases.push(player.canvas.clone());
		}
	};
	canvas_vector_to_string(canvases)
}
