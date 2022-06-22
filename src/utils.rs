use std::collections::HashSet;

use serde::Serialize;
use serde_json::to_string;
use wasm_bindgen::prelude::*;

use crate::Engine;

#[derive(Clone, Copy, Serialize, PartialEq)]
pub struct Vec2<T>
where
	T: Copy,
{
	pub x: T,
	pub y: T,
}

pub fn vec2<T>(x: T, y: T) -> Vec2<T>
where
	T: Copy,
{
	Vec2 { x, y }
}

#[derive(PartialEq, Serialize)]
pub struct Edge {
	pub start: Vec2<f64>,
	pub end: Vec2<f64>,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum CollisionShape {
	RECT,
	ELIPSE,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CollisionBody {
	shape: CollisionShape,
	dimensions: Vec2<i32>,
	pos: Vec2<f64>,
}

#[wasm_bindgen]
impl CollisionBody {
	#[wasm_bindgen(constructor)]
	pub fn new(shape: CollisionShape, width: i32, height: i32, x: f64, y: f64) -> Self {
		Self {
			shape,
			dimensions: vec2(width, height),
			pos: vec2(x, y),
		}
	}
	pub fn get_shape(&self) -> CollisionShape {
		self.shape
	}
}

impl CollisionBody {
	pub fn get_edges(&self, parent_pos: Vec2<f64>) -> Vec<Edge> {
		match self.shape {
			CollisionShape::RECT => vec![
				Edge /* top */ {
					start: vec2(parent_pos.x + self.pos.x, parent_pos.y + self.pos.y),
					end: vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64, parent_pos.y + self.pos.y),
				},
				Edge /* right */ {
					start: vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64, parent_pos.y + self.pos.y),
					end: vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64, parent_pos.y + self.pos.y + self.dimensions.y as f64),
				},
				Edge /* bottom */ {
					start: vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64, parent_pos.y + self.pos.y + self.dimensions.y as f64),
					end: vec2(parent_pos.x + self.pos.x, parent_pos.y + self.pos.y + self.dimensions.y as f64),
				},
				Edge /* left */ {
					start: vec2(parent_pos.x + self.pos.x, parent_pos.y + self.pos.y + self.dimensions.y as f64),
					end: vec2(parent_pos.x + self.pos.x, parent_pos.y + self.pos.y),
				},
			],
			CollisionShape::ELIPSE => vec![
				Edge /* horizontal radius */ {
					start: vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64 / 2., parent_pos.y + self.pos.y + self.dimensions.y as f64 / 2.),
					end: vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64, parent_pos.y + self.pos.y + self.dimensions.y as f64 / 2.),
				},
				Edge /* vertical radius */ {
					start: vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64 / 2., parent_pos.y + self.pos.y + self.dimensions.y as f64 / 2.),
					end: vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64 / 2., parent_pos.y + self.pos.y),
				},
			],
		}
	}
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Element {
	collision_body: Vec<CollisionBody>,
	pos: Vec2<f64>,
	collision_groups: HashSet<String>,
}

impl Default for Element {
	fn default() -> Self {
		Self {
			collision_body: vec![],
			pos: vec2(0f64, 0f64),
			collision_groups: HashSet::new(),
		}
	}
}

