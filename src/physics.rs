use wasm_bindgen::prelude::*;

use crate::utils::{CollisionShape, Element, Vec2};
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

	let mut collides = false;
	moving_element.get_collision_bodies().iter().for_each(|me| {
		let me_edges = me.get_edges(moving_element.get_pos());
		collider.get_collision_bodies().iter().for_each(|ce| {
			let c_edges = ce.get_edges(collider.get_pos());
			if me
				.get_collision_groups()
				.intersection(ce.get_collision_groups())
				.count() == 0
			{
				collides = false;
			} else {
				match (me.get_shape(), ce.get_shape()) {
					(CollisionShape::RECT, CollisionShape::RECT) => {
						me_edges.iter().for_each(|me_edge| {
							let p0 = me_edge.start;
							let p1 = me_edge.end;
							c_edges.iter().for_each(|c_edge| {
								let p2 = c_edge.start;
								let p3 = c_edge.end;
								if line_segments_collide(p0, p1, p2, p3) {
									collides = true;
								}
							})
						});
					}
					(CollisionShape::CIRCLE, CollisionShape::CIRCLE) => {}
					(CollisionShape::RECT, CollisionShape::CIRCLE)
					| (CollisionShape::CIRCLE, CollisionShape::RECT) => {}
				}
			}
		})
	});

	Ok(collides)
}

fn line_segments_collide(p0: Vec2<f64>, p1: Vec2<f64>, p2: Vec2<f64>, p3: Vec2<f64>) -> bool {
	let mut t_a = (p3.x - p2.x) * (p0.y - p2.y) - (p3.y - p2.y) * (p0.x - p2.x);
	let mut t_b = (p1.x - p0.x) * (p0.y - p2.y) - (p1.y - p0.y) * (p0.x - p2.x);
	let denominator = (p3.y - p2.y) * (p1.x - p0.x) - (p3.x - p2.x) * (p1.y - p0.y);

	if t_a == 0.0 && t_b == 0.0 && denominator == 0.0 {
		return false;
	}
	if denominator == 0.0 {
		return false;
	}

	t_a /= denominator;
	t_b /= denominator;

	(0.0..=1.0).contains(&t_a) && t_b >= 0.0 && t_b <= 1.0
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
		let mut coll_body = CollisionBody::new(CollisionShape::RECT, 40, 40, 0., 0., None, Some(0));
		let mut coll_body2 = CollisionBody::new(CollisionShape::RECT, 40, 40, 0., 0., None, Some(0));
        coll_body.add_collision_group(1);
        coll_body2.add_collision_group(1);

		engine.create_element("banana".to_owned(), Some(coll_body), 39.9, 0.0);
		engine
			.get_element("banana".to_string())
			.unwrap();
		engine.create_element("abacate".to_owned(), Some(coll_body2), 0.0, 0.0);
		engine
			.get_element("abacate".to_string())
			.unwrap();

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
		let mut coll_body = CollisionBody::new(CollisionShape::RECT, 40, 40, 0., 0., None, Some(0));
		let mut coll_body2 = CollisionBody::new(CollisionShape::RECT, 40, 40, 0., 0., None, Some(0));
        coll_body.add_collision_group(1);
        coll_body2.add_collision_group(1);

		engine.create_element("banana".to_owned(), Some(coll_body), 41.0, 0.0);
		engine
			.get_element("banana".to_string())
			.unwrap();
		engine.create_element("abacate".to_owned(), Some(coll_body2), 0.0, 0.0);
		engine
			.get_element("abacate".to_string())
			.unwrap();

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
		let mut coll_body = CollisionBody::new(CollisionShape::RECT, 40, 40, 0., 0., None, Some(0));
		let mut coll_body2 = CollisionBody::new(CollisionShape::RECT, 40, 40, 0., 0., None, Some(0));
        coll_body.add_collision_group(1);
        coll_body2.add_collision_group(1);

		engine.create_element("banana".to_owned(), Some(coll_body), 0.0, 41.0);
		engine
			.get_element("banana".to_string())
			.unwrap();
		engine.create_element("abacate".to_owned(), Some(coll_body2), 0.0, 0.0);
		engine
			.get_element("abacate".to_string())
			.unwrap();

		let collides = get_collision_between_collider_and_moving_object(
			"banana".to_owned(),
			"abacate".to_owned(),
			&engine,
		)
		.unwrap();

		assert_eq!(collides, false);
	}
}
