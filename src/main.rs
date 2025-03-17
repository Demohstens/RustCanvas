mod graph;

use graph::ToSvg;

slint::include_modules!();
fn main () {
    let mut graph = graph::Graph::new();
    
    for i in 0..10 {
        let node = graph::Node::new(i, format!("Node {}", i));
        graph.insert_node(node);
    }

    let slint_image = slint::Image::load_from_path(std::path::Path::new("media/cat.png")).unwrap();
    let graph_svg = graph.to_svg();

    let custom_slint_image = slint::Image::load_from_svg_data(graph_svg.as_bytes()).unwrap();
    // Create Slint window and set the image data
    let window = GraphWindow::new().unwrap();

    window.set_graph(custom_slint_image); // Pass the base64 string to the image source
    window.on_clicked(move |x, y| {
        println!("x: {x}, y: {y}");
    });
    window.run().unwrap();

}
