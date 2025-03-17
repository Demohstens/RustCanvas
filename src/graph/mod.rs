#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub name: String,
}

#[derive(Debug)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: f64,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    pub fn insert_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

impl Node {
    pub fn new(id: usize, name: String) -> Self {
        Self { id, name }
    }
}

impl Edge {
    pub fn new(from: usize, to: usize, weight: f64) -> Self {
        Self { from, to, weight }
    }
}

pub trait ToSvg {
    fn to_svg(&self) -> String;
}

impl ToSvg for Graph { 
    fn to_svg(&self) -> String {
        let path = svg::node::element::Circle::new();
        let mut paths = vec![path];
        for node in &self.nodes {
            let new_path = svg::node::element::Circle::new()
                .set("cx", node.id * 10)
                .set("cy", node.id * 10)
                .set("r", node.id * 1)
                .set("fill", "blue");
            paths.push(new_path);
        }
        let mut doc: svg::Document = svg::Document::new();
        for path in paths {
            doc = doc.add(path);
        }
        doc.to_string()
    }
}