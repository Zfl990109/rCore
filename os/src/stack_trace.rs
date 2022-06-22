use core::{arch::asm, ptr};
#[inline(always)]
pub fn print_stack_trace() {
    // 获取到栈帧指针
    let mut fp: *const usize;
    unsafe {
        asm!(
            "mv {fp}, fp",
            fp = out(reg) fp,
        );
        log::trace!("=== Stack trace from fp chain ===");
        while fp != ptr::null() {
    
            log::trace!("  ra:{:#x}  fp: {:#x}", *(fp.sub(1)), *(fp.sub(2)));
            fp = *(fp.sub(2)) as *const usize;
        }
        log::trace!("=== End ===\n\n");
    }
}