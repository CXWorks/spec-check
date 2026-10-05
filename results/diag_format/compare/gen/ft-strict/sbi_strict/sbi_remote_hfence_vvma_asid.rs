pub open spec fn sbi_remote_hfence_vvma_asid_spec(hart_mask: unsigned long, hart_mask_base: unsigned long, start_addr: unsigned long, size: unsigned long, asid: unsigned long, result: Result<(), SbiStatusCode>, old_s: S, new_s: S) -> bool {
  (result == SBI_SUCCESS ==> RemoteHartsExecutedHfenceVvma(new_s, hart_mask, hart_mask_base, start_addr, start_addr + size, asid, CurrentVmid(new_s)))
}