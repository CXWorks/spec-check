pub open spec fn sbi_nacl_sync_sret_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (result != 0 ==> false)
    && (NaclSharedMemoryCsrsSynchronized() ==> NaclSharedMemoryCsrsSynchronized(new_s))
    && (NaclSharedMemoryHfencesSynchronized() ==> NaclSharedMemoryHfencesSynchronized(new_s))
    && (SretEmulated() ==> SretEmulated(new_s))
    && (!ReturnsToCaller() ==> !ReturnsToCaller(new_s))
}