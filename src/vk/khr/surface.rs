use vk::*;
use libc::*;

pub const SPEC_VERSION: u32 = 25;
pub const EXTENSION_NAME: &'static str = "VK_KHR_surface";

impl_enum!{Result;
    ERROR_SURFACE_LOST = -1000000000,
    ERROR_NATIVE_WINDOW_IN_USE = -1000000001,
}
make_flag!{TransformFlag; TransformFlags;
    IDENTITY = 0x00000001,
    ROTATE_90 = 0x00000002,
    ROTATE_180 = 0x00000004,
    ROTATE_270 = 0x00000008,
    HORIZONTAL_MIRROR = 0x00000010,
    HORIZONTAL_MIRROR_ROTATE_90 = 0x00000020,
    HORIZONTAL_MIRROR_ROTATE_180 = 0x00000040,
    HORIZONTAL_MIRROR_ROTATE_270 = 0x00000080,
    INHERIT = 0x00000100,
}
make_flag!{CompositeAlphaFlag; CompositeAlphaFlags;
    OPAQUE = 0x00000001,
    PRE_MULTIPLIED = 0x00000002,
    POST_MULTIPLIED = 0x00000004,
    INHERIT = 0x00000008,
}
make_enum!{ColorSpace;
    SRGB_NONLINEAR = 0,
}
make_enum!{PresentMode;
    IMMEDIATE = 0,
    MAILBOX = 1,
    FIFO = 2,
    FIFO_RELAXED = 3,
}

opaque!{_Surface, Surface}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Capabilities {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: Extent2D,
    pub minImageExtent: Extent2D,
    pub maxImageExtent: Extent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: khr::surface::TransformFlags,
    pub currentTransform: khr::surface::TransformFlag,
    pub supportedCompositeAlpha: CompositeAlphaFlags,
    pub supportedUsageFlags: ImageUsageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Format {
    pub format: ::vk::Format,
    pub colorSpace: ColorSpace,
}

pub type PFN_vkDestroySurface =
    ::std::option::Option<unsafe extern "C" fn(instance: Instance,
                                               surface: Surface,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkGetPhysicalDeviceSurfaceSupport =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               queueFamilyIndex: u32,
                                               surface: Surface,
                                               pSupported: *mut Bool32)
                              -> Result>;
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               surface: Surface,
                                               pSurfaceCapabilities: *mut khr::surface::Capabilities)
                              -> Result>;
pub type PFN_vkGetPhysicalDeviceSurfaceFormats =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               surface: Surface,
                                               pSurfaceFormatCount: *mut u32,
                                               pSurfaceFormats: *mut khr::surface::Format)
                              -> Result>;
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               surface: Surface,
                                               pPresentModeCount: *mut u32,
                                               pPresentModes: *mut khr::surface::PresentMode)
                              -> Result>;

#[link(name = "vulkan")]
extern "C" {
    pub fn vkDestroySurface(instance: Instance, surface: khr::Surface,
                            pAllocator: *const AllocationCallbacks);
    pub fn vkGetPhysicalDeviceSurfaceSupport(physicalDevice: PhysicalDevice,
                                             queueFamilyIndex: uint32_t,
                                             surface: khr::Surface,
                                             pSupported: *mut Bool32) -> Result;
    pub fn vkGetPhysicalDeviceSurfaceCapabilities(physicalDevice: PhysicalDevice,
                                                  surface: khr::surface::Surface,
                                                  pSurfaceCapabilities: *mut khr::surface::Capabilities) -> Result;
    pub fn vkGetPhysicalDeviceSurfaceFormats(physicalDevice: PhysicalDevice,
                                             surface: khr::surface::Surface,
                                             pSurfaceFormatCount: *mut uint32_t,
                                             pSurfaceFormats: *mut khr::surface::Format) -> Result;
    pub fn vkGetPhysicalDeviceSurfacePresentModes(physicalDevice: PhysicalDevice,
                                                  surface: khr::Surface,
                                                  pPresentModeCount: *mut uint32_t,
                                                  pPresentModes: *mut khr::surface::PresentMode) -> Result;
}
