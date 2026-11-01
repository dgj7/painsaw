use glcore::{GL_FILL, GL_FRONT_AND_BACK, GL_TEXTURE_2D};
use crate::geometry::orient::Orientation;
use crate::geometry::primitive::prim3d::Primitive3D;
use crate::graphics::camera::Camera;
use crate::graphics::subsystem::opengl::ffp::api::{gl_bind_texture, gl_color_4f, gl_disable, gl_enable, gl_end, gl_load_identity, gl_matrix_mode, gl_normal_3f, gl_polygon_mode, gl_pop_attrib, gl_pop_matrix, gl_push_attrib, gl_push_matrix, gl_rotate_f, gl_scale_f, gl_tex_coord_2f, gl_translate_f, gl_vertex_3f, glu_perspective};
use windows::Win32::Graphics::OpenGL::{
    GL_ALL_ATTRIB_BITS, GL_DEPTH_TEST, GL_MODELVIEW, GL_PROJECTION,
};
use crate::geometry::orient::matrix::m4x4::columnmajor;
use crate::graphics::storage::asset::Asset3D;
use crate::graphics::storage::g3d::Graph3D;
use crate::graphics::subsystem::opengl::ffp::ffpxd::{ffp_initialize_texture, ffp_update_texture};
use crate::graphics::subsystem::opengl::ffp::util::gl_begin_triangles;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

pub(crate) fn ffp_3d_setup(camera: &Camera) {
    /* save prior state before 3d rendering */
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    /* gather camera data */
    let position = columnmajor::position(&camera.orientation.transform);

    /* projection: reset matrix */
    gl_matrix_mode(GL_PROJECTION);
    gl_load_identity();

    /* adjust perspective (removes ortho) */
    glu_perspective(camera.projection.fov as f64, camera.projection.to_aspect() as f64, camera.projection.near as f64, camera.projection.far as f64, );

    /* storage/view: reset matrix; enable depth test; ready for 3d drawing */
    gl_matrix_mode(GL_MODELVIEW);
    gl_load_identity();
    gl_enable(GL_DEPTH_TEST);

    /* storage/view: adjust camera, before drawing */
    gl_rotate_f(-camera.orientation.pitch.degrees, 1.0, 0.0, 0.0);
    gl_rotate_f(-camera.orientation.yaw.degrees, 0.0, 1.0, 0.0);
    gl_translate_f(-position.x, -position.y, -position.z);
}

pub(crate) fn ffp_3d_teardown() {
    gl_disable(GL_DEPTH_TEST);

    gl_pop_attrib();
    gl_pop_matrix();
}

// todo: rename to render_primitive
pub(crate) fn ffp_3d_render(primitive: &Primitive3D, preparation: impl Fn(), begin: impl Fn()) {
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    ffp_3d_translate(&primitive.orientation);

    gl_color_4f(primitive.color.red, primitive.color.green, primitive.color.blue, primitive.color.alpha, );
    gl_polygon_mode(primitive.face.to_u32(), primitive.mode.to_u32());

    preparation();

    begin();
    for vert in &primitive.vertices {
        gl_vertex_3f(vert.x, vert.y, vert.z);
    }
    gl_end();

    gl_pop_attrib();
    gl_pop_matrix();
}

pub(crate) fn ffp_3d_render_asset(asset: &Asset3D) {
    if !asset.visible {
        return;
    }

    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    for mesh in asset.meshes.iter() {
        let material = mesh
            .material_index
            .and_then(|index| asset.materials.get(index));

        match material {
            Some(material) => {
                gl_color_4f(
                    material.diffuse_color.red,
                    material.diffuse_color.green,
                    material.diffuse_color.blue,
                    material.diffuse_color.alpha,
                );

                match &material.diffuse_texture {
                    Some(texture) if texture.initialized => {
                        gl_enable(GL_TEXTURE_2D);
                        gl_bind_texture(GL_TEXTURE_2D, texture.id);
                    }
                    _ => {
                        gl_disable(GL_TEXTURE_2D);
                    }
                }
            }
            None => {
                gl_disable(GL_TEXTURE_2D);
                gl_color_4f(1.0, 1.0, 1.0, 1.0);
            }
        }

        // todo: use PolygonFace here
        gl_polygon_mode(GL_FRONT_AND_BACK, GL_FILL);

        gl_begin_triangles();

        for index in mesh.indices.iter() {
            let Some(vertex) = mesh.vertices.get(*index as usize) else {
                continue;
            };

            if let Some(normal) = &vertex.normal {
                gl_normal_3f(normal.x, normal.y, normal.z);
            }

            if let Some(tex_coord) = &vertex.tex_coord {
                gl_tex_coord_2f(tex_coord.x, tex_coord.y);
            }

            gl_vertex_3f(
                vertex.position.x,
                vertex.position.y,
                vertex.position.z,
            );
        }

        gl_end();

        gl_disable(GL_TEXTURE_2D);
    }

    gl_pop_attrib();
    gl_pop_matrix();
}

fn ffp_3d_translate(orientation: &Orientation) {
    let position = columnmajor::position(&orientation.transform);
    gl_translate_f(position.x, position.y, position.z);
    gl_rotate_f(orientation.pitch.degrees, 1.0, 0.0, 0.0);
    gl_rotate_f(orientation.yaw.degrees, 0.0, 1.0, 0.0);
    gl_scale_f(orientation.x_scale, orientation.y_scale, orientation.z_scale, );
}

pub(crate) fn ffp_3d_initialize_textures(g3d: &mut Graph3D) {
    gl_enable(GL_TEXTURE_2D);

    for (_, value) in g3d.iter_mut() {
        for tex in &mut value.textures {
            ffp_initialize_texture(tex);
        }
    }

    for asset in g3d.assets.iter_mut() {
        for material in asset.materials.iter_mut() {
            if let Some(texture) = &mut material.diffuse_texture {
                ffp_initialize_texture(texture);
            }
        }
    }

    log(LogLevel::Debug, &|| String::from("initialization complete"));
}

pub(crate) fn ffp_3d_update_textures(g3d: &mut Graph3D) {
    for (_, model) in &mut g3d.iter_mut() {
        for texture in &mut model.textures {
            if !texture.initialized {
                ffp_initialize_texture(texture);
            }

            ffp_update_texture(texture);
        }
    }

    for asset in g3d.assets.iter_mut() {
        for material in asset.materials.iter_mut() {
            if let Some(texture) = &mut material.diffuse_texture {
                if !texture.initialized {
                    ffp_initialize_texture(texture);
                }

                ffp_update_texture(texture);
            }
        }
    }
}
