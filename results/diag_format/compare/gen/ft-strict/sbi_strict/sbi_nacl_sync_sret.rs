pub open spec fn sbi_nacl_sync_sret_spec(old_s: S, new_s: S) -> bool {
  NaclSharedMemoryCsrsSynchronized(new_s)
  && NaclSharedMemoryHfencesSynchronized(new_s)
  && SretEmulated(new_s)
  && !ReturnsToCaller(new_s)
  && (NaclSharedMemoryCsrsSynchronized(old_s) &&
       NaclSharedMemoryHfencesSynchronized(old_s) &&
       SretEmulated(old_s) &&
       ReturnsToCaller(old_s))
  ==> (NaclSharedMemoryCsrsSynchronized(new_s) &&
       NaclSharedMemoryHfencesSynchronized(new_s) &&
       SretEmulated(new_s) &&
       !ReturnsToCaller(new_s))
}