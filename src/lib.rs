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
}

impl Vulkan {
    pub fn new() -> Result<Self, RunError> {
        let entry = unsafe { ash::Entry::load() }?;
        let instance = Self::get_instance(&entry)?;

        Ok(Self { entry, instance })
    }
}

pub fn run() -> Result<(), RunError> {
    let _vulkan = Vulkan::new()?;
    Ok(())
}
