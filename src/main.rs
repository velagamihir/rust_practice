enum Event {
    KeyPress { ch: char },
    MouseClick { x: i32, y: i32 },
    Resize { width: u32, height: u32 },
    Quit,
}
fn main() {
    let event1 = Event::KeyPress { ch: 'A' };
    let event2 = Event::MouseClick { x: 90, y: 80 };
    let event3 = Event::Resize {
        width: 1800,
        height: 1600,
    };
    let event4 = Event::Quit;
    handle_event(event1);
    handle_event(event2);
    handle_event(event3);
    handle_event(event4);
}
fn handle_event(event: Event) {
    match event {
        Event::KeyPress { ch } => println!("Key Pressed: {}", ch),
        Event::MouseClick { x, y } => println!("Mouse clicked at ({},{})", x, y),
        Event::Resize { width, height } => println!("Window Resized to {}*{}", width, height),
        Event::Quit => println!("System quitting"),
    }
}
