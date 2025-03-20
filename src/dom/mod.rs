use std::fmt::Debug;

use image::ImageBuffer;
use serde::de;
use slint::{platform::software_renderer::TargetPixel, SharedPixelBuffer};
use svg::Node;

use crate::graph::ToSvg;


pub struct Dom {
    pub children: Vec<Option<DomElement>>, 
}

impl Dom {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
    pub fn dispatch_event(&self, ev: Event) -> () {
        // Recoursively find the first element that has an event handler and call it
        for child in &self.children {
            match child {
                Some(child) => {
                    if child.data.inside(Point::new(ev.x, ev.y)) {
                        println!("Event inside: {:?}", child.name);
                    match &child.onclick {
                        Some(handler) => {
                            handler(ev);
                            return;
                        }
                        None => {}
                    }}
                }
                None => {}
            }
        }
    }
}

impl ToSvg for Dom {
    fn to_svg(&self) -> svg::Document {
        let mut svg = svg::Document::new().set("viewBox", (0, 0, 100, 100)).set("width", 100).set("height", 100);
        println!("{:?}", svg.get_attributes());
        for child in &self.children {
            let mut paths: Vec<Box<dyn svg::Node>> = Vec::new();
            match child {
                Some(child) => {
                    let new_path = child.data.to_svg();
                    paths.push(new_path);
                    
                }
                None => {}
            }
            for path in paths.iter() {  // Example: Adjust as necessary
                svg = svg.add(path.clone());
            }   
        }
        svg
    }
}impl Dom {
    pub fn to_img(&self, width: u32, height: u32) -> SharedPixelBuffer<slint::Rgba8Pixel> {
        // Create a SharedPixelBuffer directly with the right size
    
        // Get a mutable slice to the buffer's data
        let mut img_bytes = Vec::with_capacity((width * height * 4) as usize);
        for y in 0..height {
            for x in 0..width {
                // White pixel (RGBA)
                img_bytes.push(0); // R
                img_bytes.push(0); // G
                img_bytes.push(0); // B
                img_bytes.push(0); // A
            }
        }
        for child in &self.children {
            match child {
                Some(child) => {
                    let shape = &child.data;
                    for y in 0..height {
                        for x in 0..width {
                            let p = Point::new(x as f32, y as f32);
                            if shape.inside(p) {
                                // Red pixel (RGBA)
                                img_bytes[(y * width + x) as usize * 4] = 255; // R
                                img_bytes[(y * width + x) as usize * 4 + 1] = 0; // G
                                img_bytes[(y * width + x) as usize * 4 + 2] = 0; // B
                                img_bytes[(y * width + x) as usize * 4 + 3] = 255; // A
                                
                            } 
                        }
                    }
                }
                None => {}
            }
        }
        
        SharedPixelBuffer::clone_from_slice(&img_bytes, width, height)
        }
}

#[derive(Debug)]
pub struct Event {
    pub x: f32,
    pub y: f32,
}
impl Event {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
pub struct DomElement {
    id: usize,
    pub name: String,
    pub onclick: Option<Box<dyn Fn(Event)>>,
    pub data: Box<dyn Shape>,
    pub children: Vec<Option<DomElement>>,
}

impl DomElement {
    pub fn new(id: usize, name: String, data: Box<dyn Shape>) -> Self {
        Self {
            id,
            name,
            onclick: None,
            data,
            children: Vec::new(),
        }
    }
    pub fn register_event_handler(&mut self, event: Box<dyn Fn(Event)>) {
        self.onclick = Some(event);
    }
}
#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub r: f32,
}
impl Circle {
    pub fn new(x: f32, y: f32, r: f32) -> Self {
        Self {
            center: Point::new(x, y),
            r,
        }
    }
}
impl Shape for Circle {
    fn inside(&self, p: Point) -> bool {
        let dx = p.x - self.center.x;
        let dy = p.y - self.center.y;
        dx * dx + dy * dy <= self.r * self.r
    }   
    fn to_svg(&self) -> Box<dyn svg::Node> {
        let circle = svg::node::element::Circle::new()
            .set("cx", self.center.x)
            .set("cy", self.center.y)
            .set("r", self.r)
            .set("fill", "red");
        Box::new(circle)
    }
    fn fmt(&self) -> String {
        format!("Circle at ({}, {}) with radius {}", self.center.x, self.center.y, self.r)
    }
}
#[derive(Debug)]
pub struct Rectangle {
    pub origin: Point,
    pub width: f32,
    pub height: f32,
}
impl Shape for Rectangle {
    fn inside(&self, p: Point) -> bool {
        p.x >= self.origin.x
            && p.x <= self.origin.x + self.width
            && p.y >= self.origin.y
            && p.y <= self.origin.y + self.height
    }
    fn to_svg(&self) -> Box<dyn svg::Node> {
        let rect = svg::node::element::Rectangle::new()
            .set("x", self.origin.x)
            .set("y", self.origin.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1);
        Box::new(rect)
    }
    fn fmt(&self) -> String {
        format!(
            "Rectangle at ({}, {}) with width {} and height {}",
            self.origin.x, self.origin.y, self.width, self.height
        )
    }
}
#[derive(Debug)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}
impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }
}
impl Shape for Line {
    fn inside(&self, p: Point) -> bool {
        p.x >= self.p1.x
            && p.x <= self.p2.x
            && p.y >= self.p1.y
            && p.y <= self.p2.y
    }
    fn to_svg(&self) -> Box<dyn svg::Node> {
        let path = svg::node::element::Path::new()
            .set("d", format!("M {} {} L {} {}", self.p1.x, self.p1.y, self.p2.x, self.p2.y))
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("fill", "none");
        Box::new(path)
    }
    fn fmt(&self) -> String {
        format!("Line from ({}, {}) to ({}, {})", self.p1.x, self.p1.y, self.p2.x, self.p2.y)
    }
}

pub trait Shape {
    fn inside(&self, p: Point) -> bool;
    fn to_svg(&self) -> Box<dyn svg::Node>;
    fn fmt(&self) -> String;
}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
