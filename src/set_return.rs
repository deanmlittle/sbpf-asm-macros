/// # Set Return from Immediate
/// Sets the return register `r0` to an immediate value.
///
/// ### CU Cost
/// 1 CU
/// 
/// ### ASM
/// `lddw r0, imm`
/// 
/// ### Parameters
/// - `value`: The immediate value to set the return register.
///
/// ### Example
/// ```
/// set_return_imm(1337);
/// ```
///
/// This will set the register `r0` to `1337`.
#[macro_export]
macro_rules! set_return_imm {
    ($value:expr) => {
        let _: u64 = $value; // Compile-time error check for u64
        unsafe {
            core::arch::asm!(concat!("lddw r0, ", stringify!($value)));
        }
    };
}

/// # Set Return from Register
/// Sets the return register `r0` to a value stored in a register. For static return values, consider using `set_return_imm`, as it will be 1 CU cheaper by avoiding an additional register allocation.
///
/// ### CU Cost
/// 1 CU (+1 CU for register allocation)
/// 
/// ### ASM
/// `mov64 r0, r1`
/// 
/// ### Parameters
/// - `value`: The stored value to set the return register.
///
/// ### Example
/// ```
/// let n = 1337;
/// set_return_reg(n);
/// ```
///
/// This will assign `1337` to `n`, then set the register `r0` to `n`.

#[macro_export]
macro_rules! set_return_reg {
    ($value:expr) => {
        let _: u64 = $value; // Compile-time error check for u64
        unsafe {
            core::arch::asm!(
                "mov64 r0, {0}",
                in(reg) $value
            );
        }
    };
}