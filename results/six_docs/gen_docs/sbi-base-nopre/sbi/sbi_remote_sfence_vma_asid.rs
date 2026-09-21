pub open spec fn sbi_remote_sfence_vma_asid_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let hart_mask: u64 = old_s.cmd_input_hart_mask;
    let hart_mask_base: u64 = old_s.cmd_input_hart_mask_base;
    let start_addr: u64 = old_s.cmd_input_start_addr;
    let size: u64 = old_s.cmd_input_size;
    let asid: u64 = old_s.cmd_input_asid;

    (result.error == SBI_ERR_INVALID_ADDRESS ==> (start_addr as int < 0 || size as int < 0 || (start_addr as int + size as int) < (start_addr as int)))
    && (result.error == SBI_ERR_INVALID_PARAM ==> (asid as int < 0 || hart_mask as int < 0 || hart_mask_base as int < 0))
    && (result.error == SBI_ERR_FAILED ==> true)
    && (result.error == SBI_SUCCESS ==> true)
}