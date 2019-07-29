use std::collections::HashMap;
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn main() {
    let event_loop = EventLoop::new();

    // TODO(jon):
    // An application should be able to control the z-stacking order of its windows.
    // They should be able to be restored after minimize/restore actions.
    // We should have a set_active(window) method, which does not necessarily imply set_active_and_move_to_front.
    // A window should automatically gain focus on creation, in the order of window creation
    // and dispatch WindowEvent::Focused(true)
    // Some tool panel style windows should always defer their focus to their 'parent' window.

    let mut windows = HashMap::new();
    for _ in 0..2 {
        let window = Window::new(&event_loop).unwrap();
        println!("New window {:?}", window.id());
        windows.insert(window.id(), window);
    }

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event, window_id } => {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("Window {:?} has received the signal to close", window_id);

                        // This drops the window, causing it to close.
                        windows.remove(&window_id);

                        if windows.is_empty() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        let window = Window::new(&event_loop).unwrap();
                        println!("New window {:?}", window.id());
                        windows.insert(window.id(), window);
                    }
                    WindowEvent::Focused(got_focus) => {
                        if got_focus {
                            println!("Got focus: {:?}", window_id);
                        } else {
                            println!("Lost focus: {:?}", window_id);
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    })
}
