pub open spec fn ffa_notification_get2_spec(
    result: FfaReturnCode,
    old_s: FfaState,
    new_s: FfaState,
    function_id: u32,
    receiver_id: u64,
    flags: u64,
    sp_notification_bitmask: u64,
    vm_notification_bitmask: u64,
    spmc_notification_bitmask: u64,
    hypervisor_notification_bitmask: u64,
) -> bool {
    (function_id == 0xC4000097)
    && (result == FFA_SUCCESS || result == FFA_ERROR)
    && (result == FFA_ERROR ==> (
        (result == FFA_ERROR_INVALID_PARAMETERS)
        || (result == FFA_ERROR_DENIED)
        || (result == FFA_ERROR_NOT_SUPPORTED)
    ))
    && (result == FFA_SUCCESS ==> (
        (flags & 0x1) == 0 || (flags & 0x1) == 1)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x2) == 0 || (flags & 0x2) == 2)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x4) == 0 || (flags & 0x4) == 4)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x8) == 0 || (flags & 0x8) == 8)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x30) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x40) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x80) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x100) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x200) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x400) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x800) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x1000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x2000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x4000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x8000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x10000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x20000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x40000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x80000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x100000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x200000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x400000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x800000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x1000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x2000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x4000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x8000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x10000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x20000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x40000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x80000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x100000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x200000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x400000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x800000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x1000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x2000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x4000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x8000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x10000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x20000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x40000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x80000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x100000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x200000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x400000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x800000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x1000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x2000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x4000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x8000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x10000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x20000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x40000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x80000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x100000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x200000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x400000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x800000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x1000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x2000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x4000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x8000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x10000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x20000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x40000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x80000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x100000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x200000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x400000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x800000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x1000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x2000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x4000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x8000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x10000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x20000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x40000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x80000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x100000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x200000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x400000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x800000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x1000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x2000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x4000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x8000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x10000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x20000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x40000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x80000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x100000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x200000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x400000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x800000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x1000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x2000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x4000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x8000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x10000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x20000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x40000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x80000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x100000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x200000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x400000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x800000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x1000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x2000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x4000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x8000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x10000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x20000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x40000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x80000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x100000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x200000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x400000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x800000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x1000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x2000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x4000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x8000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x10000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x20000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x40000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x80000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x100000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x200000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x400000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x800000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x1000000000000000000000000000000000) == 0)
    )
    && (result == FFA_SUCCESS ==> (
        (flags & 0x2000000000000000000000000000000000) == 0)