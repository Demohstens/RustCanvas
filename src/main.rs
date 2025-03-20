mod graph;
mod dom;

use dom::{Dom, DomElement, Shape, Event};
use graph::{ToDom, ToSvg};

slint::include_modules!();
fn main () {
    let mut graph = graph::Graph::new();
    let mut vdom = Dom::new();
    for i in 0..2 {
        let node = graph::Node::new(i, format!("Node {}", i));
        graph.insert_node(node);
    }

    let mut graph_dom = graph.to_dom();
    let el = &mut graph_dom.children[0];
    match el {
        Some(el) => {
            el.register_event_handler(Box::new(|event| {
                println!("Event: {:?}", event);
            }));
        }
        None => {}
    }
    let graph_svg = graph_dom.to_svg();

    let custom_slint_image = slint::Image::load_from_svg_data(graph_svg.to_string().as_bytes()).unwrap();
    // Create Slint window and set the image data
    let window = GraphWindow::new().unwrap();

    window.set_graph(custom_slint_image); // Pass the base64 string to the image source
    window.on_clicked(move |x, y| {
        println!("x: {x}, y: {y}");
        graph_dom.dispatch_event(Event::new(x, y)); 
    });
    
    window.run().unwrap();

}
