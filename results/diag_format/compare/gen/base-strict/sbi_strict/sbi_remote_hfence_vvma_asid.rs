pub open spec fn sbi_remote_hfence_vvma_asid_spec(result: i64, old_s: S, new_s: S) -> bool {
    (RemoteHartsExecutedHfenceVvma(hart_mask(old_s), hart_mask_base(old_s), start_addr(old_s), start_addr(old_s) + size(old_s), asid(old_s), CurrentVmid()) ==> result == 0)
}