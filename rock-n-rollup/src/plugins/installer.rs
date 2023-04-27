use crate::core::Runtime;

pub trait Installer {
    // Install the kernel to /boot/kernel.wasm
    fn install(&mut self, kernel: &[u8]) -> Result<(), ()>;
}

impl<R> Installer for R
where
    R: Runtime,
{
    fn install(&mut self, kernel: &[u8]) -> Result<(), ()> {
        let path = "/tmp/boot/kernel.wasm";

        self.store_delete(path)?;

        // The kernel can be bigger than 4k
        // That's why we need to split the kernel slice to chunkcs of 4kb
        let chunk_size = 2048;

        // Write all chunks of data
        let _ = kernel
            .chunks(chunk_size)
            .fold(Ok(0), |kernel_size, chunk| match kernel_size {
                Err(()) => Err(()),
                Ok(kernel_size) => {
                    let chunk_size = chunk.len();
                    self.store_write_raw(path, chunk, kernel_size)?;
                    Ok(kernel_size + chunk_size)
                }
            })?;

        self.store_move(path, "/kernel/boot.wasm")?;
        Ok(())
    }
}
