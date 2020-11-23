pub use self::state::State;

mod state {
    use winit::{event::WindowEvent, window::Window};

    pub struct State {
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        swap_chain_descriptor: wgpu::SwapChainDescriptor,
        swap_chain: wgpu::SwapChain,
        size: winit::dpi::PhysicalSize<u32>,
    }

    impl State {
        // Creating some of the wgpu types requires async code
        pub async fn new(window: &Window) -> Self {
            let size = window.inner_size();

            let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
            let surface = unsafe { instance.create_surface(window) };
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                })
                .await
                .unwrap();

            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                        shader_validation: true,
                    },
                    None,
                )
                .await
                .unwrap();

            let swap_chain_descriptor = wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Fifo,
            };

            let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

            return Self {
                surface,
                device,
                queue,
                swap_chain_descriptor,
                swap_chain,
                size,
            };
        }

        pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
            self.size = new_size;
            self.swap_chain_descriptor.width = new_size.width;
            self.swap_chain_descriptor.height = new_size.height;
            self.swap_chain = self
                .device
                .create_swap_chain(&self.surface, &self.swap_chain_descriptor);
        }

        pub fn input(&mut self, _event: &WindowEvent) -> bool {
            return false;
        }

        pub fn update(&mut self) {
            let frame = self
                .swap_chain
                .get_current_frame()
                .expect("Timeout getting texture")
                .output;

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 0.0,
                            }),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
            }

            self.queue.submit(std::iter::once(encoder.finish()));
        }
        // {} drop(_render_pass);

        pub fn render(&mut self) {
            //todo!()
        }
    }
}
