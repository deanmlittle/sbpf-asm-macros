# sbpf-asm-macros

A series of macros to enable optimal low-level sBPF functionality such as directly setting register values which would otherwise be inaccessible.

### Usage

These macros enable you to directly set registers, and may also provide other low-level functionality in the future to enable extreme optimisation down to the bytecode level. Below is one such example.

##### Near-optimal bytecode

A simple example of close-to-optimal Rust program that checks the balance of a token account against an amount in instruction data would be:

```rs
#[no_mangle]
pub unsafe extern "C" fn entrypoint(ptr: *mut u8) -> u64 {
    let ix_balance = (ptr as *const u8).add(0x2918) as *const u64;
    let account_balance = (ptr as *const u8).add(0x00a0) as *const u64;
    if *ix_balance > *account_balance {
        return 1
    }
    0
}
```

This produces the following ASM:

```asm
ldxdw r2, [r1+160]    ; 1CU
ldxdw r1, [r1+10520]  ; 1CU
mov64 r0, 1           ; 1CU
jgt r1, r2, +1        ; 1CU
mov64 r0, 0           ; 1CU
exit                  ; 1CU
```

__CU Costs:__
Success: 6 CUs
Failure: 5 CUs

##### Optimal bytecode

As SVM initializes its return register 0 with the value of 0, it is possible to save compute units by removing the return signature of the entrypoint and anticipating the value of the return register to be 0 rather than explicitly setting it. With `sbpf-asm-macros`, we are able to ergonomically access the return register to make such optimizations:

```rs
#![cfg_attr(target_os = "solana", feature(asm_experimental_arch, asm_const))]
use sbpf_asm::set_return_imm;

#[no_mangle]
pub unsafe extern "C" fn entrypoint(ptr: *mut u8) {
    let ix_balance = (ptr as *const u8).add(0x2918) as *const u64;
    let account_balance = (ptr as *const u8).add(0x00a0) as *const u64;
    if *ix_balance > *account_balance {
        set_return_imm!(1);
    }
}
```

This produces the following ASM:

```asm
ldxdw r2, [r1+10520] ; 1CU
ldxdw r1, [r1+160]   ; 1CU
jge r1, r2, +2       ; 1CU
lddw r9, 1337        ; 1CU
exit                 ; 1CU
```

__CU Costs:__
Success: 4 CUs
Failure: 5 CUs

As you can see, ASM unlocks some very low-level optimizations that would otherwise be unreachable.
