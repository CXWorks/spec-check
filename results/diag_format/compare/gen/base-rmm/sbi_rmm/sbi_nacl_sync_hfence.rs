pub open spec fn sbi_nacl_sync_hfence_spec(error: SbiErrorCode, entry_index: UInt, old_s: S, new_s: S) -> bool {
    (!NaclFeatureAvailable(SBI_NACL_FEAT_SYNC_HFENCE) ==> ResultEqual(error, SBI_ERR_NOT_SUPPORTED))
    && (!IsAllOnes(entry_index) && entry_index >= (3840 / XLEN as UInt) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (!NaclSharedMemoryAvailable() ==> ResultEqual(error, SBI_ERR_NO_SHMEM))
    && (ResultEqual(error, SBI_SUCCESS) ==> (IsAllOnes(entry_index) ==> AllNestedHfenceEntriesSynchronized(old_s, new_s) || entry_index < (3840 / XLEN as UInt) ==> NestedHfenceEntrySynchronized(old_s, new_s, entry_index)))
}