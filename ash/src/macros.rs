#[macro_export]
macro_rules! vk_make_version {
    ( $ major : expr , $ minor : expr , $ patch : expr ) => {
        (($major as u32) << 22) | (($minor as u32) << 12) | $patch as u32
    };
}
#[macro_export]
macro_rules! vk_version_major {
    ( $ major : expr ) => {
        ($major as u32) >> 22
    };
}
#[macro_export]
macro_rules! vk_version_minor {
    ( $ minor : expr ) => {
        (($minor as u32) >> 12) & 0x3ff
    };
}
#[macro_export]
macro_rules! vk_version_patch {
    ( $ minor : expr ) => {
        ($minor as u32) & 0xfff
    };
}
