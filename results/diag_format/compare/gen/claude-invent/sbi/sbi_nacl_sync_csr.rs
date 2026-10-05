pub open spec fn sbi_nacl_sync_csr_spec(result: SbiRet, csr_num: UInt64, old_s: S, new_s: S) -> bool {
    (!NaclFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_CSR) ==> result.error != SBI_SUCCESS)
    && (!NaclShmemAvailable(old_s) ==> result.error != SBI_SUCCESS)
    && ((csr_num != 0xFFFF_FFFF_FFFF_FFFFu64
            && ((csr_num & 0x300u64) != 0x200u64
                || (csr_num as int) >= 0x1000
                || !IsHCsrImplemented(old_s, csr_num)))
        ==> result.error != SBI_SUCCESS)
    && (result.error == SBI_ERR_NOT_SUPPORTED ==> !NaclFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_CSR))
    && (result.error == SBI_ERR_NO_SHMEM ==> !NaclShmemAvailable(old_s))
    && (result.error == SBI_ERR_INVALID_PARAM ==>
            (csr_num != 0xFFFF_FFFF_FFFF_FFFFu64
                && ((csr_num & 0x300u64) != 0x200u64
                    || (csr_num as int) >= 0x1000
                    || !IsHCsrImplemented(old_s, csr_num))))
    && (result.error != SBI_SUCCESS ==> new_s == old_s)
    && ((NaclFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_CSR)
            && NaclShmemAvailable(old_s)
            && csr_num == 0xFFFF_FFFF_FFFF_FFFFu64)
        ==> (result.error == SBI_SUCCESS && NaclAllHCsrsSynchronized(old_s, new_s)))
    && ((NaclFeatureAvailable(old_s, SBI_NACL_FEAT_SYNC_CSR)
            && NaclShmemAvailable(old_s)
            && csr_num != 0xFFFF_FFFF_FFFF_FFFFu64
            && (csr_num & 0x300u64) == 0x200u64
            && (csr_num as int) < 0x1000
            && IsHCsrImplemented(old_s, csr_num))
        ==> (result.error == SBI_SUCCESS && NaclHCsrSynchronized(old_s, new_s, csr_num)))
}
