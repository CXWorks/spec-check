pub open spec fn sbi_remote_hfence_gvma_vmid_spec(result: i64, old_s: S, new_s: S) -> bool {
    (result < 0 ==> false)
    && (result >= 0 ==> HfenceGvmaExecutedOnHarts(old_s, hart_mask, hart_mask_base, start_addr, size, vmid))
}