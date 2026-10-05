pub open spec fn sbi_nacl_sync_csr_spec(csr_num: unsigned long, result: long, old_s: S, new_s: S) -> bool {
  (result == 0 && (csr_num == AllOnes())) ==> AllImplementedHCsrsSynchronized(new_s)
  && (result == 0 && ((csr_num & 0x300) == 0x200) && (csr_num < 0x1000)) ==> HCsrSynchronized(new_s, csr_num)
  && ((!(result == 0 && (csr_num == AllOnes())) &&
       !((result == 0) && ((csr_num & 0x300) == 0x200) && (csr_num < 0x1000))))
    ==> AllImplementedHCsrsSynchronized(new_s)
  && (result != 0)
    ==> AllImplementedHCsrsSynchronized(new_s)
}