pub open spec fn sbi_debug_set_shmem_spec(shmem_phys_lo: UInt64, shmem_phys_hi: UInt64, flags: UInt64, result: SBI_SBI_RETURN_CODE, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_PARAM ==> flags == 0)
  && (result == SBI_ERR_INVALID_PARAM ==> (shmem_phys_lo % ((64 / 8) as int)) == 0)
  && ((!(result == SBI_ERR_INVALID_PARAM) &&
       result == SBI_SUCCESS)
    ==> true)
}