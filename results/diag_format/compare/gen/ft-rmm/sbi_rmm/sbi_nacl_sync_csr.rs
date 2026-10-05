pub open spec fn sbi_nacl_sync_csr_spec(csr_num: unsigned long, error: long, old_s: S, new_s: S) -> bool {
  (csr_num == 18446744073709551615 ==> AllImplementedHCsrsSynchronized(new_s))
  && (((csr_num & 480) == 512) && (csr_num < 4096) ==> HCsrSynchronized(new_s, csr_num))
  && ((!((csr_num == 18446744073709551615)) &&
       !(((csr_num & 480) == 512) && (csr_num < 4096)))
    ==> true)
}