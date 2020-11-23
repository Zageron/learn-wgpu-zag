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
        clear_color: wgpu::Color,
        render_pipeline: wgpu::RenderPipeline,
    }

    impl State {
        pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
            return self.size;
        }

        pub fn update_clear_color(&mut self, x_modifier: f64, y_modifier: f64) {
            self.clear_color = wgpu::Color {
                r: x_modifier,
                g: y_modifier,
                b: 1.0,
                a: 0.0,
            };
        }

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

            let swap_chain_descriptor: wgpu::SwapChainDescriptor = wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Fifo,
            };

            let swap_chain: wgpu::SwapChain =
                device.create_swap_chain(&surface, &swap_chain_descriptor);

            let clear_color: wgpu::Color = wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 0.0,
            };

            // Shader Pipeline
            let vs_module = device.create_shader_module(wgpu::include_spirv!("shader.vert.spv"));
            let fs_module = device.create_shader_module(wgpu::include_spirv!("shader.frag.spv"));

            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                    clamp_depth: false,
                }),
                color_states: &[wgpu::ColorStateDescriptor {
                    format: swap_chain_descriptor.format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

            return Self {
                surface,
                device,
                queue,
                swap_chain_descriptor,
                swap_chain,
                size,
                clear_color,
                render_pipeline,
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

        pub fn update(&mut self) {}

        pub fn render(&mut self) {
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
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.clear_color),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.draw(0..3, 0..1);
            }
            // {} drop(_render_pass);

            self.queue.submit(std::iter::once(encoder.finish()));
        }
    }
}
