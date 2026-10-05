pub open spec fn sbi_mpxy_get_channel_ids_spec(start_index: UInt32, error: SbiError, uvalue: UInt64, old_s: S, new_s: S) -> bool {
    let shmem_ok = MpxyShmemIsSetUp(old_s);
    let index_ok = MpxyStartIndexValid(old_s, start_index as int);
    let allowed = MpxyGetChannelIdsAllowed(old_s);
    let total = MpxyChannelCount(old_s) as int;
    (uvalue == 0)
    && (error == SBI_SUCCESS || error == SBI_ERR_INVALID_PARAM || error == SBI_ERR_NO_SHMEM || error == SBI_ERR_DENIED || error == SBI_ERR_FAILED)
    && (error == SBI_ERR_NO_SHMEM ==> !shmem_ok)
    && (error == SBI_ERR_INVALID_PARAM ==> !index_ok)
    && (error == SBI_ERR_DENIED ==> !allowed)
    && ((!shmem_ok || !index_ok || !allowed) ==> error != SBI_SUCCESS)
    && ((shmem_ok && index_ok && allowed) ==> (error == SBI_SUCCESS || error == SBI_ERR_FAILED))
    && (error == SBI_SUCCESS ==> {
        let remaining = MpxyShmemReadU32(new_s, 0x0) as int;
        let returned = MpxyShmemReadU32(new_s, 0x4) as int;
        returned >= 0
        && (start_index as int) + returned <= total
        && remaining == total - (start_index as int) - returned
        && (forall|i: int| 0 <= i < returned ==>
                #[trigger] MpxyShmemChannelIdSlot(new_s, i) == MpxyChannelIdAt(old_s, (start_index as int) + i))
    })
    && (MpxyChannelCount(new_s) == MpxyChannelCount(old_s))
}
