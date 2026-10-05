pub open spec fn sbi_nacl_sync_sret_spec(old_s: S, new_s: S) -> bool {
  (NaclSharedMemCsrsSynchronized(new_s))
  && (NaclSharedMemHfencesSynchronized(new_s))
  && (SretEmulated(new_s))
}