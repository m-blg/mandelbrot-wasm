use glow::HasContext as _;
use winit::{
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, MouseScrollDelta, DeviceEvent},
    event_loop::EventLoop,
    window::WindowBuilder, dpi::PhysicalPosition, platform::web::WindowBuilderExtWebSys,
};

mod vector;
use vector::*;

fn main() {
    unsafe {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");

        let event_loop = EventLoop::new();

        let canvas = wasm::get_canvas_by_id("canvas");        
        log::warn!("test");
        let window = WindowBuilder::new()
            .with_canvas(canvas)
            .with_title("A fantastic window!")
            .build(&event_loop)
            .unwrap();


        // wasm::insert_canvas(&window);

        // Create a context from a WebGL2 context on wasm32 targets
        // #[cfg(target_arch = "wasm32")]
        let (gl, shader_version) = {
            use wasm_bindgen::JsCast;
            // let canvas = web_sys::window()
            //     .unwrap()
            //     .document()
            //     .unwrap()
            //     .get_element_by_id("canvas")
            //     .unwrap()
            //     .dyn_into::<web_sys::HtmlCanvasElement>()
            //     .unwrap();
            use winit::platform::web::WindowExtWebSys;
            let webgl2_context = window.canvas()
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();
            let gl = glow::Context::from_webgl2_context(webgl2_context);
            (gl, "#version 300 es")
        };
        let gl = std::sync::Arc::new(gl);

        // use egui_glow::
        let mut egui_glow = egui_glow::EguiGlow::new(&event_loop, gl.clone());
        // let egui_painter = egui_glow::painter::Painter::new(gl.clone(), "", Some(ShaderVersion::Es300));

            // let vertex1 = QuadVertex { position: [-1.0, -1.0], uv: [0.0, 0.0] };
            // let vertex2 = QuadVertex { position: [ 1.0,  -1.0], uv: [1.0, 0.0] };
            // let vertex3 = QuadVertex { position: [ 1.0, 1.0], uv: [1.0, 1.0] };
            // let vertex4 = QuadVertex { position: [ -1.0, 1.0], uv: [0.0, 1.0] };
            // let quad = vec![vertex1, vertex2, vertex3, vertex4];
            // let quad_indices = [0_u32, 1, 2, 0, 2, 3];
        

        let vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vao));

        // let vbo = gl.create_buffer().unwrap();
        // let ibo = gl.create_buffer().unwrap();

        // gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        // gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, usage)
        // gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

        let program = gl.create_program().expect("Cannot create program");

        let vertex_shader_source = include_str!("../assets/quad.vs.glsl");
        let fragment_shader_source = include_str!("../assets/quad.fs.glsl");

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{:x}: {}", shader_type, gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));

        // NOTE(mb): can be optimized out
        let u_trans = gl.get_uniform_location(program, "u_trans");
        let u_scale = gl.get_uniform_location(program, "u_scale");
        let u_color_coefs = gl.get_uniform_location(program, "u_color_coefs");
        let u_R = gl.get_uniform_location(program, "u_R");
        let u_MAX_ITER = gl.get_uniform_location(program, "u_MAX_ITER");

        let mut translation = vec2f::new(-0.55, 0.);
        let mut scale = vec2f::new(0.26,0.26);

        let mut color_coefs = [3., 20., 1., 0., 0., 0.];

        let mut MAX_ITER = 1000;
        let mut R = 10.0;


        // log::info!("window size: {:?}", window.inner_size());

        // let event_loop = EventLoop::new();

        // let window = WindowBuilder::new()
        //     .with_title("A fantastic window!")
        //     .build(&event_loop)
        //     .unwrap();

        // #[cfg(target_arch = "wasm32")]
        // let log_list = wasm::insert_canvas_and_create_log_list(&window);

                    // gl.clear_color(0.1, 0.2, 0.3, 1.0);
                    // gl.clear(glow::COLOR_BUFFER_BIT);
                    // gl.draw_arrays(glow::TRIANGLES, 0, 3);
                    // gl.delete_program(program);
                    // gl.delete_vertex_array(vertex_array);
        let mut t: f32 = 0.;

        let mut prev_frame_time = instant::Instant::now();

        let mut clear_color = [0.1, 0.1, 0.1];


        let mut mouse_pos = vec2f::new(0.,0.);
        event_loop.run(move |event, _, control_flow| {
            let frame_begin_time = instant::Instant::now();
            let dt_dur = frame_begin_time - prev_frame_time;
            let dt = dt_dur.as_secs() as f32 + dt_dur.subsec_nanos() as f32 / 1_000_000_000.0;

            // control_flow.set_wait_until(frame_begin_time + ::std::time::Duration::new(0, 1_000_000_000u32 / 60));
            control_flow.set_poll();

            // #[cfg(target_arch = "wasm32")]
            // wasm::log_event(&log_list, &event);
            match event {
                
                Event::DeviceEvent {event, ..} => {
                    match event {
                        // DeviceEvent::MouseMotion {delta} => {
                        //     mouse_pos = vec2f::new(delta.0 as f32, delta.1 as f32);
                        // },
                        _ => ()
                    }
                }

                Event::WindowEvent {
                    event,
                    window_id,
                } if window_id == window.id() => {
                    egui_glow.on_event(&event);
                    if event == WindowEvent::CloseRequested {
                        control_flow.set_exit();
                        gl.delete_program(program);
                        gl.delete_vertex_array(vao);
                    }

                    match event {
                        WindowEvent::KeyboardInput {input, ..} => {
                                        // state: ElementState::Pressed,
                                        // virtual_keycode: Some(VirtualKeyCode::Escape),
                            let KeyboardInput {state, virtual_keycode, ..} = input;
                            let speed = 0.1 / scale.x;
                            match virtual_keycode {
                                Some(VirtualKeyCode::W) => translation += vec2f::new(0.,speed),
                                Some(VirtualKeyCode::S) => translation += vec2f::new(0.,-speed),
                                Some(VirtualKeyCode::D) => translation += vec2f::new(speed,0.),
                                Some(VirtualKeyCode::A) => translation += vec2f::new(-speed,-0.),
                                Some(VirtualKeyCode::Space) => scale = 1.5*scale,
                                Some(VirtualKeyCode::X) => scale = (1.0/1.5)*scale,
                                _ => ()
                            }
                        },
                        WindowEvent::CursorMoved {position, ..} => {
                            mouse_pos.x = position.x as f32;
                            mouse_pos.y = position.y as f32;
                        }
                        WindowEvent::MouseWheel {delta, ..} => {
                            // log::warn!("{:?}", mouse_pos);
                            if let MouseScrollDelta::PixelDelta(p) = delta {
                                scale = ((p.y as f32)*0.001 + 1.)*scale;
                            }
                        },
                        _ => ()
                    }
                },
                Event::MainEventsCleared => {
                    // egui_glow.on_event(&event);
                    window.request_redraw();
                    t += 0.01;
                },
                Event::RedrawRequested(_) => {
                if dt_dur >= ::std::time::Duration::new(0, 1_000_000_000u32 / 60) {
                    prev_frame_time = frame_begin_time;

                    {
                        gl.clear_color(0.5*f32::sin(t)+0.5, clear_color[1], clear_color[2], 0.0);
                        gl.clear(glow::COLOR_BUFFER_BIT);
                        gl.use_program(Some(program));
                        gl.uniform_2_f32(u_trans.as_ref(), translation.x, translation.y);
                        gl.uniform_2_f32(u_scale.as_ref(), scale.x, scale.y);
                        gl.uniform_1_f32_slice(u_color_coefs.as_ref(), &color_coefs);
                        gl.uniform_1_f32(u_R.as_ref(), R);
                        gl.uniform_1_i32(u_MAX_ITER.as_ref(), MAX_ITER);
                        //gl.uniform_2_f32(u_mouse_pos.as_ref(), 
                            //mouse_pos.x/window.inner_size().width as f32, 
                            //mouse_pos.y/window.inner_size().height as f32);
                        // gl.bind_vertex_array(Some(vao));
                        gl.draw_arrays(glow::TRIANGLES, 0, 6);
                        // gl.bind_vertex_array(None);
                        egui_glow.run(&window, |egui_ctx| {
                            egui::Window::new("Settings")
                                .resizable(true)
                                .show(egui_ctx, |mut ui| 
                                {
                                    ui.label(format!("fps: {}", 1./dt));
                                    ui.horizontal(|mut ui| {
                                        ui.label("translation: ");
                                        gui_vec2(&mut ui, translation.as_mut(), -5_f32..=5_f32);
                                    });
                                    ui.horizontal(|mut ui| {
                                        ui.label("scale: ");
                                        gui_vec2(&mut ui, scale.as_mut(), -5_f32..=5_f32);
                                    });
                                    ui.label("coloring: ");
                                    ui.horizontal(|ui| {
                                        for c in color_coefs.iter_mut() {
                                            ui.add(egui::DragValue::new(c));
                                        }
                                    });

                                    ui.label("mandelbrot params: ");
                                    ui.horizontal(|ui| {
                                        ui.label("R: ");
                                        ui.add(egui::DragValue::new(&mut R));
                                        ui.label("max iter: ");
                                        ui.add(egui::DragValue::new(&mut MAX_ITER));
                                    });

                                    ui.add_space(30.);

                                    ui.label(
r"Controls: WASD + space + x, mouse wheel.
Mandelbrot set: let f(c, z) = z^2 + c; for each complex point p, 
if sequence f(p, 0), f(p, f(p,0)), f(p, f(p, f(p,0))), ... 
converges (in our case stays whithin circle with radius R certain number of iterations (max iter)), 
then its color - black, otherwise it has random color, 
depending on the number of iteration it stays in the circle");

                                });

                                // egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {

                                //     ui.heading("Hello World!");
                                //     // ui.color_edit_button_rgb(&mut clear_color);
                                // });
                            });
                    };


                    egui_glow.paint(&window);
                }
                }
                _ => (),
            }
        });
    }
}

