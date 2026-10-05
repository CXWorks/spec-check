pub open spec fn sbi_nacl_sync_hfence_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let entry_index = old_s.cmd_input_entry_index;
    let is_all_ones = entry_index == ((1u64 << 64) - 1);
    let max_index = 3840 / (old_s.xlen as u64);
    let feature_available = old_s.impl_features.feat_sync_hfence == RmmFeature::TRUE;
    let shmem_available = old_s.nacl_shmem_available;

    (!feature_available ==> result.error == SBI_ERR_NOT_SUPPORTED)
    && (!shmem_available ==> result.error == SBI_ERR_NO_SHMEM)
    && (!is_all_ones && entry_index >= max_index ==> result.error == SBI_ERR_INVALID_PARAM)
    && (feature_available && shmem_available && (!is_all_ones && entry_index < max_index) ==> result.error == SBI_SUCCESS)
    && (feature_available && shmem_available && is_all_ones ==> result.error == SBI_SUCCESS)
}