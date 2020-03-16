#![allow(dead_code)]
use crate::prelude::*;
use crate::version::{EntryV1_0, InstanceV1_0};
use crate::vk;
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct TimelineSemaphore {
    handle: vk::Instance,
    timeline_semaphore_fn: vk::KhrTimelineSemaphoreFn,
}

impl TimelineSemaphore {
    pub fn new<E: EntryV1_0, I: InstanceV1_0>(entry: &E, instance: &I) -> TimelineSemaphore {
        let timeline_semaphore_fn = vk::KhrTimelineSemaphoreFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        TimelineSemaphore {
            handle: instance.handle(),
            timeline_semaphore_fn,
        }
    }

    pub fn name() -> &'static CStr {
        vk::KhrTimelineSemaphoreFn::name()
    }

    pub fn fp(&self) -> &vk::KhrTimelineSemaphoreFn {
        &self.timeline_semaphore_fn
    }

    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}