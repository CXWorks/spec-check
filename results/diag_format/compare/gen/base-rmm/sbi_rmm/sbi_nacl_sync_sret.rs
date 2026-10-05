pub open spec fn sbi_nacl_sync_sret_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (result == SBI_NACL_SYNC_SRET_ERROR_INPUT ==> !old_s.NaclSharedMemCsrsSynchronized())
    && (result == SBI_NACL_SYNC_SRET_ERROR_STATE ==> !old_s.NaclSharedMemHfencesSynchronized())
    && (result == SBI_NACL_SYNC_SRET_ERROR_UNKNOWN ==> !old_s.SretEmulated())
    && (result == SBI_SUCCESS ==> old_s.NaclSharedMemCsrsSynchronized() && old_s.NaclSharedMemHfencesSynchronized() && old_s.SretEmulated())
    && (result == SBI_SUCCESS ==> new_s.NaclSharedMemCsrsSynchronized() && new_s.NaclSharedMemHfencesSynchronized() && new_s.SretEmulated())
    && (result == SBI_SUCCESS ==> FunctionDoesNotReturn())
}