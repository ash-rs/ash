#![allow(dead_code)]
use crate::vk;
use std::ffi::CStr;

#[derive(Clone)]
pub struct RayQuery {}

impl RayQuery {
    pub fn name() -> &'static CStr {
        vk::KhrRayQueryFn::name()
    }
}
