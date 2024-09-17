/// # Set Register Immediate
/// Set a specific register to an immedate value.
///
/// ### CU Cost
/// 1 CU
/// 
/// ### ASM
/// lddw r3, imm
/// 
/// ### Parameters
/// - `register`: The register to set. Must be 0-9.
/// - `value`: The value to set the return register.
///
/// ### Example
/// ```
/// set_register_imm(3, 1337);
/// ```
///
/// This will set register `r3` to `1337`.
#[macro_export]
macro_rules! set_register_imm {
    ($register:expr,$value:expr) => {
        let _: u64 = $value; // Compile-time error check for u64

        // Compile-time check for register bounds (0-9)
        const R: u8 = $register;
        // Ensure the register is within the valid range
        #[allow(overflowing_literals)]
        const _: [(); 0 - ((R > 9) as usize)] = []; // Compile-time error to check for R > 9
        
        unsafe {
            core::arch::asm!(
                "lddw r{reg}, {val}",
                reg = const $register,
                val = const $value,
                options(nomem, nostack, preserves_flags)
            );
        }
    };
}