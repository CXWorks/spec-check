pub open spec fn sbi_remote_sfence_vma_spec(hart_mask: unsigned long, hart_mask_base: unsigned long, start_addr: unsigned long, size: unsigned long, result: sbiret.error, old_s: S, new_s: S) -> bool {
  (!IsValidStartAddr(old_s, start_addr) || !IsValidSize(old_s, size) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> IpiSentToAll(old_s, TargetHarts(old_s, hart_mask, hart_mask_base)))
  && (result == SBI_SUCCESS ==> (forall hart in TargetHarts(old_s, hart_mask, hart_mask_base): SfenceVmaExecuted(old_s, hart, start_addr, start_addr + size)))
  && ((IsValidStartAddr(old_s, start_addr) && IsValidSize(old_s, size))
    ==> result == SBI_SUCCESS)
}