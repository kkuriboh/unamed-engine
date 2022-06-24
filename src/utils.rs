use std::{collections::HashSet, f64::consts::PI};

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
	CIRCLE,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CollisionBody {
	shape: CollisionShape,
	dimensions: Vec2<i32>,
	collision_groups: HashSet<i32>,
	pos: Vec2<f64>,
	rotation: f64,
}

#[wasm_bindgen]
impl CollisionBody {
	#[wasm_bindgen(constructor)]
	pub fn new(
		shape: CollisionShape,
		width: i32,
		height: i32,
		x: f64,
		y: f64,
		rotation: Option<f64>,
		collision_group: Option<i32>,
	) -> Self {
		Self {
			shape,
			dimensions: vec2(width, height),
			pos: vec2(x, y),
			rotation: rotation.unwrap_or(0.0),
			collision_groups: if let Some(collision_group) = collision_group {
				let mut collision_groups = HashSet::new();
				collision_groups.insert(collision_group);
				collision_groups
			} else {
				HashSet::new()
			},
		}
	}
	pub fn get_shape(&self) -> CollisionShape {
		self.shape
	}
	pub fn add_collision_group(&mut self, group: i32) {
		self.collision_groups.insert(group);
	}
	pub fn remove_collision_group(&mut self, group: i32) {
		self.collision_groups.remove(&group);
	}
	pub fn check_collision_group(&mut self, group: i32) -> bool {
		self.collision_groups.contains(&group)
	}
	pub fn set_rotation(&mut self, degres: f64) {
		self.rotation = degres;
	}
}

impl CollisionBody {
	pub fn get_collision_groups(&self) -> &HashSet<i32> {
		&self.collision_groups
	}
	pub fn get_edges(&self, parent_pos: Vec2<f64>) -> Vec<Edge> {
		match self.shape {
			CollisionShape::RECT => {
				if self.rotation == 0. {
					return vec![
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
					];
				}
				let rad = self.rotation * PI / 180.;
				let sin = rad.sin();
				let cos = rad.cos();
				let pivot = vec2(
					parent_pos.x + self.pos.x + self.dimensions.x as f64 * 0.5,
					parent_pos.y + self.pos.y + self.dimensions.y as f64 * 0.5,
				);
				vec![
					Edge /* top */ {
						start: rotate_vertex(pivot, sin, cos, vec2(parent_pos.x + self.pos.x, parent_pos.y + self.pos.y)),
						end: rotate_vertex(pivot, sin, cos, vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64, parent_pos.y + self.pos.y)),
					},
					Edge /* right */ {
						start: rotate_vertex(pivot, sin, cos, vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64, parent_pos.y + self.pos.y)),
						end: rotate_vertex(pivot, sin, cos, vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64, parent_pos.y + self.pos.y + self.dimensions.y as f64)),
					},
					Edge /* bottom */ {
						start: rotate_vertex(pivot, sin, cos, vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64, parent_pos.y + self.pos.y + self.dimensions.y as f64)),
						end: rotate_vertex(pivot, sin, cos, vec2(parent_pos.x + self.pos.x, parent_pos.y + self.pos.y + self.dimensions.y as f64)),
					},
					Edge /* left */ {
						start: rotate_vertex(pivot, sin, cos, vec2(parent_pos.x + self.pos.x, parent_pos.y + self.pos.y + self.dimensions.y as f64)),
						end: rotate_vertex(pivot, sin, cos, vec2(parent_pos.x + self.pos.x, parent_pos.y + self.pos.y)),
					},
				]
			}
			CollisionShape::CIRCLE => vec![
				Edge /* radius */ {
					start: vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64 * 0.5, parent_pos.y + self.pos.y + self.dimensions.y as f64 / 2.),
					end: vec2(parent_pos.x + self.pos.x + self.dimensions.x as f64, parent_pos.y + self.pos.y + self.dimensions.y as f64 * 0.5),
				},
				Edge /* pivot */ {
					start: vec2(0., 0.),
					end: vec2(0., 0.)
				},
			],
		}
	}
}

