pub open spec fn sbi_remote_hfence_gvma_vmid_spec(hart_mask: unsigned long, hart_mask_base: unsigned long, start_addr: unsigned long, size: unsigned long, vmid: unsigned long, result: struct sbiret, old_s: S, new_s: S) -> bool {
  (result.error == 0 ==> HfenceGvmaExecutedOnHarts(new_s, hart_mask, hart_mask_base, start_addr, size, vmid))
}