impl Element {
	pub fn get_pos(&self) -> Vec2<f64> {
		self.pos
	}
	pub fn get_collision_group(&self) -> HashSet<String> {
		self.collision_groups.clone()
	}
	pub fn get_collision_bodies(&self) -> Vec<CollisionBody> {
		self.collision_body.clone()
	}
	pub fn get_edges(&self) -> Vec<Edge> {
		self.collision_body
			.iter()
			.flat_map(|body| match body.shape {
				CollisionShape::RECT => vec![
					Edge /* top */ {
						start: vec2(self.pos.x + body.pos.x, self.pos.y + body.pos.y),
						end: vec2(self.pos.x + body.pos.x + body.dimensions.x as f64, self.pos.y + body.pos.y),
					},
					Edge /* right */ {
						start: vec2(self.pos.x + body.pos.x + body.dimensions.x as f64, self.pos.y + body.pos.y),
						end: vec2(self.pos.x + body.pos.x + body.dimensions.x as f64, self.pos.y + body.pos.y + body.dimensions.y as f64),
					},
					Edge /* bottom */ {
						start: vec2(self.pos.x + body.pos.x + body.dimensions.x as f64, self.pos.y + body.pos.y + body.dimensions.y as f64),
						end: vec2(self.pos.x + body.pos.x, self.pos.y + body.pos.y + body.dimensions.y as f64),
					},
					Edge /* left */ {
						start: vec2(self.pos.x + body.pos.x, self.pos.y + body.pos.y + body.dimensions.y as f64),
						end: vec2(self.pos.x + body.pos.x, self.pos.y + body.pos.y),
					}
				],
				CollisionShape::ELIPSE => vec![
					Edge /* horizontal radius */ {
						start: vec2(self.pos.x + body.pos.x + body.dimensions.x as f64 / 2., self.pos.y + body.pos.y + body.dimensions.y as f64 / 2.),
						end: vec2(self.pos.x + body.pos.x + body.dimensions.x as f64, self.pos.y + body.pos.y + body.dimensions.y as f64 / 2.),
					},
					Edge /* vertical radius */ {
						start: vec2(self.pos.x + body.pos.x + body.dimensions.x as f64 / 2., self.pos.y + body.pos.y + body.dimensions.y as f64 / 2.),
						end: vec2(self.pos.x + body.pos.x + body.dimensions.x as f64 / 2., self.pos.y + body.pos.y),
					}
				],
			})
			.collect()
	}
}

#[wasm_bindgen]
impl Element {
	#[wasm_bindgen(constructor)]
	pub fn new(collision_shape: Option<CollisionBody>, x: f64, y: f64) -> Self {
		Self {
			collision_body: if let Some(body) = collision_shape {
				vec![body]
			} else {
				vec![]
			},
			pos: vec2(x, y),
			collision_groups: HashSet::new(),
		}
	}
	pub fn get_pos_json(&self) -> String {
		to_string(&self.pos).unwrap_throw()
	}
	pub fn add_collision_group(&mut self, group: String) {
		self.collision_groups.insert(group);
	}
	pub fn add_collision_body(&mut self, body: CollisionBody) {
		self.collision_body.push(body);
	}
	pub fn remove_collision_group(&mut self, group: String) {
		self.collision_groups.remove(&group);
	}
	pub fn check_collision_group(&mut self, group: String) -> bool {
		self.collision_groups.contains(&group)
	}
	pub fn draw_collisions(&self, engine: &Engine) {
		let context = engine.get_context();
		context.begin_path();
		self.collision_body
			.iter()
			.for_each(|body| match body.shape {
				CollisionShape::RECT => {
					context.rect(
						self.pos.x + body.pos.x,
						self.pos.y + body.pos.y,
						body.dimensions.x as f64,
						body.dimensions.y as f64,
					);
				}
				CollisionShape::ELIPSE => {
					context
						.ellipse(
							body.pos.x + body.dimensions.x as f64,
							body.pos.y + body.dimensions.y as f64,
							body.dimensions.x as f64,
							body.dimensions.y as f64,
							0.0,
							0.0,
							std::f64::consts::PI * 2.0,
						)
						.unwrap();
				}
			});
		context.stroke();
	}
}

#[cfg(test)]
mod tests {
	use crate::utils::{CollisionBody, CollisionShape};

	use super::Element;

	#[test]
	fn group_should_be_inserted() {
		let coll_body = CollisionBody::new(CollisionShape::RECT, 12, 12, 0f64, 0f64);
		let mut element = Element::new(Some(coll_body), 0f64, 0f64);
		element.add_collision_group("avocado".to_string());
		assert_eq!(element.collision_groups.contains("avocado"), true);
	}

	#[test]
	fn group_should_be_removed() {
		let coll_body = CollisionBody::new(CollisionShape::RECT, 12, 12, 0f64, 0f64);
		let mut element = Element::new(Some(coll_body), 0f64, 0f64);
		element.add_collision_group("avocado".to_string());
		element.remove_collision_group("avocado".to_string());
		assert_eq!(element.collision_groups.contains("avocado"), false);
	}
}