fn rotate_vertex(pivot: Vec2<f64>, sin: f64, cos: f64, point: Vec2<f64>) -> Vec2<f64> {
	let point = vec2(point.x - pivot.x, point.y - pivot.y);
	vec2(
		cos * point.x - sin * point.y + pivot.x,
		sin * point.x + cos * point.y + pivot.y,
	)
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Element {
	collision_body: Vec<CollisionBody>,
	pos: Vec2<f64>,
}

impl Default for Element {
	fn default() -> Self {
		Self {
			collision_body: vec![],
			pos: vec2(0f64, 0f64),
		}
	}
}

impl Element {
	pub fn get_pos(&self) -> Vec2<f64> {
		self.pos
	}
	pub fn get_collision_bodies(&self) -> Vec<CollisionBody> {
		self.collision_body.clone()
	}
	/*pub fn get_edges(&self) -> Vec<Edge> {
		self.collision_body
			.iter()
			.flat_map(|body| {
				let sin = body.rotation.sin();
				let cos = body.rotation.cos();
				let pivot = vec2(
					body.pos.x + body.dimensions.x as f64 * 0.5,
					body.pos.y + body.dimensions.y as f64 * 0.5,
				);
				match body.shape {
					CollisionShape::RECT => vec![
						Edge /* top */ {
							start: rotate_point(pivot, sin, cos, vec2(self.pos.x + body.pos.x, self.pos.y + body.pos.y)),
							end: rotate_point(pivot, sin, cos, vec2(self.pos.x + body.pos.x + body.dimensions.x as f64, self.pos.y + body.pos.y)),
						},
						Edge /* right */ {
							start: rotate_point(pivot, sin, cos, vec2(self.pos.x + body.pos.x + body.dimensions.x as f64, self.pos.y + body.pos.y)),
							end: rotate_point(pivot, sin, cos, vec2(self.pos.x + body.pos.x + body.dimensions.x as f64, self.pos.y + body.pos.y + body.dimensions.y as f64)),
						},
						Edge /* bottom */ {
							start: rotate_point(pivot, sin, cos, vec2(self.pos.x + body.pos.x + body.dimensions.x as f64, self.pos.y + body.pos.y + body.dimensions.y as f64)),
							end: rotate_point(pivot, sin, cos, vec2(self.pos.x + body.pos.x, self.pos.y + body.pos.y + body.dimensions.y as f64)),
						},
						Edge /* left */ {
							start: rotate_point(pivot, sin, cos, vec2(self.pos.x + body.pos.x, self.pos.y + body.pos.y + body.dimensions.y as f64)),
							end: rotate_point(pivot, sin, cos, vec2(self.pos.x + body.pos.x, self.pos.y + body.pos.y)),
						}
					],
					CollisionShape::CIRCLE => vec![
						Edge /* horizontal radius */ {
							start: vec2(self.pos.x + body.pos.x + body.dimensions.x as f64 * 0.5, self.pos.y + body.pos.y + body.dimensions.y as f64 / 2.),
							end: vec2(self.pos.x + body.pos.x + body.dimensions.x as f64, self.pos.y + body.pos.y + body.dimensions.y as f64 * 0.5),
						},
						Edge /* vertical radius */ {
							start: vec2(self.pos.x + body.pos.x + body.dimensions.x as f64 * 0.5, self.pos.y + body.pos.y + body.dimensions.y as f64 / 2.),
							end: vec2(self.pos.x + body.pos.x + body.dimensions.x as f64 * 0.5, self.pos.y + body.pos.y),
						}
					],
				}
			}).collect()
	}*/
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
		}
	}
	pub fn get_pos_json(&self) -> String {
		to_string(&self.pos).unwrap_throw()
	}
	pub fn add_collision_body(&mut self, body: CollisionBody) {
		self.collision_body.push(body);
	}
	pub fn draw_collisions(&self, engine: &Engine) {
		let context = engine.get_context();
		context.begin_path();
		self.collision_body
			.iter()
			.for_each(|body| match body.shape {
				CollisionShape::RECT => {
					body.get_edges(self.pos).iter().for_each(|edge| {
						context.line_to(edge.start.x, edge.start.y);
						context.line_to(edge.end.x, edge.end.y);
					});
					context
						.arc(
							self.pos.x + body.pos.x + body.dimensions.x as f64 * 0.5 - 2.,
							self.pos.y + body.pos.y + body.dimensions.y as f64 * 0.5 - 2.,
							2.,
							0.,
							PI * 2.,
						)
						.unwrap();
				}
				CollisionShape::CIRCLE => {
					context
						.arc(
							self.pos.x + body.pos.x + body.dimensions.x as f64 * 0.5,
							self.pos.y + body.pos.y + body.dimensions.y as f64 * 0.5,
							body.dimensions.x as f64 * 0.5,
							0.,
							PI * 2.,
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

	#[test]
	fn group_should_be_inserted() {
		let coll_body = CollisionBody::new(CollisionShape::RECT, 12, 12, 0f64, 0f64, None, Some(1));
		assert_eq!(coll_body.collision_groups.contains(&1), true);
	}

	#[test]
	fn group_should_be_removed() {
		let mut coll_body =
			CollisionBody::new(CollisionShape::RECT, 12, 12, 0f64, 0f64, None, Some(1));
		coll_body.remove_collision_group(1);
		assert_eq!(coll_body.collision_groups.contains(&1), false);
	}
}
