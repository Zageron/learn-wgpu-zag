use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod state;
use state::State;

fn run(
    event_loop: EventLoop<()>,
    window: Window,
    _swapchain_format: wgpu::TextureFormat,
    new_state: state::State,
) {
    let mut state = new_state;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { ref event, .. } if !state.input(event) => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(size) => state.resize(*size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(**new_inner_size)
            }
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    state: ElementState::Pressed,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        },
        Event::RedrawRequested(_) => {
            state.update();
            state.render();
        }
        Event::RedrawEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    use futures::executor::block_on;
    let state: State = block_on(State::new(&window));

    run(
        event_loop,
        window,
        wgpu::TextureFormat::Bgra8UnormSrgb,
        state,
    );
}
