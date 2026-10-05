pub open spec fn sbi_nacl_set_shmem_spec(shmem_phys_lo: UInt64, shmem_phys_hi: UInt64, flags: UInt64, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (flags != 0 ==> result == SBI_ERR_INVALID_PARAM)
    && ((flags == 0
        && !(shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64 && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64)
        && (shmem_phys_lo as int) % 4096 != 0) ==> result == SBI_ERR_INVALID_PARAM)
    && ((flags == 0
        && !(shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64 && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64)
        && (shmem_phys_lo as int) % 4096 == 0
        && !SbiShmemSatisfiesRequirements(old_s, (shmem_phys_hi as int) * 0x1_0000_0000_0000_0000 + (shmem_phys_lo as int), 4096 + 64 * 128)) ==> result == SBI_ERR_INVALID_ADDRESS)
    && ((flags == 0
        && !(shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64 && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64)
        && (shmem_phys_lo as int) % 4096 == 0
        && SbiShmemSatisfiesRequirements(old_s, (shmem_phys_hi as int) * 0x1_0000_0000_0000_0000 + (shmem_phys_lo as int), 4096 + 64 * 128)) ==> (result == SBI_SUCCESS || result == SBI_ERR_FAILED))
    && ((flags == 0
        && shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64
        && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64) ==> (result == SBI_SUCCESS || result == SBI_ERR_FAILED))
    && ((result == SBI_SUCCESS
        && !(shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64 && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64)) ==>
        (NaclShmemEnabled(new_s, CallingHart(old_s))
        && NaclShmemBase(new_s, CallingHart(old_s)) == (shmem_phys_hi as int) * 0x1_0000_0000_0000_0000 + (shmem_phys_lo as int)))
    && ((result == SBI_SUCCESS
        && shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64
        && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64) ==>
        !NaclShmemEnabled(new_s, CallingHart(old_s)))
    && (result != SBI_SUCCESS ==>
        (NaclShmemEnabled(new_s, CallingHart(old_s)) == NaclShmemEnabled(old_s, CallingHart(old_s))
        && NaclShmemBase(new_s, CallingHart(old_s)) == NaclShmemBase(old_s, CallingHart(old_s))))
}
