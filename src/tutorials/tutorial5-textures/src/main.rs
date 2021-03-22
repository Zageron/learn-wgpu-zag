use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod state;
use state::State;

#[allow(dead_code, deprecated, unused_variables)]
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
            WindowEvent::KeyboardInput { input, .. } => {
                if let KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    state: ElementState::Pressed,
                    ..
                } = input
                {
                    *control_flow = ControlFlow::Exit
                }
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
                modifiers,
            } => {}
            WindowEvent::Moved(_) => {}
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::ReceivedCharacter(_) => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::CursorEntered { device_id } => {}
            WindowEvent::CursorLeft { device_id } => {}
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
                modifiers,
            } => {}
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
                modifiers,
            } => {}
            WindowEvent::TouchpadPressure {
                device_id,
                pressure,
                stage,
            } => {}
            WindowEvent::AxisMotion {
                device_id,
                axis,
                value,
            } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ThemeChanged(_) => {}
        },
        Event::RedrawRequested(_) => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Recreate the swap_chain if lost
                Err(wgpu::SwapChainError::Lost) => state.resize(state.size()),
                // The system is out of memory, we should probably quit
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
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
