pub open spec fn sbi_nacl_sync_sret_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (SbiFeatureEnabled(old_s, SBI_NACL_FEAT_SYNC_SRET) ==> result.error == 0)
    && (!SbiFeatureEnabled(old_s, SBI_NACL_FEAT_SYNC_SRET) ==> result.error != 0)
}