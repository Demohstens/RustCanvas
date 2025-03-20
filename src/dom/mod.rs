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
                    match &child.onclick {
                        Some(handler) => {
                            handler(ev);
                            return;
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
    }
}

impl ToSvg for Dom {
    fn to_svg(&self) -> svg::Document {
        let mut paths: Vec<Box<dyn svg::Node>> = Vec::new();
        let mut svg = svg::Document::new();
        for child in &self.children {
            match child {
                Some(child) => {
                    match &child.el_type {
                        Shape::Circle { cx, cy, r } => {
                            let new_path = svg::node::element::Circle::new()
                                .set("cx", *cx)
                                .set("cy", *cy)
                                .set("r", *r);
                            paths.push(Box::new(new_path));
                        }
                        Shape::Rectangle { x, y, width, height } => {
                            let new_path = svg::node::element::Rectangle::new()
                                .set("x", *x)
                                .set("y", *y)
                                .set("width", *width)
                                .set("height", *height);
                            paths.push(Box::new(new_path));
                        }
                        Shape::Line { x1, y1, x2, y2 } => {
                            let new_path = svg::node::element::Line::new()
                                .set("x1", *x1)
                                .set("y1", *y1)
                                .set("x2", *x2)
                                .set("y2", *y2);
                            paths.push(Box::new(new_path));
                        }
                    }
                    for path in paths.iter() {
                        svg = svg.add(path.clone());
                    }
                }
                None => {}
            }
        }
        svg
    }
}

#[derive(Debug)]
pub struct Event {
    x: f32,
    y: f32,
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
    pub el_type: Shape,
    pub children: Vec<Option<DomElement>>,
}

impl DomElement {
    pub fn new(id: usize, name: String, el_type: Shape) -> Self {
        Self {
            id,
            name,
            onclick: None,
            el_type,
            children: Vec::new(),
        }
    }
    pub fn register_event_handler(&mut self, event: Box<dyn Fn(Event)>) {
        self.onclick = Some(event);
    }
}
pub enum Shape {
    Circle { cx: i32, cy: i32, r: i32 },
    Rectangle { x: i32, y: i32, width: i32, height: i32 },
    Line { x1: i32, y1: i32, x2: i32, y2: i32 },
}