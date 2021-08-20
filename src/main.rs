use futures::executor::block_on;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use voxel_lib::State;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut state = block_on(State::new(&window));

    let mut last_render_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_window_id) => {
            let now = std::time::Instant::now();
            let dt = now - last_render_time;
            last_render_time = now;
            state.update(dt);
            match state.render() {
                Ok(_) => {}
                // recreate swap_chain if lost
                Err(wgpu::SwapChainError::Lost) => state.create_swap_chain(),
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // all other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => println!("{:?}", e),
            }
        }
        Event::DeviceEvent { ref event, .. } => {
            state.input(event);
        }
        Event::MainEventsCleared => {
            // all events have been handled
            window.request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // new_inner_size is &&mut so we have to dereference twice
                    state.resize(**new_inner_size);
                }
                _ => {}
            }
        }
        _ => {}
    });
}
