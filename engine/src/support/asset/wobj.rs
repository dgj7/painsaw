//!
//! parser for wavefront obj files.
//!
//! the expected format is https://en.wikipedia.org/wiki/Wavefront_.obj_file
//!
//! the typical file extension is .obj
//!

use std::collections::HashMap;
use std::io::{BufRead, Error, ErrorKind, Seek};
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::v3d::Vertex3D;
use crate::support::asset::{Asset3D, Asset3DLoader, Mesh3D, MeshVertex3D};

///
/// dummy struct for wavefront obj file parser.
///
pub struct WObj {}

///
/// implement the parser.
///
impl Asset3DLoader for WObj {
    fn load_from_buf_read<R: BufRead + Seek>(reader: R) -> std::io::Result<Asset3D> {
        let mut name = String::new();

        let mut temp_positions: Vec<[f32; 3]> = Vec::new();
        let mut temp_tex_coords: Vec<[f32; 2]> = Vec::new();
        let mut temp_normals: Vec<[f32; 3]> = Vec::new();

        let mut vertices: Vec<MeshVertex3D> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut vertex_to_index: HashMap<(usize, Option<usize>, Option<usize>), u32> = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let mut parts = trimmed.split_whitespace();
            let header = match parts.next() {
                Some(h) => h,
                None => continue,
            };

            match header {
                /* object name */
                "o" => {
                    name = parts.collect::<Vec<&str>>().join(" ");
                }
                /* positions */
                "v" => {
                    let x = parse_next_f32(&mut parts, "OBJ: missing vertex x coordinate")?;
                    let y = parse_next_f32(&mut parts, "OBJ: missing vertex y coordinate")?;
                    let z = parse_next_f32(&mut parts, "OBJ: missing vertex z coordinate")?;
                    temp_positions.push([x, y, z]);
                }
                /* tex coords */
                "vt" => {
                    let u = parse_next_f32(&mut parts, "OBJ: missing texture u coordinate")?;
                    let v = parse_next_f32(&mut parts, "OBJ: missing texture v coordinate")?;
                    temp_tex_coords.push([u, v]);
                }
                /* normals */
                "vn" => {
                    let x = parse_next_f32(&mut parts, "OBJ: missing normal x coordinate")?;
                    let y = parse_next_f32(&mut parts, "OBJ: missing normal y coordinate")?;
                    let z = parse_next_f32(&mut parts, "OBJ: missing normal z coordinate")?;
                    temp_normals.push([x, y, z]);
                }
                /* faces */
                "f" => {
                    let mut face_indices: Vec<u32> = Vec::new();

                    for vertex_str in parts {
                        let key = parse_obj_face_vertex(
                            vertex_str,
                            temp_positions.len(),
                            temp_tex_coords.len(),
                            temp_normals.len(),
                        )?;

                        let vertex_index = match vertex_to_index.get(&key) {
                            Some(&existing_index) => existing_index,
                            None => {
                                let position = temp_positions.get(key.0).ok_or_else(|| {
                                    Error::new(
                                        ErrorKind::InvalidData,
                                        format!("OBJ: position index out of range: {}", key.0 + 1),
                                    )
                                })?;

                                let tex_coord = match key.1 {
                                    Some(tex_idx) => {
                                        let tex_coord = temp_tex_coords.get(tex_idx).ok_or_else(|| {
                                            Error::new(
                                                ErrorKind::InvalidData,
                                                format!(
                                                    "OBJ: texture coordinate index out of range: {}",
                                                    tex_idx + 1
                                                ),
                                            )
                                        })?;

                                        Some(Vertex2D::new(tex_coord[0], tex_coord[1]))
                                    }
                                    None => None,
                                };

                                let normal = match key.2 {
                                    Some(norm_idx) => {
                                        let normal = temp_normals.get(norm_idx).ok_or_else(|| {
                                            Error::new(
                                                ErrorKind::InvalidData,
                                                format!(
                                                    "OBJ: normal index out of range: {}",
                                                    norm_idx + 1
                                                ),
                                            )
                                        })?;

                                        Some(Vertex3D::new(normal[0], normal[1], normal[2]))
                                    }
                                    None => None,
                                };

                                let new_index = vertices.len() as u32;

                                vertices.push(MeshVertex3D {
                                    position: Vertex3D::new(position[0], position[1], position[2]),
                                    normal,
                                    tex_coord,
                                });

                                vertex_to_index.insert(key, new_index);

                                new_index
                            }
                        };

                        face_indices.push(vertex_index);
                    }

                    if face_indices.len() < 3 {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "OBJ: face must contain at least 3 vertices",
                        ));
                    }

                    // Triangulate as a fan:
                    // quad 0,1,2,3 becomes 0,1,2 and 0,2,3.
                    for i in 1..face_indices.len() - 1 {
                        indices.push(face_indices[0]);
                        indices.push(face_indices[i]);
                        indices.push(face_indices[i + 1]);
                    }
                }
                _ => {
                    // mtllib, usemtl, s, g, l, p
                }
            }
        }

        Ok(Asset3D {
            id: 0,
            name,
            meshes: vec![Mesh3D {
                vertices,
                indices,
                material_index: None,
            }],
            materials: vec![],
        })
    }
}

