pub type PVOID = *mut core::ffi::c_void;

extern "system" {
    pub fn MmIsAddressValid(virtual_address: PVOID) -> bool;
}
