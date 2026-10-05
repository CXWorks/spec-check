pub open spec fn ffa_notification_unbind2_spec(
    result: FfaReturnCode,
    function_id: UInt32,
    sender_receiver_ids: UInt32,
    reserved: UInt64,
    bitmap0: UInt64,
    bitmap1: UInt64,
    bitmap2: UInt64,
    bitmap3: UInt64,
    bitmap4: UInt64,
    bitmap5: UInt64,
    old_s: S,
    new_s: S,
) -> bool {
    let sender: int = ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as int;
    let receiver: int = (sender_receiver_ids & 0xFFFFu32) as int;
    let bit_set = |i: int| {
        let w: u64 = if i / 64 == 0 { bitmap0 } else if i / 64 == 1 { bitmap1 } else if i / 64 == 2 { bitmap2 } else if i / 64 == 3 { bitmap3 } else if i / 64 == 4 { bitmap4 } else { bitmap5 };
        0 <= i && i < 384 && ((w >> ((i % 64) as u64)) & 1u64) == 1u64
    };
    let fail_not_supported: bool = !FfaFunctionImplemented(old_s, 0xC4000095u32);
    let fail_invalid: bool =
        !FfaIsValidPartitionId(old_s, sender)
        || !FfaIsValidPartitionId(old_s, receiver)
        || (bitmap0 == 0 && bitmap1 == 0 && bitmap2 == 0 && bitmap3 == 0 && bitmap4 == 0 && bitmap5 == 0)
        || (exists|i: int| 0 <= i && i < 384 && bit_set(i) && i >= FfaNumSupportedNotifications(old_s) as int);
    let fail_denied: bool =
        !FfaCallerAllowedNotificationUnbind2(old_s, sender, receiver)
        || (exists|i: int| 0 <= i && i < 384 && bit_set(i)
            && ((FfaNotificationIsBound(old_s, receiver, i) && FfaNotificationBoundSender(old_s, receiver, i) != sender)
                || FfaNotificationIsPending(old_s, receiver, i)));
    let fail_aborted: bool = FfaPartitionAborted(old_s, sender);
    function_id == 0xC4000095u32 ==> (
        (fail_not_supported ==> result == NOT_SUPPORTED)
        && ((fail_invalid || fail_denied || fail_aborted) ==> result != FFA_SUCCESS)
        && (result == NOT_SUPPORTED ==> fail_not_supported)
        && (result == INVALID_PARAMETERS ==> fail_invalid)
        && (result == DENIED ==> fail_denied)
        && (result == ABORTED ==> fail_aborted)
        && (result != FFA_SUCCESS ==> new_s == old_s)
        && ((!fail_not_supported && !fail_invalid && !fail_denied && !fail_aborted) ==> (
            result == FFA_SUCCESS
            && (forall|i: int| 0 <= i && i < 384 && bit_set(i) ==>
                !FfaNotificationIsBound(new_s, receiver, i))
            && (forall|r: int, i: int| 0 <= i && i < 384 && !(r == receiver && bit_set(i)) ==>
                FfaNotificationIsBound(new_s, r, i) == FfaNotificationIsBound(old_s, r, i)
                && FfaNotificationBoundSender(new_s, r, i) == FfaNotificationBoundSender(old_s, r, i))
            && (forall|r: int, i: int| 0 <= i && i < 384 ==>
                FfaNotificationIsPending(new_s, r, i) == FfaNotificationIsPending(old_s, r, i))
        ))
    )
}
