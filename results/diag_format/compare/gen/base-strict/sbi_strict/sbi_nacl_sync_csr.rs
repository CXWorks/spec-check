pub open spec fn sbi_nacl_sync_csr_spec(result: i64, old_s: S, new_s: S) -> bool {
    (!AllImplementedHCsrsSynchronized(old_s) ==> result != 0)
    && (AllImplementedHCsrsSynchronized(old_s) && (csr_num == AllOnes()) ==> AllImplementedHCsrsSynchronized(new_s))
    && (AllImplementedHCsrsSynchronized(old_s) && (((csr_num & 0x300) == 0x200) && (csr_num < 0x1000)) ==> HCsrSynchronized(csr_num, new_s))
}