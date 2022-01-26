use ash::vk;

fn main() {
    unsafe {
        #[cfg(feature = "linked")]
        let entry = ash::Entry::linked();

        #[cfg(all(not(feature = "linked"), feature = "loaded"))]
        let entry = ash::Entry::load().unwrap();

        let instance = entry
            .create_instance(&vk::InstanceCreateInfo::builder(), None)
            .unwrap();
        instance.destroy_instance(None);
    }
}
