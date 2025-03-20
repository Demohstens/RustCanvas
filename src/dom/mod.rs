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
        let mut svg = svg::Document::new();
        for child in &self.children {
            let mut paths: Vec<Box<dyn svg::Node>> = Vec::new();
            match child {
                Some(child) => {
                    let new_path = child.data.to_svg();
                    paths.push(new_path);
                    
                }
                None => {}
            }
            for path in paths.iter() {
                svg = svg.add(path.clone());
            }   
        }
        svg
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
}
#[derive(Debug)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
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
}

pub trait Shape {
    fn inside(&self, p: Point) -> bool;
    fn to_svg(&self) -> Box<dyn svg::Node>;
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