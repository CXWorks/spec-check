pub open spec fn sbi_nacl_sync_hfence_spec(result: i64, old_s: S, new_s: S) -> bool {
    let feature_available = old_s.nacl_features & (1u64 << SBI_NACL_FEAT_SYNC_HFENCE) != 0;
    let entry_index = old_s.nacl_sync_hfence_entry_index;
    let max_entries = 3840u64 / (old_s.xlen as u64);
    let is_all_ones = entry_index == ((1u64 << (old_s.xlen as u64)) - 1);
    let is_single_entry = entry_index < max_entries;
    let is_invalid_param = !is_all_ones && !is_single_entry;
    let has_shmem = old_s.nacl_shmem_available;

    (!feature_available ==> result == SBI_ERR_NOT_SUPPORTED)
    && (feature_available && !has_shmem ==> result == SBI_ERR_NO_SHMEM)
    && (feature_available && has_shmem && is_invalid_param ==> result == SBI_ERR_INVALID_PARAM)
    && (feature_available && has_shmem && !is_invalid_param ==> result == SBI_SUCCESS)
}