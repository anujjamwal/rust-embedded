pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManageFault();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    fn SysTick();
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector {
        handler: MemManageFault,
    },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {}
