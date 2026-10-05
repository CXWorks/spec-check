pub open spec fn sbi_mpxy_get_shmem_size_spec(error: SbiErrorCode, uvalue: UInt, old_s: S, new_s: S) -> bool {
    (ResultEqual(error, SBI_SUCCESS) ==> uvalue == MpxyShmemSize())
    && (ResultEqual(error, SBI_SUCCESS) ==> forall hart: MpxyShmemSizeOnHart(hart) == uvalue)
    && (ResultEqual(error, SBI_SUCCESS) ==> uvalue >= 4096)
    && (ResultEqual(error, SBI_SUCCESS) ==> (uvalue % 4096) == 0)
    && (ResultEqual(error, SBI_SUCCESS) ==> forall chan in MpxyChannels(): uvalue >= MsgDataMaxLen(chan))
    && (ResultEqual(error, SBI_SUCCESS) ==> old_s == new_s)
}