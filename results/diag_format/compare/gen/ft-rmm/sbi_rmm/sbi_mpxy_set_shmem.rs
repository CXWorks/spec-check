pub open spec fn sbi_mpxy_set_shmem_spec(shmem_phys_lo: unsigned long, shmem_phys_hi: unsigned long, flags: unsigned long, result: struct sbiret, old_s: S, new_s: S) -> bool {
  (!((IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi)) && !IsAligned(old_s, shmem_phys_lo, 4096)) ==> result.code == 0)
  && ((flags & ((1) << (64 - 2)) - 1) != 0 ==> result.code == 0)
  && (result.code == 0 && !((IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi))) ==> ShmemBase(new_s, CallingHart(new_s)) == Concat(new_s, shmem_phys_hi, shmem_phys_lo))
  && (result.code == 0 && !((IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi))) ==> ShmemSize(new_s, CallingHart(new_s)) == GetShmemSize(new_s))
  && (result.code == 0 && IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi) ==> !ShmemEnabled(new_s, CallingHart(new_s)))
  && ((result.code != 0) ==> ShmemBase(new_s, CallingHart(new_s)) == ShmemBase(old_s, CallingHart(old_s)))
  && ((result.code != 0) ==> ShmemSize(new_s, CallingHart(new_s)) == ShmemSize(old_s, CallingHart(old_s)))
  && ((result.code != 0) ==> ShmemEnabled(new_s, CallingHart(new_s)) == ShmemEnabled(old_s, CallingHart(old_s)))
}