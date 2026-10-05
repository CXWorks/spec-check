pub open spec fn sbi_mpxy_get_shmem_size_spec(error: SbiReturnCode, uvalue: UInt64, old_s: S, new_s: S) -> bool {
    (true ==> ResultEqual(error, SBI_SUCCESS))
    && (true ==> forall|h: Hart| MpxyShmemSize(h) == uvalue)
    && (true ==> uvalue >= 4096)
    && (true ==> uvalue % 4096 == 0)
    && (true ==> forall|c: MpxyChannel| uvalue >= MsgDataMaxLen(c))
    && (old_s == new_s)
}