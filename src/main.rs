extern crate glium;

use glium::Surface;
use glium::Frame;

fn draw(target: &mut Frame) {
    target.clear_color(0.05, 0.06, 0.09, 1.0);
}

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    loop {
        let mut target = display.draw();
        draw(&mut target);
        target.finish().unwrap();
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,   // the window has been closed by the user
                _ => ()
            }
        }
    }
}