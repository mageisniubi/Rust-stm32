#![no_std]
#![no_main]
#[export_name = "main"]
fn main() {

    unsafe {
        // 首先要使能对应 GPIO 端口的时钟
        let rcc_base = 0x4002_3800;
        let rcc_ahb1_ptr = (rcc_base + 0x30) as *mut u32;
        // 将 GPIOC 时钟使能
        *rcc_ahb1_ptr |= 1 << 2;

        // 配置 GPIOC9 为推挽输出模式
        let gpioc_base = 0x4002_0800;
        let gpioc_mode = (gpioc_base + 0x00) as *mut u32;
        // 使用位掩码设置输出模式为推挽输出
        *gpioc_mode |= 0b01 << 18;     // 设置为推挽输出

        // 配置上下拉
        let gpioc_pull = (gpioc_base + 0x0c) as *mut u32;
        *gpioc_pull |= 0b01 << 18;      // 设置为上拉

        // 配置置位寄存器，将 GPIOC9 输出置高
        let gpioc_bsrr = (gpioc_base + 0x18) as *mut u32;
        *gpioc_bsrr |= 1 << (9+16);
    }


    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

