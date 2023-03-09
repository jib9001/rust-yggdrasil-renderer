use yggdrasil::graphics::gl_wrapper::*;
use yggdrasil::graphics::window::Window;

use gl::types::*;
use std::mem;
use std::ptr;

//use yggdrasil::custom_errors::Errors;

fn main() {
    //yggdrasil::logger::init();

    let mut window = Window::new(800, 600, "I'm aliiiive");

    let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    window.init_gl();

    let vao = yggdrasil::graphics::gl_wrapper::Vao::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.store_f32_data(&vertices);

    let position_attribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * (mem::size_of::<GLfloat>() as GLsizei),
        ptr::null()
    );

    position_attribute.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 24);
        }
        window.update();
    }

    // error_test(1);
}

// fn error_test(num: i32) -> Result<(), Errors> {
//     if num == 1 {
//         return Err(Errors::TestError.into());
//     }
//     Ok(())
// }