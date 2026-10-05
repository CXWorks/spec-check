pub open spec fn sbi_mpxy_get_shmem_size_spec(sbiret_error: i64, sbiret_uvalue: u64, old_s: S, new_s: S) -> bool {
    (sbiret_error == SBI_SUCCESS)
    && (sbiret_uvalue as int == MpxyShmemSize(old_s) as int)
    && (sbiret_uvalue as int >= 4096)
    && ((sbiret_uvalue as int) % 4096 == 0)
    && (sbiret_uvalue as int >= MpxyMaxMsgDataMaxLenAllChannels(old_s) as int)
    && (new_s == old_s)
}
