use vk_debug::VulkanDebug;
use vk_device::VulkanDevice;

mod vk_debug;
mod vk_device;
mod vk_instance;

#[derive(thiserror::Error, Debug)]
pub enum RunError {
    #[error(transparent)]
    AshEntryError(#[from] ash::LoadingError),
    #[error(transparent)]
    VulkanResult(#[from] ash::vk::Result),
}

pub struct Vulkan {
    pub instance: ash::Instance,
    pub entry: ash::Entry,

    debug: VulkanDebug,
    device: VulkanDevice,
}

impl Vulkan {
    pub fn new() -> Result<Self, RunError> {
        let entry = unsafe { ash::Entry::load() }?;
        let instance = Self::get_instance(&entry)?;

        let debug = Self::get_debug(&entry, &instance)?;
        let device = Self::get_device(&instance)?.unwrap();

        Ok(Self {
            entry,
            instance,
            debug,
            device,
        })
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        self.destroy_device();
        self.destroy_debug();
        self.destroy_instance();
    }
}

pub fn run() -> Result<(), RunError> {
    let _vulkan = Vulkan::new()?;
    Ok(())
}
