#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod graph;
mod dom;


use std::cell::UnsafeCell;

use dom::{Circle, Dom, DomElement, Event, Point, Line};
use graph::{ToDom, ToSvg};

slint::include_modules!();
fn main () {
    let vdom = std::rc::Rc::new(std::cell::RefCell::new(Dom::new()));

    // let custom_slint_image = slint::Image::load_from_svg_data(vdom.to_svg().to_string().as_bytes()).unwrap();
    // let custom_slint_image = slint::Image::from_rgba8(vdom.to_img());
    // Create Slint window and set the image data
    let window = GraphWindow::new().unwrap();

    let vdom_clone = vdom.clone();
    window.on_clicked(move |x, y| {
        let vdom = vdom_clone.borrow_mut();
        println!("x: {x}, y: {y}");
        vdom.dispatch_event(Event::new(x, y)); 
    });
    let vdom_clone = vdom.clone();
    let weak_window = window.as_weak();
    window.on_dimensions_changed(move |width, height| {
        let mut vdom = vdom_clone.borrow_mut();

        let mut elements = Vec::new();
        elements.push(Some( DomElement::new(1, format!("Node 2"), Box::new(Circle::new( width- 40.0,  20.0, 10.0)))));    
        elements.push(Some( DomElement::new(2, format!("Node 3"), Box::new(Circle::new( 40.0,  height - 20.0, 10.0)))));
        elements.push(Some( DomElement::new(3, format!("Node 4"), Box::new(Circle::new( 45.0,  height - 20.0, 10.0)))));

    
        let el = &mut elements[0];
        match el {
            Some(el) => {
                el.register_event_handler(Box::new(|event| {
                    println!("Event: {:?}", event);
                }));
            }
            None => {}
        }
        vdom.children = elements;
        let img = vdom.to_img(width as u32, height as u32);
        let new_frame_custom_slint_image = slint::Image::from_rgba8(img);
        weak_window.unwrap().set_graph(new_frame_custom_slint_image);
    });
    
    vdom.borrow_mut().children.push(Some(DomElement::new(0, format!("Node 1"), Box::new(Line::new(Point::new(10.0, 10.0), Point::new(20.0, 20.0))))));
    window.run().unwrap();

}

