use crate::vk;
pub type VkResult<T> = Result<T, vk::Result>;

impl From<vk::Result> for VkResult<()> {
    fn from(err_code: vk::Result) -> Self {
        match err_code {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err_code),
        }
    }
}
