pub open spec fn sbi_mpxy_set_shmem_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let shmem_phys_lo = old_s.cmd_input_shmem_phys_lo;
    let shmem_phys_hi = old_s.cmd_input_shmem_phys_hi;
    let flags = old_s.cmd_input_flags;
    let shmem_align = !(IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi)) && !IsAligned(shmem_phys_lo, 4096);
    let flags_rsvd = flags[XLEN-1 as usize..2 as usize] != 0;
    let shmem_set = !(IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi));
    let shmem_disable = IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi);
    let shmem_base_new = ShmemBase(new_s);
    let shmem_size_new = ShmemSize(new_s);
    let shmem_enabled_new = ShmemEnabled(new_s);
    let get_shmem_size = GetShmemSize();
    (!shmem_align ==> result.ret != 0)
    && (!flags_rsvd ==> result.ret == 0)
    && (shmem_set ==> result.ret == 0 && shmem_base_new == Concat(shmem_phys_hi, shmem_phys_lo) && shmem_size_new == get_shmem_size)
    && (shmem_disable ==> result.ret == 0 && !shmem_enabled_new)
}