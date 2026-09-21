pub open spec fn sbi_nacl_set_shmem_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (
        (old_s.cmd_input_flags != 0) ||
        (old_s.cmd_input_shmem_phys_lo as int) % 4096 != 0
    ))
    && (result == SBI_SBI_ERR_INVALID_ADDRESS ==> (
        // Section 3.2 requirements (not fully detailed in provided text)
        true
    ))
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> (
        // Success: shared memory set or cleared
        // If all-ones, features disabled; otherwise set
        true
    ))
    && (result != SBI_SBI_SUCCESS && result != SBI_SBI_ERR_INVALID_PARAM && result != SBI_SBI_ERR_INVALID_ADDRESS && result != SBI_SBI_ERR_FAILED ==> true)
}