use egui::Ui;
use std::ops::RangeInclusive;
fn gui_slider_vec2(ui: &mut Ui, v: &mut [f32;2], range: RangeInclusive<f32>)  {
    ui.add(egui::Slider::new(&mut v[0], range.clone()));
    ui.add(egui::Slider::new(&mut v[1], range.clone()));
}
fn gui_vec2(ui: &mut Ui, v: &mut [f32;2], range: RangeInclusive<f32>)  {
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut v[0]));
        ui.add(egui::DragValue::new(&mut v[1]));
    });
}

// #![allow(clippy::single_match)]


// pub fn main2() {
//     let event_loop = EventLoop::new();

//     let window = WindowBuilder::new()
//         .with_title("A fantastic window!")
//         .build(&event_loop)
//         .unwrap();

//     #[cfg(target_arch = "wasm32")]
//     let log_list = wasm::insert_canvas_and_create_log_list(&window);

//     event_loop.run(move |event, _, control_flow| {
//         control_flow.set_wait();

//         #[cfg(target_arch = "wasm32")]
//         wasm::log_event(&log_list, &event);

//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 window_id,
//             } if window_id == window.id() => control_flow.set_exit(),
//             Event::MainEventsCleared => {
//                 window.request_redraw();
//             }
//             _ => (),
//         }
//     });
// }

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::HtmlCanvasElement;
    use winit::{event::Event, window::Window};

    // #[wasm_bindgen(start)]
    // pub fn run() {
    //     console_log::init_with_level(log::Level::Debug).expect("error initializing logger");

    //     #[allow(clippy::main_recursion)]
    //     super::main();
    // }

    pub fn get_canvas_by_id(id: &str) -> Option<HtmlCanvasElement> {
        use winit::platform::web::WindowExtWebSys;
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(id).unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        Some(canvas)
    }

    pub fn insert_canvas(window: &Window) {
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        // Set a background color for the canvas to make it easier to tell where the canvas is for debugging purposes.
        canvas.style().set_css_text("background-color: crimson;");
        body.append_child(&canvas).unwrap();
    }

    pub fn insert_canvas_and_create_log_list(window: &Window) -> web_sys::Element {
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        // Set a background color for the canvas to make it easier to tell where the canvas is for debugging purposes.
        canvas.style().set_css_text("background-color: crimson;");
        body.append_child(&canvas).unwrap();

        let log_header = document.create_element("h2").unwrap();
        log_header.set_text_content(Some("Event Log"));
        body.append_child(&log_header).unwrap();

        let log_list = document.create_element("ul").unwrap();
        body.append_child(&log_list).unwrap();
        log_list
    }

    pub fn log_event(log_list: &web_sys::Element, event: &Event<()>) {
        log::debug!("{:?}", event);

        // Getting access to browser logs requires a lot of setup on mobile devices.
        // So we implement this basic logging system into the page to give developers an easy alternative.
        // As a bonus its also kind of handy on desktop.
        if let Event::WindowEvent { event, .. } = &event {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let log = document.create_element("li").unwrap();
            log.set_text_content(Some(&format!("{:?}", event)));
            log_list
                .insert_before(&log, log_list.first_child().as_ref())
                .unwrap();
        }
    }
}
