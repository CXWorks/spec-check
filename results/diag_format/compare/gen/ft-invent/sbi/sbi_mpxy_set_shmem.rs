pub open spec fn sbi_mpxy_set_shmem_spec(shmem_phys_lo: UInt64, shmem_phys_hi: UInt64, flags: UInt64, old_s: S, new_s: S) -> bool {
  ((shmem_phys_lo != 0) && (shmem_phys_hi != 0) ==> (RealmAt(new_s, 0).shmem_phys_lo == shmem_phys_lo))
  && ((shmem_phys_lo != 0) && (shmem_phys_hi != 0) ==> (RealmAt(new_s, 0).shmem_phys_hi == shmem_phys_hi))
  && ((!(shmem_phys_lo != 0) && (shmem_phys_hi != 0)) ==> (RealmAt(new_s, 0).shmem_phys_lo == 0))
  && ((!(shmem_phys_lo != 0) && (shmem_phys_hi != 0)) ==> (RealmAt(new_s, 0).shmem_phys_hi == 0))
  && ((!(shmem_phys_lo != 0) && (shmem_phys_hi != 0)) ==> (RealmAt(new_s, 0).shmem_phys_lo == 0))
  && ((!(shmem_phys_lo != 0) && (shmem_phys_hi != 0)) ==> (RealmAt(new_s, 0).shmem_phys_hi == 0))
  && ((!(shmem_phys_lo != 0) && (shmem_phys_hi != 0)) ==> (RealmAt(new_s, 0).shmem_phys_lo == 0))
  && ((!(shmem_phys_lo != 0) && (shmem_phys_hi != 0)) ==> (RealmAt(new_s, 0).shmem_phys_hi == 0))
  && ((!(shmem_phys_lo != 0) && (shmem_phys_hi != 0)) ==> (RealmAt(new_s, 0).shmem_phys_lo == 0))
  && ((!(shmem_phys_lo != 0) && (shmem_phys_hi != 0)) ==> (RealmAt(new_s, 0).shmem_phys_hi == 0))
  && (RealmAt(new_s, 0).shmem_phys_lo == RealmAt(old_s, 0).shmem_phys_lo)
  && (RealmAt(new_s, 0).shmem_phys_hi == RealmAt(old_s, 0).shmem_phys_hi)
}