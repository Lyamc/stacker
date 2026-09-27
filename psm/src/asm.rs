//! Stack pointer and stack switching, assembled by rustc. Replaces the
//! `cc` build that used to compile these same instructions.

use core::arch::global_asm;

#[cfg(all(target_arch = "x86_64", not(any(target_os = "macos", target_os = "ios"))))]
global_asm!(
    r#"
    .intel_syntax noprefix
    .globl rust_psm_stack_direction
    rust_psm_stack_direction:
        mov al, 2
        ret
    .globl rust_psm_stack_pointer
    rust_psm_stack_pointer:
        lea rax, [rsp + 8]
        ret
    "#
);

#[cfg(all(target_arch = "x86_64", not(target_os = "windows"), not(any(target_os = "macos", target_os = "ios"))))]
global_asm!(
    r#"
    .intel_syntax noprefix
    .globl rust_psm_replace_stack
    rust_psm_replace_stack:
        lea rsp, [rdx - 8]
        jmp rsi
    .globl rust_psm_on_stack
    rust_psm_on_stack:
        push rbp
        mov rbp, rsp
        mov rsp, rcx
        call rdx
        mov rsp, rbp
        pop rbp
        ret
    "#
);

#[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "ios")))]
global_asm!(
    r#"
    .intel_syntax noprefix
    .globl _rust_psm_stack_direction
    _rust_psm_stack_direction:
        mov al, 2
        ret
    .globl _rust_psm_stack_pointer
    _rust_psm_stack_pointer:
        lea rax, [rsp + 8]
        ret
    .globl _rust_psm_replace_stack
    _rust_psm_replace_stack:
        lea rsp, [rdx - 8]
        jmp rsi
    .globl _rust_psm_on_stack
    _rust_psm_on_stack:
        push rbp
        mov rbp, rsp
        mov rsp, rcx
        call rdx
        mov rsp, rbp
        pop rbp
        ret
    "#
);

#[cfg(all(target_arch = "x86", not(any(target_os = "macos", target_os = "ios"))))]
global_asm!(
    r#"
    .intel_syntax noprefix
    .globl rust_psm_stack_direction
    rust_psm_stack_direction:
        mov al, 2
        ret
    .globl rust_psm_stack_pointer
    rust_psm_stack_pointer:
        lea eax, [esp + 4]
        ret
    "#
);

#[cfg(all(target_arch = "aarch64", not(any(target_os = "macos", target_os = "ios"))))]
global_asm!(
    r#"
    .globl rust_psm_stack_direction
    rust_psm_stack_direction:
        orr w0, wzr, #2
        ret
    .globl rust_psm_stack_pointer
    rust_psm_stack_pointer:
        mov x0, sp
        ret
    "#
);

#[cfg(all(target_arch = "aarch64", not(target_os = "windows"), not(any(target_os = "macos", target_os = "ios"))))]
global_asm!(
    r#"
    .globl rust_psm_replace_stack
    rust_psm_replace_stack:
        mov sp, x2
        br x1
    .globl rust_psm_on_stack
    rust_psm_on_stack:
        stp x29, x30, [sp, #-16]!
        mov x29, sp
        mov sp, x3
        blr x2
        mov sp, x29
        ldp x29, x30, [sp], #16
        ret
    "#
);