fn parse_next_f32<'a, I>(parts: &mut I, error: &'static str) -> std::io::Result<f32>
where
    I: Iterator<Item = &'a str>,
{
    let value = parts
        .next()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, error))?;

    value
        .parse::<f32>()
        .map_err(|_| Error::new(ErrorKind::InvalidData, format!("OBJ: invalid float: [{}]", value)))
}

fn parse_obj_face_vertex(
    vertex_str: &str,
    position_count: usize,
    tex_coord_count: usize,
    normal_count: usize,
) -> std::io::Result<(usize, Option<usize>, Option<usize>)> {
    let mut indices = vertex_str.split('/');

    let pos_idx = parse_obj_index(
        indices.next(),
        position_count,
        "OBJ: missing face position index",
        "OBJ: invalid face position index",
    )?;

    let tex_idx = match indices.next() {
        Some("") | None => None,
        Some(value) => Some(parse_obj_index(
            Some(value),
            tex_coord_count,
            "OBJ: missing face texture coordinate index",
            "OBJ: invalid face texture coordinate index",
        )?),
    };

    let norm_idx = match indices.next() {
        Some("") | None => None,
        Some(value) => Some(parse_obj_index(
            Some(value),
            normal_count,
            "OBJ: missing face normal index",
            "OBJ: invalid face normal index",
        )?),
    };

    if indices.next().is_some() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("OBJ: invalid face vertex: {}", vertex_str),
        ));
    }

    Ok((pos_idx, tex_idx, norm_idx))
}

