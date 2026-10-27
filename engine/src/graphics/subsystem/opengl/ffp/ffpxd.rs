use std::ffi::c_void;
use windows::Win32::Graphics::OpenGL::{GL_NEAREST, GL_RGBA, GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_UNSIGNED_BYTE};
use crate::graphics::subsystem::opengl::ffp::api::{gl_bind_texture, gl_gen_textures, gl_tex_image_2d, gl_tex_parameter_i, gl_tex_sub_image_2d};
use crate::graphics::texture::Texture;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

///
/// initialize a new texture.
///
/// note that glTexImage2D() creates a new texture.
///
pub(crate) fn ffp_initialize_texture(texture: &mut Texture) {
    /* gen 1 texture; bind it */
    let texture_id_ptr: *mut u32 = (&mut texture.id) as *mut u32;
    gl_gen_textures(1, texture_id_ptr);
    gl_bind_texture(GL_TEXTURE_2D, texture.id);

    /* set texture params */
    gl_tex_parameter_i(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as i32);
    gl_tex_parameter_i(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as i32);

    /* inform opengl of the texture data; see https://registry.khronos.org/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml */
    gl_tex_image_2d(
        GL_TEXTURE_2D,                                  // target
        0,                                         // level
        GL_RGBA as i32,                                 // internal format; the number of color components in the texture data
        texture.image.width as i32,                     // width
        texture.image.height as i32,                    // height
        0,                                       // border
        GL_RGBA,                                        // format: (or order) of pixel data (r,g,b,a)
        GL_UNSIGNED_BYTE,                               // r-type: the data type of the pixel data
        texture.image.data.as_ptr() as *const c_void,   // pixels
    );

    /* mark the texture as initialized (ready) */
    texture.initialized = true;

    /* done */
    log(LogLevel::Info, &|| {
        String::from(format!("created texture, id=[{}]", texture.id))
    });
}

///
/// update an existing texture.
///
/// note that glTexSubImage2D() works on an existing texture.
///
pub(crate) fn ffp_update_texture(texture: &mut Texture) {
    if texture.replacement.is_some() {
        let repl = texture.replacement.take().unwrap();
        gl_bind_texture(GL_TEXTURE_2D, texture.id);
        gl_tex_sub_image_2d(
            GL_TEXTURE_2D,                       // target
            0,                             // level
            0,                            // x-offset
            0,                            // y-offset
            repl.width as i32,                   // width
            repl.height as i32,                  // height
            GL_RGBA,                             // format: (order) of pixel data (r, g, b, a)
            GL_UNSIGNED_BYTE,                    // r-type: the dat type of the pixel data
            repl.data.as_ptr() as *const c_void, // pixels
        );
        texture.image = repl;
    }
}
