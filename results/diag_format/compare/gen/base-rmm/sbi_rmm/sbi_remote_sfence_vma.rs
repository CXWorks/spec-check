pub open spec fn sbi_remote_sfence_vma_spec(result: sbiret, hart_mask: u64, hart_mask_base: u64, start_addr: u64, size: u64, old_s: S, new_s: S) -> bool {
    (!IsValidStartAddr(start_addr) || !IsValidSize(size) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
    && (ResultEqual(result, SBI_SUCCESS) ==> IpiSentToAll(TargetHarts(hart_mask, hart_mask_base)))
    && (ResultEqual(result, SBI_SUCCESS) ==> forall hart in TargetHarts(hart_mask, hart_mask_base): SfenceVmaExecuted(hart, start_addr, start_addr + size))
}