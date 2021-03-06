//! Copyright (c) ChefKiss Inc 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives license.

super::generic_exception!(div0_handler, "division by zero");
super::generic_exception!(debug_handler, "debug");
super::generic_exception!(nmi_handler, "non-maskable interrupt");
super::generic_exception!(breakpoint_handler, "breakpoint");
super::generic_exception!(overflow_handler, "overflow");
super::generic_exception!(bound_range_handler, "bound range exceeded");
super::generic_exception!(invalid_opcode_handler, "invalid opcode");
super::generic_exception!(dev_unavailable_handler, "device unavailable");
super::generic_exception!(
    coproc_segment_overrun_handler,
    "coprocessor segment overrun"
);

pub(crate) unsafe extern "sysv64" fn reserved_handler(regs: &mut crate::sys::RegisterState) {
    exc_msg!("reserved", regs);
    error!("This should NEVER happen! Make an issue and attach the serial output.");
}

super::generic_exception!(x87_fp_handler, "x87 floating-point");
super::generic_exception!(align_chk_handler, "alignment check");
super::generic_exception!(machine_chk_handler, "machine check");
super::generic_exception!(simd_fp_handler, "SIMD floating-point");
super::generic_exception!(hv_injection_handler, "hypervisor injection");
super::generic_exception!(vmm_com_handler, "VMM communication");
super::generic_exception!(security_handler, "security");

pub(crate) unsafe extern "sysv64" fn spurious(_regs: &mut crate::sys::RegisterState) {
    use log::warn;

    while crate::sys::io::serial::SERIAL.is_locked() {
        crate::sys::io::serial::SERIAL.force_unlock()
    }

    warn!("Received spurious interrupt.");
}
