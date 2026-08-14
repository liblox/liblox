//! 3D meshes

use glam::*;
use sdl3::gpu::{Buffer, BufferRegion, BufferUsageFlags, CopyPass, Device, TransferBuffer, TransferBufferLocation, TransferBufferUsage};

#[derive(Clone, Copy)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec2
}

impl Vertex {
    /// Converts a vertex to a slice.
    /// 
    /// The order is:
    /// 1. Position (12 bytes; vec3)
    /// 2. Normal (12 bytes; vec3)
    /// 3. UV (8 bytes; vec2)
    /// As 8 f32s.
    pub fn to_slice(&self) -> [f32; 8] {
        [self.position.x, self.position.y, self.position.z, self.normal.x, self.normal.y, self.normal.z, self.uv.x, self.uv.y]
    }
}

pub trait Object3D {
    fn vertices(&self) -> Vec<Vertex>;
    fn indices(&self) -> Vec<u16>;
    fn translate(&mut self, translation: Vec3) -> ();
    fn rotate(&mut self, euler_rotation: Vec3) -> ();
    fn srt_matrix(&self) -> Mat4;
    fn get_verts_as_buf(&self) -> Vec<f32>;
}

pub trait Hexahedron3D: Object3D {
    fn from_verts(verts: Vec<Vertex>) -> Self;
}

#[derive(Clone, Copy)]
pub struct Cube3D {
    mesh_vertices: [Vertex; 24],
    mesh_indices: [u16; 36],
    scale: Vec3,
    position: Vec3,
    rotation: Quat
}

impl Object3D for Cube3D {
    fn vertices(&self) -> Vec<Vertex> {
        self.mesh_vertices.to_vec()
    }

    fn indices(&self) -> Vec<u16> {
        self.mesh_indices.to_vec()
    }

    fn srt_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    fn translate(&mut self, translation: Vec3) -> () {
        self.position += translation;
    }
    
    /// Rotates the mesh using Euler rotation.
    /// In degrees because radians suck.
    fn rotate(&mut self, euler_rotation: Vec3) -> () {
        self.rotation *= Quat::from_euler(
            EulerRot::XYZ,
            euler_rotation.x.to_radians(),
            euler_rotation.y.to_radians(),
            euler_rotation.z.to_radians()
        );
    }
    
    fn get_verts_as_buf(&self) -> Vec<f32> {
        self.vertices().iter().flat_map(|v| {
            v.position.to_array()
                .into_iter()
                .chain(v.normal.to_array())
                .chain(v.uv.to_array())
                .collect::<Vec<f32>>()
        }).collect()
    }
}

impl Hexahedron3D for Cube3D {
    fn from_verts(verts: Vec<Vertex>) -> Self {
        todo!()
    }
}

