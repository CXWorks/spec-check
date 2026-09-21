pub open spec fn sbi_nacl_set_shmem_spec(shmem_phys_lo: UInt64, shmem_phys_hi: UInt64, flags: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
  (result.error != SBI_SUCCESS && result.error != SBI_ERR_INVALID_PARAM && result.error != SBI_ERR_INVALID_ADDRESS && result.error != SBI_ERR_FAILED ==> flags == 0)
  && (result.error == SBI_ERR_INVALID_PARAM || result.error == SBI_ERR_INVALID_ADDRESS || result.error == SBI_ERR_FAILED ==> flags == 0)
  && ((!(shmem_phys_lo == -1) && !(shmem_phys_hi == -1)) ==> (shmem_phys_lo % 4096 == 0))
  && (result.error == SBI_SUCCESS || result.error == SBI_ERR_INVALID_PARAM || result.error == SBI_ERR_INVALID_ADDRESS || result.error == SBI_ERR_FAILED ==> flags == 0)
}