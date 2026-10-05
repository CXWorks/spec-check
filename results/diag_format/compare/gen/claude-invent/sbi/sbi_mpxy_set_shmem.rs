pub open spec fn sbi_mpxy_set_shmem_spec(result: SbiRet, old_s: S, new_s: S, hartid: u64, shmem_phys_lo: u64, shmem_phys_hi: u64, flags: u64) -> bool {
    ((flags >> 2u64) != 0u64 ==> result.error == SBI_ERR_INVALID_PARAM)
    && ((!(shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64 && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64) && (shmem_phys_lo % 4096u64) != 0u64) ==> result.error == SBI_ERR_INVALID_ADDRESS)
    && (result.error != SBI_SUCCESS ==> (
        MpxyShmemEnabled(new_s, hartid) == MpxyShmemEnabled(old_s, hartid)
        && MpxyShmemBase(new_s, hartid) == MpxyShmemBase(old_s, hartid)
        && MpxyShmemSize(new_s) == MpxyShmemSize(old_s)
    ))
    && (((flags >> 2u64) == 0u64
        && IsValidMpxyShmemMode(old_s, flags & 3u64)
        && shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64
        && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64) ==> (
        result.error == SBI_SUCCESS
        && !MpxyShmemEnabled(new_s, hartid)
        && MpxyShmemSize(new_s) == MpxyShmemSize(old_s)
    ))
    && (((flags >> 2u64) == 0u64
        && IsValidMpxyShmemMode(old_s, flags & 3u64)
        && !(shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64 && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64)
        && (shmem_phys_lo % 4096u64) == 0u64
        && MpxyShmemAccessible(old_s, (shmem_phys_hi as int) * 0x1_0000_0000_0000_0000int + (shmem_phys_lo as int), MpxyShmemSize(old_s) as int)) ==> (
        result.error == SBI_SUCCESS
        && MpxyShmemEnabled(new_s, hartid)
        && MpxyShmemBase(new_s, hartid) as int == (shmem_phys_hi as int) * 0x1_0000_0000_0000_0000int + (shmem_phys_lo as int)
        && MpxyShmemSize(new_s) == MpxyShmemSize(old_s)
    ))
}
