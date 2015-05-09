#![feature(convert)]
#![feature(collections)]

extern crate glfw;
extern crate gl;
extern crate image;
extern crate cgmath;
extern crate libc;

mod macros;
mod glutil;
mod tile;
mod camera;
mod gen;
mod world;

use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;
use glfw::{Context, Key, Action};
use image::RgbImage;
use cgmath::Vector2;

use world::World;
use tile::Chunk;
use camera::Camera;

fn main() {
    println!("Initing GLFW");
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).ok().expect("Failed to init glfw.");
    
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    println!("Creating window.");
    let (mut window, events) = glfw.create_window(
        1280, 960, "Capital Larceny", glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window.");
    
    window.set_key_polling(true);
    window.make_current();
    
    // Load the external functions. From the gl-rs crate.
    gl::load_with(|s| window.get_proc_address(s));
    
    unsafe {
        // Basic OpenGL configs.
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::DepthFunc(gl::LEQUAL);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
    }
    
    println!("Creating camera");
    let mut camera = Camera::new(1280, 960, 50.0);
    
    println!("Loading tile program");
    let tile_program = tile::Program::new();
    
    let mut world = World::new(&tile_program);
    
    /*println!("Loading test image");
    let image_buf: RgbImage = image::open(&Path::new("assets/maps/test-map.png")).unwrap().to_rgb();
    let (img_w, img_h) = image_buf.dimensions();
    println!("Image w: {}, image h: {}", img_w, img_h);
    let chunk = Chunk::from_image_buffer(&tile_program, 0, 0, &image_buf, 0, 0);*/
    //let chunk = Chunk::blank(&tile_program, 0, 0);
    //world.chunks[0].push(Rc::new(RefCell::new(chunk)));
    
    println!("Starting main loop");
    while !window.should_close() {
        let (width, height) = window.get_size();
        camera.resize(width as u16, height as u16);
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        for chunks in world.chunks.iter() {
            for chunk in chunks.iter() {
                chunk.borrow().draw(&tile_program, &camera);
            }
        }
        
        window.swap_buffers();
        
        glfw.poll_events();
        glfw::flush_messages(&events);
        
        // Pan camera with W and S.
        if window.get_key(Key::W) == Action::Press {
            let zoom = camera.zoom();
            camera.translate(Vector2::new(0.0, 20.0 / zoom));
        }
        if window.get_key(Key::S) == Action::Press {
            let zoom = camera.zoom();
            camera.translate(Vector2::new(0.0, -20.0 / zoom));
        }
        
        // Pan camera with A and D.
        if window.get_key(Key::A) == Action::Press {
            let zoom = camera.zoom();
            camera.translate(Vector2::new(20.0 / zoom, 0.0));
        }
        if window.get_key(Key::D) == Action::Press {
            let zoom = camera.zoom();
            camera.translate(Vector2::new(-20.0 / zoom, 0.0));
        }
        
        // Zoom camera with Z and X.
        if window.get_key(Key::Z) == Action::Press {
            camera.zoom_by(1.05);
        }
        if window.get_key(Key::X) == Action::Press {
            camera.zoom_by(0.9523809524);
        }
    }
}
