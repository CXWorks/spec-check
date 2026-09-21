pub open spec fn sbi_nacl_sync_hfence_spec(result: int, entry_index: UInt64, old_s: S, new_s: S) -> bool {
    (SbiFeatureNotAvailable(old_s, SBI_NACL_FEAT_SYNC_HFENCE) ==> ResultEqual(result, SBI_SBI_ERR_NOT_SUPPORTED))
    && (SbiFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_HFENCE) ==> (
        (entry_index == !0u64 ==> ResultEqual(result, SBI_SBI_SUCCESS))
        && (entry_index < (3840u64 / (old_s.xlen as u64)) ==> (
            (entry_index >= (3840u64 / (old_s.xlen as u64)) ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
            && (ResultEqual(result, SBI_SBI_SUCCESS) ==> <unchanged state constraints>)
        ))
        && (entry_index >= (3840u64 / (old_s.xlen as u64)) && entry_index != !0u64 ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
        && (ResultEqual(result, SBI_SBI_ERR_NO_SHMEM) ==> <no shmem constraint>)
    ))
}