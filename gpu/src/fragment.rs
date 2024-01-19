use std::borrow::Cow;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub async fn run(event_loop: EventLoop<()>, window: Window) {
    let size = window.inner_size();

    let instance = wgpu::Instance::default();

    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: std::mem::size_of::<u32>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let compute_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: std::mem::size_of::<(f32, f32, f32)>() as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &buffer,
                offset: 0,
                size: None,
            }),
        }],
    });

    // let compute_bind_group_layout =
    //     device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    //         label: Some("compute_bind_group_layout"),
    //         entries: &[wgpu::BindGroupLayoutEntry {
    //             binding: 0,
    //             visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
    //             ty: wgpu::BindingType::Buffer {
    //                 ty: wgpu::BufferBindingType::Storage { read_only: false },
    //                 has_dynamic_offset: false,
    //                 min_binding_size: None,
    //             },
    //             count: None,
    //         }],
    //     });
    // let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    //     label: None,
    //     layout: &bind_group_layout,
    //     entries: &[wgpu::BindGroupEntry {
    //         binding: 0,
    //         resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
    //             buffer: &buffer,
    //             offset: 0,
    //             size: None,
    //         }),
    //     }],
    // });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = wgpu::TextureFormat::Rgba8Unorm; //swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });
    //
    // let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    //     label: None,
    //     bind_group_layouts: &[&compute_bind_group_layout],
    //     push_constant_ranges: &[],
    // });
    //
    // let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
    //     label: None,
    //     layout: Some(&compute_pipeline_layout),
    //     module: &shader,
    //     entry_point: "cs_main",
    // });

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    let mut count = 0u32;

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&instance, &adapter, &shader, &pipeline_layout);

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Reconfigure the surface with the new size
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);
                // On macos the window needs to be redrawn manually after resizing
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                // {
                //     let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                //         label: None,
                //         timestamp_writes: None,
                //     });
                //
                //     cpass.set_bind_group(0, &compute_bind_group, &[]);
                //     cpass.set_pipeline(&compute_pipeline);
                //     cpass.dispatch_workgroups(1, 1, 1);
                // }
                //
                // encoder.copy_buffer_to_buffer(&compute_buffer, 0, &buffer, 0, 12);
                //

                queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&[count]));

                count += 1;

                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    rpass.set_bind_group(0, &bind_group, &[]);
                    rpass.draw(0..3, 0..1);
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
