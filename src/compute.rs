use crate::HEIGHT;
use crate::WIDTH;
use crate::math::*;
use crate::World;

use std::borrow::Cow;
use renderdoc::RenderDoc;
use renderdoc::V110;
use wgpu::util::DeviceExt;

const BIG: u64 = 1;

pub async fn gpu_compute(world: &World) -> Option<Vec<Vec3>> {
    let mut acc = vec![Vec3::zero(); (WIDTH * HEIGHT) as usize];

    // Instantiates instance of WebGPU
    let instance = wgpu::Instance::default();

    // `request_adapter` instantiates the general connection to the GPU
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await?;

    // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
    //  `features` being the available features.
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .unwrap();

    // Loads the shader from WGSL
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    // Gets the size in bytes of the buffer.
    let size = (WIDTH * HEIGHT * std::mem::size_of::<Vec3>() as u32) as wgpu::BufferAddress;


    for i in 0..BIG {
        // Instantiates buffer without data.
        // `usage` of buffer specifies how it can be used:
        //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
        //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    
        // Instantiates buffer with data.
        // Usage allowing the buffer to be:
        //   A storage buffer (can be bound within a bind group and thus available to a shader).
        //   The destination of a copy.
        //   The source of a copy.
        let storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
    
        // A bind group defines how buffers are accessed by shaders.
        // It is to WebGPU what a descriptor set is to Vulkan.
        // `binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).
    
        // A pipeline specifies the operation of a shader
    
        // Instantiates the pipeline.
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &cs_module,
            entry_point: "main",
        });
    
        // Instantiates the bind group, once again specifying the binding of buffers.
        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            }],
        });
        // A command encoder executes one or many pipelines.
        // It is to WebGPU what a command buffer is to Vulkan.
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            // cpass.insert_debug_marker("compute collatz iterations");
            cpass.dispatch_workgroups(WIDTH / 16, HEIGHT / 16, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }
        // Sets adds copy operation to command encoder.
        // Will copy data from storage buffer on GPU to staging buffer on CPU.
        encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer, 0, size);
    
        // Submits command encoder for processing
        queue.submit(Some(encoder.finish()));

        let mut rd: RenderDoc<V110> = RenderDoc::new().expect("Unable to connect");

        rd.trigger_capture();
        match rd.get_capture(0) {
            Some((path, capture_time)) => println!("ID: 0, Path: {}, Captured: {:?}", path.display(), capture_time),
            None => println!("No capture found with ID of 0!"),
        }
        match rd.launch_replay_ui(true, None) {
            Ok(pid) => println!("Launched replay UI (PID {pid})"),
            Err(e) => eprintln!("Failed to launch replay UI: {e}"),
        }
    
        // Note that we're not calling `.await` here.
        let buffer_slice = staging_buffer.slice(..);
        // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
        let (
            sender, 
            receiver
        ) = flume::bounded(1);
        buffer_slice.map_async(
            wgpu::MapMode::Read, 
            move |v| sender.send(v).unwrap()
        );
    
        // Poll the device in a blocking manner so that our future resolves.
        // In an actual application, `device.poll(...)` should
        // be called in an event loop or on another thread.
        device.poll(wgpu::Maintain::Wait);
    
        // Awaits until `buffer_future` can be read from
        if let Ok(Ok(())) = receiver.recv_async().await {
            // Gets contents of buffer
            let data = buffer_slice.get_mapped_range();
            // Since contents are got in bytes, this converts these bytes back to u32
            let result = bytemuck::cast_slice(&data).to_vec();
    
            // With the current interface, we have to make sure all mapped views are
            // dropped before we unmap the buffer.
            drop(data);
            staging_buffer.unmap(); // Unmaps buffer from memory
                                    // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                    //   delete myPointer;
                                    //   myPointer = NULL;
                                    // It effectively frees the memory
    
            // Returns data from buffer
            (0..acc.len()).for_each(|i| acc[i] += result[i]);
            println!("iter: {} / {}", i, BIG);
        } else {
            panic!("failed to run compute on gpu!")
        }
    }

    Some(acc.iter().map(|x| x.clone() / BIG as f32).collect::<Vec<Vec3>>())
}
