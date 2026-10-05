pub open spec fn ffa_msg_send_direct_resp_spec(function_id: UInt32, src_dst_ids: UInt32, flags: UInt32, result: FfaReturn, old_s: S, new_s: S) -> bool {
    let src_id: UInt32 = (src_dst_ids >> 16u32) & 0xFFFFu32;
    let dst_id: UInt32 = src_dst_ids & 0xFFFFu32;
    let msg_type: UInt32 = (flags >> 31u32) & 1u32;
    let reserved_bits: UInt32 = (flags >> 8u32) & 0x7F_FFFFu32;
    let low_bits: UInt32 = flags & 0xFFu32;
    let not_supported: bool = !FfaFunctionImplemented(old_s, function_id);
    let invalid_endpoint: bool = !IsValidEndpointId(old_s, src_id) || !IsValidEndpointId(old_s, dst_id);
    let invalid_flags: bool = reserved_bits != 0u32
        || (msg_type == 0u32 && low_bits != 0u32)
        || (msg_type == 1u32 && !IsValidFrameworkMsgType(low_bits));
    let invalid_params: bool = invalid_endpoint || invalid_flags;
    let denied: bool = !CalleeCanHandleRequest(old_s, src_id, dst_id)
        || !CallerAllowedToInvoke(old_s, src_id, function_id)
        || !ReceiverSupportsDirectResp(old_s, dst_id);
    let aborted: bool = ReceiverAborted(old_s, new_s, dst_id);
    let valid_fid: bool = function_id == 0x8400_0070u32
        || (function_id == 0xC400_0070u32 && PendingDirectReqIsSmc64(old_s, src_id, dst_id));
    (not_supported ==> (FfaErrorEqual(result, NOT_SUPPORTED) && new_s == old_s))
    && ((!not_supported && invalid_params) ==> (FfaErrorEqual(result, INVALID_PARAMETERS) && new_s == old_s))
    && ((!not_supported && !invalid_params && denied) ==> (FfaErrorEqual(result, DENIED) && new_s == old_s))
    && ((!not_supported && !invalid_params && !denied && aborted) ==> FfaErrorEqual(result, ABORTED))
    && ((!not_supported && !invalid_params && !denied && !aborted && valid_fid) ==> (
        DirectRespDelivered(old_s, new_s, src_id, dst_id, msg_type, low_bits)
        && FfaMsgWaitSuccess(result, old_s, new_s, src_id)
    ))
}
