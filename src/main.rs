extern crate glfw;
extern crate gl;

use glfw::Context;
use std::cell::Cell;

/// evaluate the expression, then check for GL error.
macro_rules! glcheck {
    ($e: expr) => (
        {
            unsafe{$e};
            assert_eq!(unsafe {gl::GetError()}, 0);
        }
    )
}

fn error_callback(_: glfw::Error, description: String, error_count: &Cell<usize>) {
    println!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}

fn init_gl() {
    glcheck!(gl::ClearColor(0.3, 0.3, 0.3, 1.0) )  ;

    glcheck!(gl::FrontFace(gl::CCW));
    glcheck!(gl::Enable(gl::DEPTH_TEST));
    glcheck!(gl::Enable(gl::SCISSOR_TEST));
    glcheck!(gl::DepthFunc(gl::LEQUAL));
    glcheck!(gl::FrontFace(gl::CCW));
    glcheck!(gl::Enable(gl::CULL_FACE));
    glcheck!(gl::CullFace(gl::BACK));
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.set_error_callback(Some(
            glfw::Callback {
                f: error_callback as fn(glfw::Error, String, &Cell<usize>),
                data: Cell::new(0)
            }
    ));

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));

    let (mut window, _) = glfw.create_window(1100, 800, "Kreativity UI", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window!");

    window.set_key_polling(true);
    window.make_current();

    // use glfw to load GL function pointers
    glcheck!(gl::load_with(|name| window.get_proc_address(name) as *const _));
    init_gl();

    glfw.set_swap_interval(0);

    while !window.should_close() {
        glcheck!(gl::Clear(gl::COLOR_BUFFER_BIT));

        window.swap_buffers();
        glfw.poll_events();
    }

    println!("Hello, world!");
}
