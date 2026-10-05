pub open spec fn sbi_remote_hfence_gvma_vmid_spec(hart_mask: UnsignedLong, hart_mask_base: UnsignedLong, start_addr: UnsignedLong, size: UnsignedLong, vmid: UnsignedLong, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (forall (h: HartId), IsHartSelected(hart_mask, hart_mask_base, h) ==> HartImplementsHypervisorExtension(new_s))
  && (forall (h: HartId), IsHartSelected(hart_mask, hart_mask_base, h) ==> HfenceGvmaExecuted(new_s, h, start_addr, start_addr + size, vmid))
  && (forall (h: HartId), IsHartSelected(hart_mask, hart_mask_base, h) ==> GStageTlbEntries(new_s, h, vmid, start_addr, start_addr + size))
  && ((!(result == SBI_SUCCESS)) ==> (forall (h: HartId), !(IsHartSelected(hart_mask, hart_mask_base, h) ==> HartImplementsHypervisorExtension(new_s))))
  && (result == SBI_SUCCESS ==> forall (h: HartId), IsHartSelected(hart_mask, hart_mask_base, h) ==> HfenceGvmaExecuted(new_s, h, start_addr, start_addr + size, vmid))
}