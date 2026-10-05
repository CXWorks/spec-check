pub open spec fn sbi_debug_set_shmem_spec(shmem_phys_lo: UInt64, shmem_phys_hi: UInt64, flags: UInt64, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (flags != 0 ==> result == SBI_ERR_INVALID_PARAM)
    && ((!(IsXlenAllOnes(old_s, shmem_phys_lo) && IsXlenAllOnes(old_s, shmem_phys_hi))
        && (shmem_phys_lo as int) % ((Xlen(old_s) as int) / 8) != 0)
        ==> result == SBI_ERR_INVALID_PARAM)
    && ((flags == 0
        && !(IsXlenAllOnes(old_s, shmem_phys_lo) && IsXlenAllOnes(old_s, shmem_phys_hi))
        && (shmem_phys_lo as int) % ((Xlen(old_s) as int) / 8) == 0
        && !SbiShmemAddressValid(old_s, SbiShmemPhysAddr(old_s, shmem_phys_lo, shmem_phys_hi),
            (DebugTrigMax(old_s) as int) * ((Xlen(old_s) as int) / 2)))
        ==> result == SBI_ERR_INVALID_ADDRESS)
    && ((flags == 0
        && IsXlenAllOnes(old_s, shmem_phys_lo) && IsXlenAllOnes(old_s, shmem_phys_hi))
        ==> ((result == SBI_SUCCESS || result == SBI_ERR_FAILED)
            && (result == SBI_SUCCESS ==> !HartDebugShmemEnabled(new_s, CurrentHart(old_s)))))
    && ((flags == 0
        && !(IsXlenAllOnes(old_s, shmem_phys_lo) && IsXlenAllOnes(old_s, shmem_phys_hi))
        && (shmem_phys_lo as int) % ((Xlen(old_s) as int) / 8) == 0
        && SbiShmemAddressValid(old_s, SbiShmemPhysAddr(old_s, shmem_phys_lo, shmem_phys_hi),
            (DebugTrigMax(old_s) as int) * ((Xlen(old_s) as int) / 2)))
        ==> ((result == SBI_SUCCESS || result == SBI_ERR_FAILED)
            && (result == SBI_SUCCESS ==>
                (HartDebugShmemEnabled(new_s, CurrentHart(old_s))
                && HartDebugShmemBase(new_s, CurrentHart(old_s)) == SbiShmemPhysAddr(old_s, shmem_phys_lo, shmem_phys_hi)))))
    && (result != SBI_SUCCESS ==>
        (HartDebugShmemEnabled(new_s, CurrentHart(old_s)) == HartDebugShmemEnabled(old_s, CurrentHart(old_s))
        && HartDebugShmemBase(new_s, CurrentHart(old_s)) == HartDebugShmemBase(old_s, CurrentHart(old_s))))
    && (forall|h: int| h != CurrentHart(old_s) ==>
        (HartDebugShmemEnabled(new_s, h) == HartDebugShmemEnabled(old_s, h)
        && HartDebugShmemBase(new_s, h) == HartDebugShmemBase(old_s, h)))
}
