pub open spec fn sbi_nacl_sync_csr_spec(result: long, old_s: S, new_s: S) -> bool {
    (old_s.csr_num == ~0UL ==> result == 0 && AllImplementedHCsrsSynchronized(new_s))
    && ((old_s.csr_num & 0x300) == 0x200 && old_s.csr_num < 0x1000 ==> result == 0 && HCsrSynchronized(new_s, old_s.csr_num))
    && (result != 0 ==> false)
}