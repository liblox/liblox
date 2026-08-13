// 3D object types

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
    scale: f32,
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
        Mat4::from_scale_rotation_translation(Vec3::splat(self.scale), self.rotation.normalize(), self.position)
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
    pub fn new(pos: Vec3, scale: f32, rot: Option<Quat>) -> Self {
        // assert!(scale > 0.0, "Cube3D created with scale 0 or less");
        let r: f32 = scale / 2.0;
        Self {
            mesh_vertices: [
                // x axis
                Vertex { position: pos + Vec3::splat(-r), uv: Vec2::X, normal: Vec3::NEG_X },
                Vertex { position: pos + Vec3::new(-r, r, -r), uv: Vec2::ONE, normal: Vec3::NEG_X },
                Vertex { position: pos + Vec3::new(-r, -r, r), uv: Vec2::ZERO, normal: Vec3::NEG_X },
                Vertex { position: pos + Vec3::new(-r, r, r), uv: Vec2::Y, normal: Vec3::NEG_X },
                Vertex { position: pos + Vec3::new(r, -r, -r), uv: Vec2::ZERO, normal: Vec3::X },
                Vertex { position: pos + Vec3::new(r, r, -r), uv: Vec2::Y, normal: Vec3::X },
                Vertex { position: pos + Vec3::new(r, -r, r), uv: Vec2::X, normal: Vec3::X },
                Vertex { position: pos + Vec3::splat(r), uv: Vec2::ONE, normal: Vec3::X },

                // y axis
                Vertex { position: pos + Vec3::new(r, -r, -r), uv: Vec2::ONE, normal: Vec3::NEG_Y },
                Vertex { position: pos + Vec3::splat(-r), uv: Vec2::X, normal: Vec3::Y },
                Vertex { position: pos + Vec3::new(-r, r, -r), uv: Vec2::Y, normal: Vec3::NEG_Y },
                Vertex { position: pos + Vec3::new(-r, -r, r), uv: Vec2::ZERO, normal: Vec3::Y },
                Vertex { position: pos + Vec3::new(-r, r, r), uv: Vec2::X, normal: Vec3::NEG_Y },
                Vertex { position: pos + Vec3::new(r, r, -r), uv: Vec2::ONE, normal: Vec3::Y },
                Vertex { position: pos + Vec3::new(r, -r, r), uv: Vec2::ZERO, normal: Vec3::NEG_Y },
                Vertex { position: pos + Vec3::splat(r), uv: Vec2::Y, normal: Vec3::Y },

                // z axis
                Vertex { position: pos + Vec3::splat(-r), uv: Vec2::Y, normal: Vec3::NEG_Z },
                Vertex { position: pos + Vec3::new(-r, r, -r), uv: Vec2::ONE, normal: Vec3::NEG_Z },
                Vertex { position: pos + Vec3::new(-r, -r, r), uv: Vec2::ONE, normal: Vec3::Z },
                Vertex { position: pos + Vec3::new(-r, r, r), uv: Vec2::Y, normal: Vec3::Z },
                Vertex { position: pos + Vec3::new(r, -r, -r), uv: Vec2::ZERO, normal: Vec3::NEG_Z },
                Vertex { position: pos + Vec3::new(r, r, -r), uv: Vec2::X, normal: Vec3::NEG_Z },
                Vertex { position: pos + Vec3::new(r, -r, r), uv: Vec2::X, normal: Vec3::Z },
                Vertex { position: pos + Vec3::splat(r), uv: Vec2::ZERO, normal: Vec3::Z },
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

impl std::fmt::Display for Cube3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cube3D at {}, rot {}, scale {}", self.position, self.rotation, self.scale)
    }
}

#[derive(Debug)]
pub enum RendererError {
    NoTransferBuffer
}

impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RendererError::NoTransferBuffer => {
                write!(f, "No transfer buffer available! Create one using Renderer::resize_or_create_txbuf.")
            }
        }
    }
}

impl std::error::Error for RendererError {}

pub struct Renderer {
    pub gpu: Device,
    pub txbuf: Option<TransferBuffer>
}

impl Renderer {
    pub fn new(gpu: Device, txbuf_len: Option<u32>) -> Result<Renderer, Box<dyn std::error::Error>> {
        let txb = gpu
            .create_transfer_buffer()
            .with_size(txbuf_len.unwrap_or(1_024u32))
            .with_usage(TransferBufferUsage::UPLOAD)
            .build()?;
        Ok(Self { gpu, txbuf: Some(txb) })
    }

    pub fn gpu(&self) -> &Device {
        &self.gpu
    }

    /// Resizes a transfer buffer to a specified size.
    /// **DESTRUCTIVE!**
    pub fn resize_or_create_txbuf(&mut self, target_size: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.txbuf = Some(self.gpu
           .create_transfer_buffer()
            .with_size(target_size)
            .with_usage(TransferBufferUsage::UPLOAD)
            .build()?);
        Ok(())
    }

    pub fn unload_txbuf(&mut self) {
        self.txbuf.take();
    }

    pub fn load_to_gpu<T: Copy>(&self, copy_pass: &CopyPass, usage: BufferUsageFlags, data: &[T], offset: usize) -> Result<Buffer, Box<dyn std::error::Error>> {
        let datalen = size_of_val(data);

        let buf = self.gpu
            .create_buffer()
            .with_size(datalen.try_into().unwrap())
            .with_usage(usage)
            .build()?;

        let txbuf = self.txbuf.as_ref().ok_or(RendererError::NoTransferBuffer)?;
        let mut map = txbuf.map::<T>(&self.gpu, true);
        let mem = map.mem_mut();
        for (i, &val) in data.iter().enumerate() {
            mem[offset + i] = val;
        }
        map.unmap();

        copy_pass.upload_to_gpu_buffer(
            TransferBufferLocation::new()
                .with_offset(offset as u32 * 4)
                .with_transfer_buffer(txbuf),
            BufferRegion::new()
                .with_offset(0)
                .with_buffer(&buf)
                .with_size(datalen as u32),
            true
        );

        Ok(buf)
    }
}