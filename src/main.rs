use const_format::formatcp;
use glam::{Quat, Vec2, Vec3, Vec3Swizzles};
use liblox::three_d::objects::mesh::{Cube3D, Object3D, Plane3D, Vertex};
use liblox::three_d::objects::render::{Renderer};
use rand::random_range;
use sdl3::{
    event::Event, gpu::{BufferBinding, BufferUsageFlags, ColorTargetDescription, ColorTargetInfo, CompareOp, CullMode, DepthStencilState, DepthStencilTargetInfo, Device, Fence, FillMode, FrontFace, GraphicsPipelineTargetInfo, IndexElementSize, LoadOp, PrimitiveType, RasterizerState, SampleCount, ShaderFormat, ShaderStage, StoreOp, TextureCreateInfo, TextureFormat, TextureType, TextureUsage, VertexAttribute, VertexBufferDescription, VertexElementFormat, VertexInputRate, VertexInputState}, keyboard::Keycode, messagebox::{self, ButtonData, MessageBoxButtonFlag, MessageBoxFlag}, pixels::Color, sys::gpu::SDL_GPUShaderFormat
};

const RO_TOO_BIG: &str = "Rendered objects' size exceeded 4GiB";
const USIZEMAX_CONST_FMT: &str = if cfg!(target_pointer_width = "64") { "18,446,744,073,709,551,615" } else { if cfg!(target_pointer_width = "32") { "4,294,967,295" } else { formatcp!("{}", usize::MAX) } };
const TOO_MANY_OBJ: &str = const_format::formatcp!("You have {} or more objects to be rendered.

▓▓▓▓▒▒▒▒▒▒▒▒▒▓▓▓▓▓▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▓▓▓▓▒▓▓▓▓▓▓▒▒▒▒▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▓▓▓▓▓▓▒▒▓▒▓▓▓▓▓▓▓
▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▓▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▒▓▒▒▓▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▓▓▓▓▒▒▒▒▓▓▓▓▒▒▒▒▓▒▒▒▒▒▓▓▒▒▒▒▒▒▒▒▒▓▓▓▒▒▒▒▒▒▒▒▒▓▓▒▒▒▒▒▒▒▒▒▓▓▓▒▒▒▒▒▒▒▒▓▓▓▓▒▓▓▓▒▒▒▒▒▒▓▒▓▓▒▓▒▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▓▓▓▓▒▒░▒▒▓▓▒▒▒░▒▓▒░░▒▒▓▒░░▒▓▓▓▒░▒▒▓▓▓▓▓▓▓▒░░░▒▓▒░▒░▒▓▓▓▓▓▒▒░░▒▓▓▒▒▒▒▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░▒▒▒▒
▓▓▓▓░░░░░░░░░░▒▓▓▒░░░▒▓░░░░░░░░░░░▓▒▒░░░░░░░░▒▓▒░░░░░░░▒▓▒░░░░░░░░░░▒▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░▒▒▒▒▒▒▒▒▒▒▒░░░░▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░
▓▓▓▓░░░░▒▓▓▓▓▓▓▓▓▒░░░▒▓░░░░▒▓▓▒▒▓▓▒░░░▒▓▓▒░░░▒▓▓▒▓▓▓▒░░░▒▒░░░▒▓▓▓▒▓▓▓▒░░░░▒▒▒▒▒▒▒▒▒▒░░░░░▒▒▒▒▒▒▒░░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▒▒▒░░░░░░    ░░░░░░▒▒▒▓▓▓▓▓▓▓▒▓▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░ 
▓▓▓▓░░░░▓▓▓▓▓▓▓▓▓▒░░░▒▓▓▒░░░░░░░░▓▓░░░░░░░░░░▒▒░░░░░░░░░▓▓▒░░░░░░░░▒▓▒░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒░░░░          ░░░░░▒▒▒▒▒▓▒▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▓▓▓▓▓▓▓▒▒▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▓▓▓▓▓▓▓▓▒▒▒▓▓▓▒░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░▒▒▒░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░▒▒░░░░░             ░░░░░░▒▒▓▒▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░  
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒░▒▒▒▒▒▒░░░░░░░░▒▒░░░░░░░░░░▒░░▒▒▒░░░▒▒▒▒▒▒░░░▒░░░░░              ░░░░░░▒▒▒▒▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒░░░░░░  
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒░░▒▒▒▒▒▒▒▒▒░░░░░░░░░░░░░░░░░░░░░░░░░░░░▒▓▒▒▒▒░░░░░░░░░             ░░░░░░░▒▒▒▒▒▓▓▓▒▒▒▒▒▒▒▒▒▓▒░░░░░░░░
▓▓▓▒▒▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▓▓▓▓▓▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░▒▓▓▓▓▓▓▓▓▓▓▓▓░░▒▒▒▒▒░░░░▒▒▒░░░░░░          ░░░░░░░░▒▒▒▒▓▓▒▒▒▒▒▒▒▒▒▓▓▒░░░░░▒▒░
▓▓▓▓▓▓▒░░░░░░░░░░░░░░░▒▓▓▓░░░░░░░░░░░░░░░░░░░░░░░▒▓▒▒▓▓▓▒░░░░░░░░░░░░░▒▒▓▓▓▒▒▒▓▓░░░░░░░░░░░░░░░░░▒▒▓▓▒▒▓▒░░░░░░░░▒▓░░░▒▒▒▒░░▒▒▒▒▒▓▓▓▒▒░░░░░░ ░░░░▒▒▒▒▒▒▒▒▒▒▒▒▓▒▒░▒▒▒▒▒▒▒▒▓▒░░░░░▒░░
▓▓▓▓▒░░░░░░░░░░░░░░░░░░░▒▓░░░░░░░░░░░░░░░░░░░░░░░▒▓▓▓▒▒░░░░░░░░░░░░░░░░░░▒▓▓▒▒▓▓░░░░░░░░░░░░░░░░░░░░▒▓▓▓▒░░░░░░░░▒▓░░░▒▒▒░░░░░░░░▒▒▓▒▒▒▒░░░░░░▒▒▒▓▓▓▒▒░░▒▒▒▒▓▓▒   ░    ░░░░░   ░░░ 
▓▓▓▒░░░░░░░░░░░░░░░░░░░▒▓▓░░░░░░░░░░░░░░░░░░░░░░░▒▓▓▒░░░░░░░░░░░░░░░░░░░░░░▒▓▒▓▓░░░░░░░░░░░░░░░░░░░░░▒▓█▒░░░░░░░░▒▓░░░▒▒▒░░░░░▒▒░▓▒▓▒▒▒▒░   ░▒▒▓▓▓▓▓▒░▒▒▒▒░▒▓▒░░░░░░░░░░░░░░░░░░░░░
▓▓▓░░░░░░░░▒▓▓▓▓▓▓▒░░░▓▓▓▓▒▒▒▒▒▒▒░░░░░░░░░▒▒▒▒▒▒▒▓▓▒░░░░░░░░░▒▒▓▒▒░░░░░░░░░░▒▓▓▓░░░░░░░░▒▓▓▓▒░░░░░░░░░▒▓▓░░░░░░░░▓▓▒░▒▒░▒░░░░░░░░░▒░░░░░░░░ ░░░░░░▓▓▒▒▒▒▒░░▒▒▒░░▒▓▓▓▓▓▓▓▓▒▒▒▓▓▓▓▓▓▓
▓▓▓░░░░░░░░░░▒▒▒▓▓▓▓▓▓▓▓▒▓▓▓▓▓▓▓▓▒░░░░░░░░▓▓▓▓▓▓▓▓▒░░░░░░░░▒▓▓▓▓▓▓▓▓░░░░░░░░░▓▓▓░░░░░░░░▒▓▓▓▓▓▒░░░░░░░▒▓▓▒░░░░░░░▓▒░░▒░▒▒░░░ ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▒▓▒▒▒
▓▓▓▒░░░░░░░░░░░░░░░░▒▒▓▓▓▒▒▒▒▒▒▒▓▒░░░░░░░░▓▒▒▒▒▒▓▓░░░░░░░░▒▓▓▒▒░▒▒▓▓▓░░░░░░░░▒▓▓░░░░░░░░▒▓▓█▓▓▒░░░░░░░▒▓▓▒░░░░░░▒▓▒░▒░░▒░░░░░░ ░ ░░░░░░░░░░░░░░░░░░ ░░░░░░░▒░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▓▓▓▓▒░░░░░░░░░░░░░░░░░░░▓▓▒▒▒▒▒▒▓▒░░░░░░░░▓▒░▒▒▒▓▓░░░░░░░░▓▓▒░░░░░░▓▓░░░░░░░░▒▓▓░░░░░░░░░░▒░░░░░░░░░░░▓▓▓▓░░░░░░▓▓▒▒░░░▒░░░░░░░░░░   ░░░░░░░░░░░░░░░░░░░░░▒░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▓▓▓▓▓▓▓▒▒░░░░░░░░░░░░░░░░▓▓▒▒▒▒▒▓▒░░░░░░░░▓▒▒▒▒▒▓▓░░░░░░░░▒▓▒░░░░░▒▓▓░░░░░░░░▒▓▓░░░░░░░░░░░░░░░░░░░░░▓▓▒▒▓▒░░░░▒▓▒▒▒░░░░▒░░░░░░░░  ░ ░░░░░░░░░░░░░░░░░░░░░░░░░░░▒▒▒▒▒░░▒▒▒▒▒▒▒▒▒▒▒░
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒░░░░░░░░░▒▓▒▒▒▒▒▓▒░░░░░░░░▓▒░░░░▒▓▒░░░░░░░░▒▓▓▓▓▓▓▓▒░░░░░░░░░▓▓▓░░░░░░░░░░░░░░░░░░▒▓▓▓▒░▒▓▓░░░░▓▓▒▒▒▒░░░░░░░░░░░  ░░░░▒▒▒░░░░░░░░░░░░░░░░▒░░░░░▒░░░░░░▒░▒▒▒▒▒░░░░░░
▓▓▓▓▒░░░░▒▒▓▓▓▓▓▓▒░░░░░░░▒▓▒▒▒▒▒▓▒░░░░░░░░▓▒▒▒▒▒░▓▓░░░░░░░░░░▒▒▒▒▒░░░░░░░░░░▓▓▓▓░░░░░░░░▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▒▒░░░▒▒░░░░░░░░░░░░░▒░░░░░░░░░░░░░░░░░▒░░░▒░░▒▒░▒▒▒▒▒▒▒▒▒▒░░▒░░░
▓▓▒░░░░░░░░░░░░░░░░░░░░░░▓▓▒▒▒▒▒▓▒░░░░░░░░▓▓▒▒▒▒▒▒▓▓▒░░░░░░░░░░░░░░░░░░░░░░▓▓▒▓▓░░░░░░░░▒▓▓▓▓▓▓▓▓▓▒▒▒▒▒▓▓░░░░░░░░▓▓▒▒░░▒▒░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒░▒▒▒▒▒░░▒▒░░░
▓▓▒░░░░░░░░░░░░░░░░░░░░░▓▓▒▒▒▒▒▓▓▒░░░░░░░░▓▓▒▒▒▒▒▒▒▓▓▓░░░░░░░░░░░░░░░░░░░▒▓▓▒▒▓▓░░░░░░░░▒▓▒▒▒▒░░░░░▒░▒▒▓▓░░░░░░░░▓▓▒▒░░░░▒░░░░░░░░░░▒▒▒░░░░░░░░░░░░░░░░░░▓▓▓▓▒▒▒▒▒▒░░░░▒▒▒▒▒░▒▒▒░░░
▓▓▓▓▓▒░░░░░░░░░░░░░░░▒▓▓▓▒▒▒▒▒▒▓▓▒░░░░░░░░▓▓▒▒▒▒▒▒▒▒▓▓▓▓▒░░░░░░░░░░░░░▒▓▓▓▓▒▒▒▓▓░░░░░░░░▒▓░░░░░░░░░░░░░▓▓░░░░░░░░▓▓░▒▓▓▓▓▓▒░░░░░░░░░░░░░░░░▒░░░░▒▒░░░░░░▒▓▓▓▓▓▓▓▒▒▒░▒▒▒░▒▒▒▒▒▒▒▒▒░░
▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▒▒▒▓▓▓▓▓▓▓▓▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▒░░░░░░░░░░░░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░░░░░░░░░░░░░░░░▒▒░░░░░░░░░░▒▒▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒░▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▒░░░░░░░░░░░░░░▒▓▓▓▓▓▒▒▓▓▓▓▓▓▓▓▓▓▒░░░░░░░░░░░░ ░░░░░░░░░░░░▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▓▓▓▓▓▓▓▓▒▒▒▒▓▒▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░░░░░░▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓░░▒▒░░░░░░░░░░░░░░▒░░░░░▒▒▒▒▒▓▓▓▓▒▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒
▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░▒▒▒▒▒▒▒▒▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒░░░░░░░░░▒░░░▒░░░░▒▒▒▒▓▓▓▓▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▓▓▒▒▒▒
▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒░░░░░░░ ░▒▓▒░░░░░░░▒▒▒▓▓▓▓▓▓▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒░░░░░░░░░▓▓▒░░░░░░░▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░▒▒░▒▒░░░░░░░░░░░▒▓▒▓▓▒▓▓▓▓▓▓▓▓▓▓▓▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░▒▓▓▓░░░░░░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▓▓▓▓▓▓
▒▒▒▒▒▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░▒▒░░░░░░░░░░░░░░░▒▓▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░▒▓▓▓▒░░░▒▒░░▒▓▓▓▓▓▓▓▒▒▓▓▓▓▓▓▓▓▓▒▒▓▓▓▓▓▓▓▓▓▓
▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░▒▓▓▓▓▒░░░░░░░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▓▓
▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▓▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░░░░░░░░░▒▓▓▓▓▓░░░░░░░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒
▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒░░░░░░▒▒▒▓▓▓░░░░░░▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▓▓▓
▓▓▓▓▓▓▒▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▒▓▓▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒░░░▒▒▒▒▓▓▒░░░░░▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▒▓▓▓▓▓▓▓▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒░░▒▒▒▒▓▓▒░░░▒▒▒▒▓▓▓▓▓▓▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▒▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▒▓▓▓▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒░░░▒▒▒▒▓▓▒░▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▓▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▓▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▓▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▓▓▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▓▓▒▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▒▒▒▒▒▒▓▓▒▓▒▓▓▒▓▒▓▒▓▒▓▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▒▒▒▒▓▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▓▓▓▓▒▒▓▒▒▒▒▓▒▒▒▒▒▓▓▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▓▓▓▓▓▒▓▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓█▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▓▓▓▓▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▓▓▓▓▓▓▓█▓▓▓▓▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓█▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓█▓▓
", USIZEMAX_CONST_FMT);

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // region Boilerplate

    std::panic::set_hook(Box::new(|info| {
        let _ = messagebox::show_message_box(MessageBoxFlag::ERROR, 
        &[ButtonData {
            flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT | MessageBoxButtonFlag::ESCAPEKEY_DEFAULT,
            button_id: 0,
            text: "Ok"
        }], "Liblox crashed!", &info.payload_as_str().unwrap_or("Unknown panic"), None, None);
        ()
    }));

    let sdl_ctx = sdl3::init().unwrap();
    let video_subsys = sdl_ctx.video().unwrap();
    let win = video_subsys
        .window("Liblox", 1920, 1080)
        .fullscreen()
        .position_centered()
        .build()
        // .map_err(|e|e.to_string())
        .unwrap();

    let mut renderer = Renderer::new(
         Device::new(
            // SPIRV, DXIL, DXBC, METALLIB
            // but less boilerplate
            ShaderFormat(SDL_GPUShaderFormat(46u32)),
            true // TODO: change to false
        ).unwrap()
        .with_window(&win).unwrap(),
         None
    ).unwrap();

    let vert_shader = renderer.gpu()
        .create_shader()
        .with_code(
            ShaderFormat::SPIRV,
            include_bytes!("shaders/cube.vert.spv"),
            ShaderStage::Vertex
        )
        .with_uniform_buffers(2)
        .with_entrypoint(c"main")
        .build()?;
    let frag_shader = renderer.gpu()
        .create_shader()
        .with_code(
            ShaderFormat::SPIRV,
            include_bytes!("shaders/cube.frag.spv"),
            ShaderStage::Fragment
        )
        .with_entrypoint(c"main")
        .build()?;

    let swc_format = renderer.gpu().get_swapchain_texture_format(&win);
    let pipeline = renderer.gpu()
        .create_graphics_pipeline()
        .with_primitive_type(PrimitiveType::TriangleList)
        .with_fragment_shader(&frag_shader)
        .with_vertex_shader(&vert_shader)
        .with_vertex_input_state(
            VertexInputState::new()
                .with_vertex_buffer_descriptions(&[
                    VertexBufferDescription::new()
                        .with_slot(0)
                        .with_pitch(size_of::<Vertex>() as u32)
                        .with_input_rate(VertexInputRate::Vertex)
                        .with_instance_step_rate(0)
                ])
                .with_vertex_attributes(&[
                    VertexAttribute::new()
                        .with_format(VertexElementFormat::Float3)
                        .with_location(0)
                        .with_buffer_slot(0)
                        .with_offset(0),
                    VertexAttribute::new()
                        .with_format(VertexElementFormat::Float3)
                        .with_location(1)
                        .with_buffer_slot(0)
                        .with_offset(size_of::<Vec3>() as u32),
                    VertexAttribute::new()
                        .with_format(VertexElementFormat::Float2)
                        .with_location(2)
                        .with_buffer_slot(0)
                        .with_offset((size_of::<Vec3>() + size_of::<Vec2>()) as u32)
                ])
        )
        .with_rasterizer_state(
            RasterizerState::new()
                .with_fill_mode(FillMode::Fill)
                .with_cull_mode(CullMode::None) // TODO: make sure indices are good and change to back
                .with_front_face(FrontFace::CounterClockwise)
        )
        .with_depth_stencil_state(
            DepthStencilState::new()
                .with_enable_depth_test(true)
                .with_enable_depth_write(true)
                .with_compare_op(CompareOp::Less)   
        )
        .with_target_info(
            GraphicsPipelineTargetInfo::new()
                .with_color_target_descriptions(&[
                    ColorTargetDescription::new()
                        .with_format(swc_format)  
                ])   
                .with_has_depth_stencil_target(true)
                .with_depth_stencil_format(TextureFormat::D16Unorm)
        ).build().unwrap();

    drop(vert_shader);
    drop(frag_shader);

    // endregion

    let mut objlist: Vec<Box<dyn Object3D>> = vec![
        Box::new(Cube3D::new(
            Vec3::new(random_range(1.0..=5.0), random_range(1.0..=3.0), random_range(3.0..=5.0)),
            Vec3::new(random_range(0.5..=2.0), random_range(0.5..=2.0), random_range(0.5..=2.0)),
            Some(Quat::from_euler(
                glam::EulerRot::XYZ,
                random_range(0.0..std::f32::consts::TAU),
                random_range(0.0..std::f32::consts::TAU),
                random_range(0.0..std::f32::consts::TAU),
            ))
        )),
        Box::new(Plane3D::new(Vec3::NEG_Y, Vec3::new(10.0, 1.0, 10.0), None))
    ];

    let mut fov: f32 = 90.0;

    let mut proj_mat = glam::camera::lh::proj::directx::perspective(fov.to_radians(), 16.0/9.0, 0.01, 10.0);

    let mut view_mat = glam::camera::lh::view::look_at_mat4(Vec3::ZERO, Vec3::Z, Vec3::Y);

    let cp_commands = renderer.gpu().acquire_command_buffer()?;
    let cpass = renderer.gpu().begin_copy_pass(&cp_commands)?;

    let mut all_vertices: Vec<f32> = vec![];
    let mut all_indices: Vec<u32> = vec![];
    let mut all_srt: Vec<f32> = vec![];

    // vertex offset, indices offset
    // calculated using the next item in a peekable iterator's offset
    // or from end to start
    let mut offset_map: Vec<(usize, usize)> = vec![];

    // same as offset_map but for srts
    let mut srt_offset_map: Vec<usize> = vec![];

    for o in &objlist {
        offset_map.push((
            all_vertices.len() / (size_of::<Vertex>() / size_of::<f32>()),
            all_indices.len(),
        ));
        srt_offset_map.push(all_srt.len());
        all_vertices.extend(o.vertices().iter().flat_map(|v|v.to_slice()));
        all_indices.extend(o.indices().iter().map(|i|*i as u32));
        all_srt.extend(o.srt_matrix().to_cols_array());
    }

    println!("{all_srt:?}");
    println!("{all_indices:?}");
    println!("{all_vertices:?}");
    println!("{offset_map:?}");

    renderer.resize_or_create_txbuf((size_of_val(all_vertices.as_slice()).checked_add(size_of_val(all_indices.as_slice())).expect(RO_TOO_BIG)).try_into()?)?;

    let vert_buf = renderer.load_to_gpu(&cpass, BufferUsageFlags::VERTEX, &all_vertices, 0)?;
    let index_buf = renderer.load_to_gpu(&cpass, BufferUsageFlags::INDEX, &all_indices, all_vertices.len())?;

    renderer.gpu().end_copy_pass(cpass);
    let cpfence = cp_commands.submit_and_acquire_fence(renderer.gpu())?;

    let mut depth_tex = renderer.gpu().create_texture(
        TextureCreateInfo::new()
            .with_type(TextureType::_2D)
            .with_width(1920)
            .with_height(1080)
            .with_layer_count_or_depth(1)
            .with_num_levels(1)
            .with_format(TextureFormat::D16Unorm)
            .with_sample_count(SampleCount::NoMultiSampling)
            .with_usage(TextureUsage::SAMPLER | TextureUsage::DEPTH_STENCIL_TARGET)
    )?;

    println!("vertices: {}", all_vertices.len() / 8);
    println!("vertex buffer bytes: {}", vert_buf.len());
    println!("indices: {:?}", all_indices);
    println!("index buffer bytes: {}", index_buf.len());

    let mut evpump = sdl_ctx.event_pump()?;
    let mut wf: Option<Fence> = None;
    'rl: loop {
        for ev in evpump.poll_iter() {
            match ev {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape | Keycode::Q), .. } => {
                    break 'rl
                },
                Event::KeyDown { keycode: Some(Keycode::R), repeat: false, .. } => {
                    objlist[0] = Box::new(Cube3D::new(
                        Vec3::new(random_range(-5.0..=5.0), random_range(-3.0..=3.0), random_range(3.0..=5.0)),
                        Vec3::new(random_range(f32::EPSILON..=5.0), random_range(f32::EPSILON..=3.0), random_range(f32::EPSILON..=5.0)),
                        Some(Quat::from_euler(
                            glam::EulerRot::XYZ,
                            random_range(0.0..std::f32::consts::TAU),
                            random_range(0.0..std::f32::consts::TAU),
                            random_range(0.0..std::f32::consts::TAU),
                        ))
                    ))
                }
                _ => {}
            }
        }

        let mut cbuf = renderer.gpu().acquire_command_buffer()?;
        if let Ok(swc) = cbuf.wait_and_acquire_swapchain_texture(&win) {
            let col_tgts = [
                ColorTargetInfo::default()
                    .with_texture(&swc)
                    .with_clear_color(Color::RGB(0, 0, 0))
                    .with_load_op(LoadOp::CLEAR)
                    .with_store_op(StoreOp::STORE)
            ];
            let depth_tgt = DepthStencilTargetInfo::new()
                .with_texture(&mut depth_tex)
                .with_cycle(true)
                .with_clear_depth(1.0f32)
                .with_clear_stencil(0)
                .with_load_op(LoadOp::CLEAR)
                .with_store_op(StoreOp::STORE)
                .with_stencil_load_op(LoadOp::CLEAR)
                .with_stencil_store_op(StoreOp::STORE);

            all_srt.clear();
            srt_offset_map.clear();
            for o in &objlist {
                srt_offset_map.push(all_srt.len());
                all_srt.extend(o.srt_matrix().to_cols_array());
            }

            // println!("{srt_offset_map:?}");
            // println!("{all_srt:?}");

            let rpass = renderer.gpu().begin_render_pass(&cbuf, &col_tgts, Some(&depth_tgt))?;

            rpass.bind_graphics_pipeline(&pipeline);

            rpass.bind_vertex_buffers(0, 
                &[BufferBinding::new().with_buffer(&vert_buf).with_offset(0)]
            );
            rpass.bind_index_buffer(&BufferBinding::new().with_buffer(&index_buf).with_offset(0), IndexElementSize::_32BIT);

            let mut rd = vec![];

            rd.extend_from_slice(&proj_mat.to_cols_array());
            rd.extend_from_slice(&view_mat.to_cols_array());

            cbuf.push_vertex_uniform_data(0, &<[f32; 32]>::try_from(rd.as_slice()).unwrap());

            drop(rd);

            let mut offsets_iter = offset_map.iter().peekable();
            let mut srt_offsets_iter = srt_offset_map.iter();
            while let Some(&(vo, io)) = offsets_iter.next() {
                let oo = *srt_offsets_iter.next().unwrap();
                cbuf.push_vertex_uniform_data(1, &<[f32; 16]>::try_from(&all_srt[oo..oo+16]).unwrap());
                rpass.draw_indexed_primitives((offsets_iter.peek().copied().map(|p|p.1).unwrap_or(all_indices.len()) - io) as u32, 1, io as u32, vo as i32, 0);
            }

            renderer.gpu().end_render_pass(rpass);
            wf = Some(cbuf.submit_and_acquire_fence(renderer.gpu())?);
        } else {
            cbuf.cancel();
        }
    }

    if let Some(fe) = wf {
        renderer.gpu().wait_fences(true, &[fe, cpfence])?;
    }

    drop(index_buf);
    drop(vert_buf);
    drop(renderer.txbuf);

    Ok(())
}