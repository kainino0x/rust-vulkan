pub type DeviceSize = u64;
pub type SampleMask = u32;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Bool32 {
    False = 0,
    True = 1,
}

pub const FALSE: Bool32 = Bool32::False;
pub const TRUE: Bool32 = Bool32::True;

// Opaque structs

opaque!{_Instance, Instance}
opaque!{_PhysicalDevice, PhysicalDevice}
opaque!{_Device, Device}
opaque!{_Queue, Queue}
opaque!{_Semaphore, Semaphore}
opaque!{_CommandBuffer, CommandBuffer}
opaque!{_Fence, Fence}
opaque!{_DeviceMemory, DeviceMemory}
opaque!{_Buffer, Buffer}
opaque!{_Image, Image}
opaque!{_Event, Event}
opaque!{_QueryPool, QueryPool}
opaque!{_BufferView, BufferView}
opaque!{_ImageView, ImageView}
opaque!{_ShaderModule, ShaderModule}
opaque!{_PipelineCache, PipelineCache}
opaque!{_PipelineLayout, PipelineLayout}
opaque!{_RenderPass, RenderPass}
opaque!{_Pipeline, Pipeline}
opaque!{_DescriptorSetLayout, DescriptorSetLayout}
opaque!{_Sampler, Sampler}
opaque!{_DescriptorPool, DescriptorPool}
opaque!{_DescriptorSet, DescriptorSet}
opaque!{_Framebuffer, Framebuffer}
opaque!{_CommandPool, CommandPool}
