extern crate ash;
use ash::vk;
#[test]
fn test_ptr_chains() {
    let mut variable_pointers = vk::PhysicalDeviceVariablePointerFeatures::builder();
    let mut corner = vk::PhysicalDeviceCornerSampledImageFeaturesNV::builder();
    let chain = vec![
        &variable_pointers as *const _ as usize,
        &corner as *const _ as usize,
    ];
    let mut device_create_info = vk::DeviceCreateInfo::builder()
        .push_next(&mut corner)
        .push_next(&mut variable_pointers);
    let chain2: Vec<usize> = unsafe {
        vk::ptr_chain_iter(&mut device_create_info)
            .skip(1)
            .map(|ptr| ptr as usize)
            .collect()
    };
    assert_eq!(chain, chain2);
}
