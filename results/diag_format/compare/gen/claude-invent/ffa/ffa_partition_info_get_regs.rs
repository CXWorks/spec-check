pub open spec fn ffa_partition_info_get_regs_spec(uuid_lo: u64, uuid_hi: u64, start_index_and_tag: u64, ret_w0: u32, ret_err: i32, ret_regs: Seq<u64>, old_s: S, new_s: S) -> bool {
    let start_index = (start_index_and_tag & 0xFFFFu64) as int;
    let in_tag = ((start_index_and_tag >> 16u64) & 0xFFFFu64) as int;
    let is_nil_uuid = uuid_lo == 0u64 && uuid_hi == 0u64;
    let impl_ok = IsFfaPartitionInfoGetRegsImplemented(old_s);
    let uuid_ok = IsValidPartitionUuid(old_s, uuid_lo, uuid_hi);
    let count = PartitionInfoCount(old_s, uuid_lo, uuid_hi) as int;
    let index_ok = start_index < count;
    let tag_ok = start_index == 0 || in_tag == PartitionInfoTag(old_s, uuid_lo, uuid_hi) as int;
    let denied = IsCalleeNotInStateToHandleRequest(old_s);
    let not_ready = IsCalleeNotReady(old_s);
    let md = ret_regs[2];
    let last_index = (md & 0xFFFFu64) as int;
    let current_index = ((md >> 16u64) & 0xFFFFu64) as int;
    let ret_tag = ((md >> 32u64) & 0xFFFFu64) as int;
    let desc_size = (md >> 48u64) as int;
    let n_entries = current_index - start_index + 1;
    (!impl_ok ==> (ret_w0 == FFA_ERROR && ret_err == NOT_SUPPORTED))
    && ((impl_ok && (!uuid_ok || !index_ok)) ==> ret_w0 == FFA_ERROR)
    && ((impl_ok && !tag_ok) ==> ret_w0 == FFA_ERROR)
    && ((impl_ok && (denied || not_ready)) ==> ret_w0 == FFA_ERROR)
    && (ret_w0 == FFA_ERROR ==> (
        (ret_err == NOT_SUPPORTED && !impl_ok)
        || (ret_err == INVALID_PARAMETERS && (!uuid_ok || !index_ok))
        || (ret_err == RETRY && !tag_ok)
        || (ret_err == DENIED && denied)
        || (ret_err == NOT_READY && not_ready)))
    && (ret_w0 == FFA_ERROR ==> new_s == old_s)
    && ((impl_ok && uuid_ok && index_ok && tag_ok && !denied && !not_ready) ==> (
        ret_w0 == FFA_SUCCESS64
        && ret_regs.len() == 18
        && last_index == count - 1
        && current_index >= start_index
        && current_index <= last_index
        && current_index <= start_index + 1
        && (current_index < last_index ==> current_index == start_index + 1)
        && ret_tag == PartitionInfoTag(old_s, uuid_lo, uuid_hi) as int
        && desc_size == 48
        && (forall|k: int| 0 <= k < n_entries ==> {
            let base = 3 + 6 * k;
            let idx = start_index + k;
            (ret_regs[base] & 0xFFFFu64) as int == PartitionId(old_s, uuid_lo, uuid_hi, idx) as int
            && ((ret_regs[base] >> 16u64) & 0xFFFFu64) as int == PartitionExecCtxCount(old_s, uuid_lo, uuid_hi, idx) as int
            && (ret_regs[base] >> 32u64) as int == PartitionProperties(old_s, uuid_lo, uuid_hi, idx) as int
            && ret_regs[base + 1] == (if is_nil_uuid { PartitionProtocolUuidLo(old_s, uuid_lo, uuid_hi, idx) } else { 0u64 })
            && ret_regs[base + 2] == (if is_nil_uuid { PartitionProtocolUuidHi(old_s, uuid_lo, uuid_hi, idx) } else { 0u64 })
            && ret_regs[base + 3] == (if PartitionHasImageUuid(old_s, uuid_lo, uuid_hi, idx) { PartitionImageUuidLo(old_s, uuid_lo, uuid_hi, idx) } else { 0u64 })
            && ret_regs[base + 4] == (if PartitionHasImageUuid(old_s, uuid_lo, uuid_hi, idx) { PartitionImageUuidHi(old_s, uuid_lo, uuid_hi, idx) } else { 0u64 })
            && (ret_regs[base + 5] >> 31u64) == 0u64
            && ((ret_regs[base + 5] >> 16u64) & 0x7FFFu64) as int == PartitionFfaMajorVersion(old_s, uuid_lo, uuid_hi, idx) as int
            && (ret_regs[base + 5] & 0xFFFFu64) as int == PartitionFfaMinorVersion(old_s, uuid_lo, uuid_hi, idx) as int
        })
        && (forall|r: int| 3 + 6 * n_entries <= r && r <= 17 ==> ret_regs[r] == 0u64)
        && new_s == old_s
    ))
}
