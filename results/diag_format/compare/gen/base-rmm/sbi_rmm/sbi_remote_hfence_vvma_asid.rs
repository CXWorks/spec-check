pub open spec fn sbi_remote_hfence_vvma_asid_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.ret == 0 ==> ForAll(hart in HartsSelectedBy(old_s, hart_mask, hart_mask_base): ExecutedHfenceVvma(hart, start_addr, start_addr + size, asid, CurrentVmid())))
}