pub open spec fn sbi_nacl_sync_hfence_spec(entry_index: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
    (!NaclFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_HFENCE) ==> (
        (result.error == SBI_ERR_NOT_SUPPORTED
            || ((entry_index != 0xFFFF_FFFF_FFFF_FFFFu64 && (entry_index as int) >= 3840 / 64) && result.error == SBI_ERR_INVALID_PARAM)
            || (!NaclShmemAvailable(old_s) && result.error == SBI_ERR_NO_SHMEM))
        && new_s == old_s
    ))
    && ((entry_index != 0xFFFF_FFFF_FFFF_FFFFu64 && (entry_index as int) >= 3840 / 64) ==> (
        (result.error == SBI_ERR_INVALID_PARAM
            || (!NaclFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_HFENCE) && result.error == SBI_ERR_NOT_SUPPORTED)
            || (!NaclShmemAvailable(old_s) && result.error == SBI_ERR_NO_SHMEM))
        && new_s == old_s
    ))
    && (!NaclShmemAvailable(old_s) ==> (
        (result.error == SBI_ERR_NO_SHMEM
            || (!NaclFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_HFENCE) && result.error == SBI_ERR_NOT_SUPPORTED)
            || ((entry_index != 0xFFFF_FFFF_FFFF_FFFFu64 && (entry_index as int) >= 3840 / 64) && result.error == SBI_ERR_INVALID_PARAM))
        && new_s == old_s
    ))
    && ((NaclFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_HFENCE)
        && !(entry_index != 0xFFFF_FFFF_FFFF_FFFFu64 && (entry_index as int) >= 3840 / 64)
        && NaclShmemAvailable(old_s)) ==> (
        result.error == SBI_SUCCESS
        && (entry_index == 0xFFFF_FFFF_FFFF_FFFFu64 ==> NaclAllHfenceEntriesSynchronized(old_s, new_s))
        && (entry_index != 0xFFFF_FFFF_FFFF_FFFFu64 ==> NaclHfenceEntrySynchronized(old_s, new_s, entry_index))
    ))
}
