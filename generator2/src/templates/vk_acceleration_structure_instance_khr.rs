#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureInstanceKHR {
    pub transform: TransformMatrixKHR,
    pub instance_custom_index_and_mask: u32,
    pub instance_shader_binding_table_record_offset_and_flags: u32,
    pub acceleration_structure_reference: AccelerationStructureReferenceKHR,
}
