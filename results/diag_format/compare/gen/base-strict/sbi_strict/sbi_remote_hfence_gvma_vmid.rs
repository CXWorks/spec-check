pub open spec fn sbi_remote_hfence_gvma_vmid_spec(result: SbiErrorCode, hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, vmid: UInt64, old_s: S, new_s: S) -> bool {
    (forall h: HartId | IsHartSelected(hart_mask, hart_mask_base, h) ==> HartImplementsHypervisorExtension(h))
    && (forall h: HartId | IsHartSelected(hart_mask, hart_mask_base, h) ==> HfenceGvmaExecuted(h, start_addr, start_addr + size, vmid))
    && (forall h: HartId | IsHartSelected(hart_mask, hart_mask_base, h) ==> GStageTlbEntries(h, vmid, start_addr, start_addr + size))
    && (result == SBI_SUCCESS)
}