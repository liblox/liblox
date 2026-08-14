use sdl3::gpu::{Buffer, BufferRegion, BufferUsageFlags, CopyPass, Device, TransferBuffer, TransferBufferLocation, TransferBufferUsage};

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
                .with_offset((offset * size_of::<T>()) as u32)
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