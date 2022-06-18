use wasm_bindgen::prelude::*;

use crate::utils::{Element, Vec2};
use crate::Engine;

#[wasm_bindgen]
pub fn get_collision_between_collider_and_moving_object(
	moving_element: String,
	collider: String,
	engine: &Engine,
) -> Result<bool, String> {
	let mut err = String::new();

	let moving_element = engine.get_element(moving_element).unwrap_or_else(|| {
		err.push_str("Element 1 doesn't exist");
		Element::default()
	});
	let collider = engine.get_element(collider).unwrap_or_else(|| {
		if !err.is_empty() {
			err = "Both elements don't exist".to_owned();
		} else {
			err.push_str("Element 2 doesn't exist");
		}
		Element::default()
	});

	if !err.is_empty() {
		return Err(err);
	}

	let mut collision_group = String::new();
	moving_element.get_collision_group().iter().for_each(|mg| {
		collider.get_collision_group().iter().for_each(|cg| {
			if cg.contains(mg) {
				collision_group.push_str(mg);
			}
		})
	});
	if !collision_group.is_empty() {
		return Ok(false);
	}

	let me_edges = moving_element.get_edges();
	let c_edges = collider.get_edges();

	let mut collides = false;
	for me_edge in me_edges {
		if collides {
			break;
		}
		for c_edge in &c_edges {
			let p0 = me_edge.start;
			let p1 = me_edge.end;
			let p2 = c_edge.start;
			let p3 = c_edge.end;

			collides = line_segments_collide(p0, p1, p2, p3);
		}
	}

	Ok(collides)
}

fn line_segments_collide(p0: Vec2<f64>, p1: Vec2<f64>, p2: Vec2<f64>, p3: Vec2<f64>) -> bool {
	let mut unknown_a = (p3.x - p2.x) * (p0.y - p2.y) - (p3.y - p2.y) * (p0.x - p2.x);
	let mut unknown_b = (p1.x - p0.x) * (p0.y - p2.y) - (p1.y - p0.y) * (p0.x - p2.x);
	let denominator = (p3.y - p2.y) * (p1.x - p0.x) - (p3.x - p2.x) * (p1.y - p0.y);

	if unknown_a == 0.0 && unknown_b == 0.0 && denominator == 0.0 {
		return false;
	}
	if denominator == 0.0 {
		return false;
	}

	unknown_a /= denominator;
	unknown_b /= denominator;

	unknown_a >= 0.0 && unknown_a <= 1.0 && unknown_b >= 0.0 && unknown_b <= 1.0
}

#[cfg(test)]
mod tests {
	use crate::{
		physics::get_collision_between_collider_and_moving_object,
		utils::{CollisionBody, CollisionShape},
		Engine,
	};

	#[test]
	fn basic_collision() {
		let mut engine = Engine::new(320, 240, 300.0, 10.0);
		let coll_body = CollisionBody::new(CollisionShape::RECT, 40, 40, 0., 0.);
		let coll_body2 = CollisionBody::new(CollisionShape::RECT, 40, 40, 0., 0.);

		engine.create_element("banana".to_owned(), Some(coll_body), 39.9, 0.0);
		engine
			.get_element("banana".to_string())
			.unwrap()
			.add_collision_group("teste".to_string());
		engine.create_element("abacate".to_owned(), Some(coll_body2), 0.0, 0.0);
		engine
			.get_element("abacate".to_string())
			.unwrap()
			.add_collision_group("teste".to_string());

		let collides = get_collision_between_collider_and_moving_object(
			"banana".to_owned(),
			"abacate".to_owned(),
			&engine,
		)
		.unwrap();

		assert_eq!(collides, true);
	}

	#[test]
	fn non_collision_vertical() {
		let mut engine = Engine::new(320, 240, 300.0, 10.0);
		let coll_body = CollisionBody::new(CollisionShape::RECT, 40, 40, 0., 0.);
		let coll_body2 = CollisionBody::new(CollisionShape::RECT, 40, 40, 0., 0.);

		engine.create_element("banana".to_owned(), Some(coll_body), 41.0, 0.0);
		engine
			.get_element("banana".to_string())
			.unwrap()
			.add_collision_group("teste".to_string());
		engine.create_element("abacate".to_owned(), Some(coll_body2), 0.0, 0.0);
		engine
			.get_element("abacate".to_string())
			.unwrap()
			.add_collision_group("teste".to_string());

		let collides = get_collision_between_collider_and_moving_object(
			"banana".to_owned(),
			"abacate".to_owned(),
			&engine,
		)
		.unwrap();

		assert_eq!(collides, false);
	}

	#[test]
	fn non_collision_horizontal() {
		let mut engine = Engine::new(320, 240, 300.0, 10.0);
		let coll_body = CollisionBody::new(CollisionShape::RECT, 40, 40, 0., 0.);
		let coll_body2 = CollisionBody::new(CollisionShape::RECT, 40, 40, 0., 0.);

		engine.create_element("banana".to_owned(), Some(coll_body), 0.0, 41.0);
		engine
			.get_element("banana".to_string())
			.unwrap()
			.add_collision_group("teste".to_string());
		engine.create_element("abacate".to_owned(), Some(coll_body2), 0.0, 0.0);
		engine
			.get_element("abacate".to_string())
			.unwrap()
			.add_collision_group("teste".to_string());

		let collides = get_collision_between_collider_and_moving_object(
			"banana".to_owned(),
			"abacate".to_owned(),
			&engine,
		)
		.unwrap();

		assert_eq!(collides, false);
	}
}