fn parse_obj_index(
    value: Option<&str>,
    item_count: usize,
    missing_error: &'static str,
    invalid_error: &'static str,
) -> std::io::Result<usize> {
    let value = value.ok_or_else(|| Error::new(ErrorKind::InvalidData, missing_error))?;

    let index = value
        .parse::<isize>()
        .map_err(|_| Error::new(ErrorKind::InvalidData, format!("{}: {}", invalid_error, value)))?;

    if index == 0 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "OBJ: indices must not be 0",
        ));
    }

    let resolved = if index > 0 {
        index - 1
    } else {
        item_count as isize + index
    };

    if resolved < 0 || resolved >= item_count as isize {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("OBJ: index out of range: {}", value),
        ));
    }

    Ok(resolved as usize)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::support::asset::Asset3DLoader;
    use super::WObj;

    const PYRAMID_OBJ: &str = include_str!("../../../tests/pyramid.obj");

    #[test]
    fn loads_external_pyramid_obj_fixture() {
        let asset = WObj::load_from_bytes(PYRAMID_OBJ.as_bytes())
            .expect("pyramid.obj should load successfully");

        assert_eq!(0, asset.id);
        assert_eq!("", asset.name);
        assert_eq!(1, asset.meshes.len());
        assert_eq!(0, asset.materials.len());

        let mesh = &asset.meshes[0];

        assert_eq!(18, mesh.indices.len());
        assert_eq!(16, mesh.vertices.len());
        assert_eq!(None, mesh.material_index);

        let first = &mesh.vertices[0];
        assert_eq!(-1.0, first.position.x);
        assert_eq!(0.0, first.position.y);
        assert_eq!(-1.0, first.position.z);

        let first_tex_coord = first.tex_coord.as_ref().expect("first vertex should have tex coord");
        assert_eq!(0.0, first_tex_coord.x);
        assert_eq!(0.0, first_tex_coord.y);

        let first_normal = first.normal.as_ref().expect("first vertex should have normal");
        assert_eq!(0.0, first_normal.x);
        assert_eq!(-1.0, first_normal.y);
        assert_eq!(0.0, first_normal.z);
    }

    #[test]
    fn loads_triangle_without_texture_coordinates_or_normals() {
        let obj = "\
            o Simple Triangle
            v 0.0 0.0 0.0
            v 1.0 0.0 0.0
            v 0.0 1.0 0.0
            f 1 2 3
            ";

        let asset = WObj::load_from_bytes(obj.as_bytes())
            .expect("simple triangle should load successfully");

        assert_eq!("Simple Triangle", asset.name);
        assert_eq!(1, asset.meshes.len());

        let mesh = &asset.meshes[0];
        assert_eq!(3, mesh.vertices.len());
        assert_eq!(vec![0, 1, 2], mesh.indices);

        assert!(mesh.vertices.iter().all(|vertex| vertex.tex_coord.is_none()));
        assert!(mesh.vertices.iter().all(|vertex| vertex.normal.is_none()));
    }

    #[test]
    fn triangulates_quad_faces() {
        let obj = "\
            v 0.0 0.0 0.0
            v 1.0 0.0 0.0
            v 1.0 1.0 0.0
            v 0.0 1.0 0.0
            f 1 2 3 4
            ";

        let asset = WObj::load_from_bytes(obj.as_bytes())
            .expect("quad should load successfully");

        let mesh = &asset.meshes[0];

        assert_eq!(4, mesh.vertices.len());
        assert_eq!(vec![0, 1, 2, 0, 2, 3], mesh.indices);
    }

    #[test]
    fn supports_relative_negative_indices() {
        let obj = "\
            v 0.0 0.0 0.0
            v 1.0 0.0 0.0
            v 0.0 1.0 0.0
            f -3 -2 -1
            ";

        let asset = WObj::load_from_bytes(obj.as_bytes())
            .expect("negative relative indices should load successfully");

        let mesh = &asset.meshes[0];

        assert_eq!(3, mesh.vertices.len());
        assert_eq!(vec![0, 1, 2], mesh.indices);
    }

    #[test]
    fn rejects_faces_with_less_than_three_vertices() {
        let obj = "\
            v 0.0 0.0 0.0
            v 1.0 0.0 0.0
            f 1 2
            ";

        match WObj::load_from_bytes(obj.as_bytes()) {
            Ok(_) => panic!("face with fewer than three vertices should fail"),
            Err(error) => {
                assert_eq!(std::io::ErrorKind::InvalidData, error.kind());
            }
        }
    }

    #[test]
    fn rejects_out_of_range_indices() {
        let obj = "\
            v 0.0 0.0 0.0
            v 1.0 0.0 0.0
            v 0.0 1.0 0.0
            f 1 2 4
            ";

        match WObj::load_from_bytes(obj.as_bytes()) {
            Ok(_) => panic!("out-of-range face index should fail"),
            Err(error) => {
                assert_eq!(std::io::ErrorKind::InvalidData, error.kind());
            }
        }
    }

    #[test]
    fn load_from_buf_read_accepts_seekable_buffer() {
        let mut cursor = Cursor::new(PYRAMID_OBJ.as_bytes());

        let asset = WObj::load_from_buf_read(&mut cursor)
            .expect("seekable buffered OBJ data should load successfully");

        assert_eq!(1, asset.meshes.len());
        assert_eq!(18, asset.meshes[0].indices.len());
    }
}