impl Cube3D {
    pub fn new(pos: Vec3, scale: Vec3, rot: Option<Quat>) -> Self {
        // assert!(scale > 0.0, "Cube3D created with scale 0 or less");
        let r: f32 = 0.5;
        Self {
            mesh_vertices: [
                // x axis
                Vertex { position: Vec3::splat(-0.5), uv: Vec2::X, normal: Vec3::NEG_X },
                Vertex { position: Vec3::new(-0.5,  0.5, -0.5), uv: Vec2::ONE, normal: Vec3::NEG_X },
                Vertex { position: Vec3::new(-0.5, -0.5,  0.5), uv: Vec2::ZERO, normal: Vec3::NEG_X },
                Vertex { position: Vec3::new(-0.5,  0.5,  0.5), uv: Vec2::Y, normal: Vec3::NEG_X },
                Vertex { position: Vec3::new( 0.5, -0.5, -0.5), uv: Vec2::ZERO, normal: Vec3::X },
                Vertex { position: Vec3::new( 0.5,  0.5, -0.5), uv: Vec2::Y, normal: Vec3::X },
                Vertex { position: Vec3::new( 0.5, -0.5,  0.5), uv: Vec2::X, normal: Vec3::X },
                Vertex { position: Vec3::splat(0.5), uv: Vec2::ONE, normal: Vec3::X },

                // y axis
                Vertex { position: Vec3::new( 0.5, -0.5, -0.5), uv: Vec2::ONE, normal: Vec3::NEG_Y },
                Vertex { position: Vec3::splat(-0.5), uv: Vec2::X, normal: Vec3::Y },
                Vertex { position: Vec3::new(-0.5,  0.5, -0.5), uv: Vec2::Y, normal: Vec3::NEG_Y },
                Vertex { position: Vec3::new(-0.5, -0.5,  0.5), uv: Vec2::ZERO, normal: Vec3::Y },
                Vertex { position: Vec3::new(-0.5,  0.5,  0.5), uv: Vec2::X, normal: Vec3::NEG_Y },
                Vertex { position: Vec3::new( 0.5,  0.5, -0.5), uv: Vec2::ONE, normal: Vec3::Y },
                Vertex { position: Vec3::new( 0.5, -0.5,  0.5), uv: Vec2::ZERO, normal: Vec3::NEG_Y },
                Vertex { position: Vec3::splat(0.5), uv: Vec2::Y, normal: Vec3::Y },

                // z axis
                Vertex { position: Vec3::splat(-0.5), uv: Vec2::Y, normal: Vec3::NEG_Z },
                Vertex { position: Vec3::new(-0.5,  0.5, -0.5), uv: Vec2::ONE, normal: Vec3::NEG_Z },
                Vertex { position: Vec3::new(-0.5, -0.5,  0.5), uv: Vec2::ONE, normal: Vec3::Z },
                Vertex { position: Vec3::new(-0.5,  0.5,  0.5), uv: Vec2::Y, normal: Vec3::Z },
                Vertex { position: Vec3::new( 0.5, -0.5, -0.5), uv: Vec2::ZERO, normal: Vec3::NEG_Z },
                Vertex { position: Vec3::new( 0.5,  0.5, -0.5), uv: Vec2::X, normal: Vec3::NEG_Z },
                Vertex { position: Vec3::new( 0.5, -0.5,  0.5), uv: Vec2::X, normal: Vec3::Z },
                Vertex { position: Vec3::splat(0.5), uv: Vec2::ZERO, normal: Vec3::Z },
            ],
            mesh_indices: [
                // bottom OK
                8,10,12,
                10,14,12,

                // top OK
                9,13,15,
                9,15,11,

                // front OK
                16,20,17,
                17,20,21,

                // back
                18,19,23,
                18,22,23,

                // left OK
                0,1,3,
                0,3,2,

                // right OK
                4,6,5,
                5,6,7,
            ],
            position: pos,
            scale,
            rotation: rot.unwrap_or(Quat::IDENTITY)
        }
    }
}

pub struct Plane3D {
    mesh_vertices: [Vertex; 4],
    mesh_indices: [u16; 6],
    scale: Vec3,
    position: Vec3,
    rotation: Quat
}

impl Object3D for Plane3D {
    fn vertices(&self) -> Vec<Vertex> {
        self.mesh_vertices.to_vec()
    }

    fn indices(&self) -> Vec<u16> {
        self.mesh_indices.to_vec()
    }

    fn translate(&mut self, translation: Vec3) -> () {
        self.position += translation
    }

    fn rotate(&mut self, euler_rotation: Vec3) -> () {
        self.rotation *= Quat::from_euler(EulerRot::XYZ, euler_rotation.x, euler_rotation.y, euler_rotation.z)
    }

    fn srt_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    fn get_verts_as_buf(&self) -> Vec<f32> {
        self.vertices().iter().flat_map(|v| {
            v.position.to_array()
                .into_iter()
                .chain(v.normal.to_array())
                .chain(v.uv.to_array())
                .collect::<Vec<f32>>()
        }).collect()
    }
}

impl Plane3D {
    pub fn new(pos: Vec3, scale: Vec3, rot: Option<Quat>) -> Self {
        Self {
            mesh_vertices: [
                Vertex { position: Vec3::new(-1.0, 0.0, 1.0), normal: Vec3::Y, uv: Vec2::Y },
                Vertex { position: Vec3::NEG_X.xyx(), normal: Vec3::Y, uv: Vec2::ZERO },
                Vertex { position: Vec3::X.xyx(), normal: Vec3::Y, uv: Vec2::ONE },
                Vertex { position: Vec3::new(1.0, 0.0, -1.0), normal: Vec3::Y, uv: Vec2::X }
            ],
            mesh_indices: [
                0,1,3,
                0,3,2
            ],
            scale,
            position: pos,
            rotation: rot.unwrap_or(Quat::IDENTITY)
        }
    }
}

impl std::fmt::Display for Cube3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cube3D at {}, rot {}, scale {}", self.position, self.rotation, self.scale)
    }
}