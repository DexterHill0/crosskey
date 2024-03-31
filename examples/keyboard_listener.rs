use crosskey::KeyboardListener;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let keyboard_listener =
        KeyboardListener::attatch(&window).expect("failed to make keyboard listener");
    dbg!(&keyboard_listener);

    std::thread::spawn(|| {
        if let Err(e) = KeyboardListener::try_recv(|e| match e {
            crosskey::Event::Press { key, repeat_count } => {
                if repeat_count > 0 {
                    println!("repeating press!: {key} x {repeat_count}");
                } else {
                    println!("press event!: {key}");
                }
            },
            crosskey::Event::Release(key) => {
                println!("release event!: {key}");
            },
        }) {
            panic!("error while receiving: {e}")
        }
    });

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                elwt.exit();
            },
            Event::AboutToWait => {
                window.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {},
            _ => (),
        })
        .unwrap();
}
