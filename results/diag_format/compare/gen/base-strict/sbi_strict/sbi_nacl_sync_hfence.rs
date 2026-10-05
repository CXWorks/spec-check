pub open spec fn sbi_nacl_sync_hfence_spec(result: sbiret, entry_index: UInt, old_s: S, new_s: S) -> bool {
    (!NaclFeatureAvailable(SBI_NACL_FEAT_SYNC_HFENCE) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (!IsAllOnes(entry_index) && entry_index >= (3840 / XLEN) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (!NaclSharedMemoryAvailable() ==> ResultEqual(result, SBI_ERR_NO_SHMEM))
    && (ResultEqual(result, SBI_SUCCESS) ==> (IsAllOnes(entry_index) ==> AllNestedHfenceEntriesSynchronized() && entry_index < (3840 / XLEN) ==> NestedHfenceEntrySynchronized(entry_index)))
}