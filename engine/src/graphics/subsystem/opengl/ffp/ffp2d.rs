//!
//! opengl fixed-function api wrapper.
//!

use crate::geometry::primitive::prim2d::Primitive2D;
use crate::geometry::primitive::PrimitiveType;
use crate::graphics::camera::Camera;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::ui::UIManager;
use crate::graphics::subsystem::opengl::ffp::api::{gl_bind_texture, gl_blend_func, gl_color_4f, gl_disable, gl_enable, gl_end, gl_hint, gl_line_width, gl_load_identity, gl_matrix_mode, gl_ortho, gl_point_size, gl_polygon_mode, gl_pop_attrib, gl_pop_matrix, gl_push_attrib, gl_push_matrix, gl_tex_coord_2f, gl_tex_env_f, gl_vertex_2f};
use crate::graphics::subsystem::opengl::ffp::util::{gl_begin_line_strip, gl_begin_lines, gl_begin_points, gl_begin_quads};
use crate::graphics::texture::t2d::Texture;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use glcore::{GL_DEPTH_TEST, GL_LINE_SMOOTH, GL_LINE_SMOOTH_HINT, GL_NICEST};
use windows::Win32::Graphics::OpenGL::{GL_ALL_ATTRIB_BITS, GL_BLEND, GL_LIGHTING, GL_MODELVIEW, GL_ONE_MINUS_SRC_ALPHA, GL_PROJECTION, GL_REPLACE, GL_SRC_ALPHA, GL_TEXTURE_2D, GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE};
use crate::graphics::subsystem::opengl::ffp::ffpxd::{ffp_initialize_texture, ffp_update_texture};

pub(crate) fn ffp_2d_setup(camera: &Camera) {
    /* save prior state before 2d rendering */
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    /* projection: reset matrix; setup ortho for 2d drawing */
    gl_matrix_mode(GL_PROJECTION);
    gl_load_identity();
    gl_ortho(0.0, camera.projection.width as f64, camera.projection.height as f64, 0.0, -99999.0, 99999.0, );

    /* storage/view: reset matrix; ready for 2d drawing */
    gl_matrix_mode(GL_MODELVIEW);
    gl_load_identity();
}

pub(crate) fn ffp_2d_teardown() {
    /* reset changes */
    gl_pop_attrib();
    gl_pop_matrix();
}

pub(crate) fn ffp_2d_render_primitive(primitive: &Primitive2D) {
    match primitive.p_type {
        PrimitiveType::Point { point_size } => { ffp_2d_render(primitive, || gl_point_size(point_size), gl_begin_points) }
        PrimitiveType::Line { thickness } => { ffp_2d_render(primitive, || gl_line_width(thickness), gl_begin_lines) }
        PrimitiveType::Cube { thickness } => { ffp_2d_render(primitive, || gl_line_width(thickness), gl_begin_quads) }
        PrimitiveType::LineStrip { thickness } => { ffp_2d_render(primitive, || gl_line_width(thickness), gl_begin_line_strip) }
    }
}

pub(crate) fn ffp_2d_render(primitive: &Primitive2D, preparation: impl Fn(), begin: impl Fn()) {
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    gl_disable(GL_LIGHTING);
    gl_disable(GL_TEXTURE_2D);
    gl_disable(GL_DEPTH_TEST);

    gl_enable(GL_BLEND);
    gl_blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

    gl_enable(GL_LINE_SMOOTH);
    gl_hint(GL_LINE_SMOOTH_HINT, GL_NICEST);

    gl_color_4f(primitive.color.red, primitive.color.green, primitive.color.blue, primitive.color.alpha);
    gl_polygon_mode(primitive.face.to_u32(), primitive.mode.to_u32());

    preparation();

    begin();
    for vertex in primitive.vertices.iter() {
        gl_vertex_2f(vertex.x, vertex.y);
    }
    gl_end();

    gl_disable(GL_BLEND);

    gl_pop_attrib();
    gl_pop_matrix();
}

pub(crate) fn ffp_2d_initialize_textures(g2d: &mut Graph2D, ui: &mut UIManager) {
    /* enable texturing in opengl */
    gl_enable(GL_TEXTURE_2D);

    /* initialize 2d model textures */
    for (_, value) in g2d.iter_mut() {
        for tex in &mut value.textures {
            ffp_initialize_texture(tex);
        }
    }

    /* initialize ui textures */
    if let Some(view) = ui.check() {
        view.model.textures.iter_mut().for_each(|texture| { ffp_initialize_texture(texture)});
    }

    /* done */
    log(LogLevel::Debug, &|| String::from("initialization complete"));
}

pub(crate) fn ffp_2d_update_textures(g2d: &mut Graph2D, ui: &mut UIManager) {
    for (_, model) in &mut g2d.iter_mut() {
        for texture in &mut model.textures {
            if !texture.initialized {
                ffp_initialize_texture(texture);
            }
            ffp_update_texture(texture);
        }
    }

    if let Some(view) = ui.check() {
        view.model
            .textures
            .iter_mut().for_each(|texture| {
                if !texture.initialized {
                    ffp_initialize_texture(texture);
                }
                ffp_update_texture(texture)
            }
        );
    }
}

pub(crate) fn ffp_2d_render_texture(texture: &Texture) {
    /* save prior state before making changes */
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    /* gather variables */
    let scale = texture.scale;
    let x = texture.location.x;
    let y = texture.location.y;
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
    gl_vertex_2f(x + width * scale, y + height * scale);

    /* top right */
    gl_tex_coord_2f(1.0, 0.0);
    gl_vertex_2f(x + width * scale, y);

    /* done */
    gl_end();

    /* turn off the stuff we enabled (specifically for texturing) */
    gl_disable(GL_TEXTURE_2D);
    gl_disable(GL_BLEND);

    /* restore prior state */
    gl_pop_attrib();
    gl_pop_matrix();
}
