#![no_std]

#[panic_handler]
fn _panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

use core::mem::*;
use windows_sys::{
    Wdk::Foundation::*, Wdk::Storage::FileSystem::Minifilters::*, Win32::Foundation::*,
};

#[no_mangle]
extern "system" fn DriverEntry(
    driver: *mut DRIVER_OBJECT,
    _path: *const UNICODE_STRING,
) -> NTSTATUS {
    unsafe {
        let mut reg: FLT_REGISTRATION = zeroed();
        reg.Size = size_of::<FLT_REGISTRATION>() as _;
        reg.Version = FLT_REGISTRATION_VERSION as _;
        reg.FilterUnloadCallback = Some(unload_callback);

        let mut handle = 0;
        let status = FltRegisterFilter(driver, &reg, &mut handle);
        assert_eq!(status, 0);

        STATUS_SUCCESS
    }
}

extern "system" fn unload_callback(_flags: u32) -> NTSTATUS {
    STATUS_SUCCESS
}
