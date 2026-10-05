pub open spec fn sbi_nacl_sync_hfence_spec(entry_index: UInt, error: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (!NaclFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_HFENCE) ==> ResultEqual(error, SBI_ERR_NOT_SUPPORTED))
  && (!IsAllOnes(old_s, entry_index) && entry_index >= (3840 / XLEN as int) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (!NaclSharedMemoryAvailable(old_s) ==> ResultEqual(error, SBI_ERR_NO_SHMEM))
  && (ResultEqual(error, SBI_SUCCESS))
  && (IsAllOnes(old_s, entry_index) ==> All nested HFENCE entries are synchronized as described in Section 15.2)
  && (entry_index < (3840 / XLEN as int) ==> The nested HFENCE entry at entry_index is synchronized as described in Section 15.2)
  && ((NaclFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_HFENCE) &&
       !(IsAllOnes(old_s, entry_index) && entry_index >= (3840 / XLEN as int)) &&
       NaclSharedMemoryAvailable(old_s))
    ==> ResultEqual(error, SBI_SUCCESS))
}