use std::collections::HashMap;

use serde_json::to_string;
use wasm_bindgen::{prelude::*, JsCast};

use utils::{vec2, CollisionBody, Element, Vec2};
use web_sys::CanvasRenderingContext2d;

pub mod physics;
pub mod utils;

#[wasm_bindgen]
pub struct Engine {
	window_dimensions: Vec2<u32>,
	speed: f64,
	acceleration: f64,
	elements: HashMap<String, Element>,
	context: CanvasRenderingContext2d,
}

// non wasm functions
impl Engine {
	pub fn get_dimensions(&self) -> Vec2<u32> {
		self.window_dimensions
	}
}

#[wasm_bindgen]
impl Engine {
	#[wasm_bindgen(constructor)]
	pub fn new(window_width: u32, window_height: u32, speed: f64, acceleration: f64) -> Self {
		let document = web_sys::window().unwrap().document().unwrap();
		let canvas = document
			.get_element_by_id("root")
			.unwrap()
			.dyn_into::<web_sys::HtmlCanvasElement>()
			.map_err(|_| ())
			.unwrap();
		canvas.set_width(window_width);
		canvas.set_height(window_height);
		let context = canvas
			.get_context("2d")
			.unwrap()
			.unwrap()
			.dyn_into::<web_sys::CanvasRenderingContext2d>()
			.unwrap();

		Self {
			window_dimensions: vec2(window_width, window_height),
			speed,
			acceleration,
			elements: HashMap::new(),
			context,
		}
	}
	pub fn get_dimensions_json(&self) -> String {
		to_string(&self.window_dimensions).unwrap_throw()
	}
	pub fn get_speed(&self) -> f64 {
		self.speed
	}
	pub fn get_acceleration(&self) -> f64 {
		self.acceleration
	}
	pub fn add_element(&mut self, name: String, element: Element) {
		self.elements.insert(name, element);
	}
	pub fn create_element(&mut self, name: String, body: Option<CollisionBody>, x: f64, y: f64) {
		self.elements.insert(name, Element::new(body, x, y));
	}
	pub fn get_element(&self, name: String) -> Option<Element> {
		self.elements.get(&name).map(|element| element.to_owned())
	}
	pub fn get_context(&self) -> CanvasRenderingContext2d {
		self.context.clone()
	}
}
