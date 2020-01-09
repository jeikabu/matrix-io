#![no_std]
#![no_main]

#[no_mangle]
pub fn app_main() {
    unsafe {
        everloop();
    }
}

unsafe fn everloop() {
    use matrix_hal_esp32_sys::*;
    let mut wb = matrix_hal::WishboneBus::default();
    wb.Init();

    ets_printf(b"Hello World\n\0".as_ptr() as *const _);

    // let mut everloop = matrix_hal::Everloop::new();
    // everloop._base.Setup(&mut wb);

    let mut counter = 0;
    loop {
        const NUMBER_LEDS: usize = matrix_hal::kMatrixCreatorNLeds as usize;
        let mut image1d = [0u8; NUMBER_LEDS * 4];
        let blue = (libm::sinf(counter as f32 / 64.0) * 10.0 + 10.0) as u8;
        ets_printf(b"counter=%d blue=%d\n\0".as_ptr() as *const _, counter as f32 as u32, blue as cty::c_uint);
        for i in 0..NUMBER_LEDS {
            image1d[i * 4 + 2] = blue;
        }
        wb.SpiWrite(matrix_hal::kEverloopBaseAddress as u16, image1d.as_ptr(), image1d.len() as i32);
        counter += 1;
    }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use matrix_hal_esp32_sys::*;
    unsafe {
        let format = b"PANIC!\n\0";
        ets_printf(format.as_ptr() as *const _);
    }
    loop {}
}