pub open spec fn sbi_pmu_snapshot_set_shmem_spec(shmem_phys_lo: unsigned long, shmem_phys_hi: unsigned long, flags: unsigned long, result: SbiCommandReturnCode, value: long, old_s: S, new_s: S) -> bool {
  (!(IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi)) ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).base == ((shmem_phys_hi << 64) | shmem_phys_lo))
  && (!(IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi)) ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).size == 4096)
  && (!(IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi)) ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).enabled == true)
  && (IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi) ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).cleared == true)
  && (IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi) ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).enabled == false)
  && ((!(IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi)))
    ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).enabled == PmuSnapshotShmem(old_s, CurrentHart(old_s)).enabled)
  && (result == SBI_SUCCESS
    ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).base == PmuSnapshotShmem(old_s, CurrentHart(old_s)).base)
  && (result != SBI_SUCCESS
    ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).base == PmuSnapshotShmem(old_s, CurrentHart(old_s)).base)
  && (result == SBI_SUCCESS
    ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).size == PmuSnapshotShmem(old_s, CurrentHart(old_s)).size)
  && (result != SBI_SUCCESS
    ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).size == PmuSnapshotShmem(old_s, CurrentHart(old_s)).size)
  && (result == SBI_SUCCESS
    ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).enabled == PmuSnapshotShmem(old_s, CurrentHart(old_s)).enabled)
  && (result != SBI_SUCCESS
    ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).enabled == PmuSnapshotShmem(old_s, CurrentHart(old_s)).enabled)
  && (result == SBI_SUCCESS
    ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).cleared == PmuSnapshotShmem(old_s, CurrentHart(old_s)).cleared)
  && (result != SBI_SUCCESS
    ==> PmuSnapshotShmem(new_s, CurrentHart(new_s)).cleared == PmuSnapshotShmem(old_s, CurrentHart(old_s)).cleared)
}