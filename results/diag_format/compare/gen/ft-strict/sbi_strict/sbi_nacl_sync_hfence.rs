pub open spec fn sbi_nacl_sync_hfence_spec(entry_index: unsigned long, result: sbiret.error, old_s: S, new_s: S) -> bool {
  (!NaclFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_HFENCE) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (!IsAllOnes(old_s, entry_index) && entry_index >= (3840 / XLEN as int) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!NaclSharedMemoryAvailable(old_s) ==> ResultEqual(result, SBI_ERR_NO_SHMEM))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS && IsAllOnes(old_s, entry_index) ==> AllNestedHfenceEntriesSynchronized(new_s))
  && (result == SBI_SUCCESS && entry_index < (3840 / XLEN as int) ==> NestedHfenceEntrySynchronized(new_s, entry_index))
  && ((NaclFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_HFENCE) &&
       (IsAllOnes(old_s, entry_index) || !(entry_index >= (3840 / XLEN as int))) &&
       NaclSharedMemoryAvailable(old_s))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> AllNestedHfenceEntriesSynchronized(new_s))
  && (result != SBI_SUCCESS
    ==> NestedHfenceEntrySynchronized(new_s, entry_index))
}