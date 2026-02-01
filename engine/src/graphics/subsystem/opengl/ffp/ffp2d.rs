//!
//! opengl fixed-function api wrapper.
//!

use std::ffi::c_void;
use num_traits::Float;
use windows::Win32::Graphics::OpenGL::{GL_ALL_ATTRIB_BITS, GL_BLEND, GL_MODELVIEW, GL_NEAREST, GL_ONE_MINUS_SRC_ALPHA, GL_PROJECTION, GL_REPLACE, GL_RGBA, GL_SRC_ALPHA, GL_TEXTURE_2D, GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_UNSIGNED_BYTE};
use crate::graphics::camera::Camera;
use crate::graphics::geometry::primitive::prim2d::Primitive2D;
use crate::graphics::image::t2d::Texture2D;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::subsystem::opengl::ffp::api::{gl_begin_lines, gl_begin_points, gl_begin_quads, gl_bind_texture, gl_blend_func, gl_color_3f, gl_disable, gl_enable, gl_end, gl_gen_textures, gl_line_width, gl_load_identity, gl_matrix_mode, gl_ortho, gl_point_size, gl_pop_attrib, gl_pop_matrix, gl_push_attrib, gl_push_matrix, gl_tex_coord_2f, gl_tex_env_f, gl_tex_image_2d, gl_tex_parameter_i, gl_tex_sub_image_2d, gl_vertex_2f};
use crate::logger::log;
use crate::logger::log_level::LogLevel;

pub(crate) fn ffp_2d_setup<F: Float>(camera: &Camera<F>) {
    /* save prior state before 2d rendering */
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    /* projection: reset matrix; setup ortho for 2d drawing */
    gl_matrix_mode(GL_PROJECTION);
    gl_load_identity();
    gl_ortho(0.0, camera.width.to_f64().unwrap(), camera.height.to_f64().unwrap(), 0.0, -99999.0, 99999.0);

    /* storage/view: reset matrix; ready for 2d drawing */
    gl_matrix_mode(GL_MODELVIEW);
    gl_load_identity();
}

pub(crate) fn ffp_2d_teardown() {
    gl_pop_attrib();
    gl_pop_matrix();
}

pub(crate) fn ffp_2d_initialize_textures<F: Float>(g2d: &mut Graph2D<F>) {
    /* initialize textures */
    gl_enable(GL_TEXTURE_2D);
    for (_, value) in g2d.iter_mut() {
        for tex in &mut value.textures {
            ffp_2d_initialize_texture(tex);
        }
    }

    /* done */
    log(LogLevel::Debug, &|| String::from("initialization complete"));
}

pub(crate) fn ffp_2d_update_textures<F: Float>(g2d: &mut Graph2D<F>) {
    for (_, model) in &mut g2d.iter_mut() {
        for texture in &mut model.textures {
            if !texture.initialized {
                ffp_2d_initialize_texture(texture);
            }
            ffp_2d_update_texture(texture);
        }
    }
}

fn ffp_2d_initialize_texture<F: Float>(texture: &mut Texture2D<F>) {
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
        0,                                              // level
        GL_RGBA as i32,                                 // internal format; the number of color components in the texture data
        texture.image.width as i32,                     // width
        texture.image.height as i32,                    // height
        0,                                              // border
        GL_RGBA,                                        // format: (or order) of pixel data (r,g,b,a)
        GL_UNSIGNED_BYTE,                               // r-type: the data type of the pixel data
        texture.image.data.as_ptr() as *const c_void,   // pixels
    );

    /* mark the texture as initialized (ready) */
    texture.initialized = true;

    /* done */
    log(LogLevel::Info, &|| String::from(format!("created texture, id=[{}]", texture.id)));
}

fn ffp_2d_update_texture<F: Float>(texture: &mut Texture2D<F>) {
    if texture.replacement.is_some() {
        let repl = texture.replacement.take().unwrap();
        gl_bind_texture(GL_TEXTURE_2D, texture.id);
        gl_tex_sub_image_2d(
            GL_TEXTURE_2D,                              // target
            0,                                          // level
            0,                                          // x-offset
            0,                                          // y-offset
            repl.width as i32,                          // width
            repl.height as i32,                         // height
            GL_RGBA,                                    // format: (order) of pixel data (r, g, b, a)
            GL_UNSIGNED_BYTE,                           // r-type: the dat type of the pixel data
            repl.data.as_ptr() as *const c_void,        // pixels
        );
        texture.image = repl;
    }
}

pub(crate) fn ffp_render_2d_points<F: Float>(primitive: &Primitive2D<F>, point_size: F) {
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    gl_color_3f(primitive.color.red, primitive.color.green, primitive.color.blue);
    gl_point_size(point_size.to_f32().unwrap());

    gl_begin_points();
    for point in primitive.vertices.iter() {
        gl_vertex_2f(point.x.to_f32().unwrap(), point.y.to_f32().unwrap());
    }
    gl_end();

    gl_pop_attrib();
    gl_pop_matrix();
}

pub(crate) fn ffp_render_2d_lines<F: Float>(primitive: &Primitive2D<F>, thickness: F) {
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    gl_color_3f(primitive.color.red, primitive.color.green, primitive.color.blue);
    gl_line_width(thickness.to_f32().unwrap());

    gl_begin_lines();
    for vertex in primitive.vertices.iter() {
        gl_vertex_2f(vertex.x.to_f32().unwrap(), vertex.y.to_f32().unwrap());
    }
    gl_end();

    gl_pop_attrib();
    gl_pop_matrix();
}

pub(crate) fn ffp_render_2d_texture<F: Float>(texture: &Texture2D<F>) {
    /* save prior state before making changes */
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    /* gather variables */
    let scale = texture.scale.to_f32().unwrap();
    let x = texture.x.to_f32().unwrap();
    let y = texture.y.to_f32().unwrap();
    let width = texture.image.width as f32;
    let height = texture.image.height as f32;

    /* prepare to render textures */
    gl_enable(GL_TEXTURE_2D);
    gl_bind_texture(GL_TEXTURE_2D, texture.id);
    gl_tex_env_f(GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_REPLACE as f32);

    /* enable transparency in textures */
    gl_enable(GL_BLEND);
    gl_blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

    /* start drawing texture; only text coords and vertexes until gl-end! */
    gl_begin_quads();

    /* top left */
    gl_tex_coord_2f(0.0, 0.0);
    gl_vertex_2f(x, y);

    /* bottom left */
    gl_tex_coord_2f(0.0, 1.0);
    gl_vertex_2f(x, y + height * scale);

    /* bottom right */
    gl_tex_coord_2f(1.0, 1.0);
    gl_vertex_2f(x + width * scale, y +  height * scale);

    /* top right */
    gl_tex_coord_2f(1.0, 0.0);
    gl_vertex_2f(x + width * scale, y);

    /* done */
    gl_end();

    /* turn off the stuff we enabled (specifically for texturing) */
    // todo: can these be removed?
    gl_disable(GL_TEXTURE_2D);
    gl_disable(GL_BLEND);

    /* restore prior state */
    gl_pop_attrib();
    gl_pop_matrix();
}
