pub open spec fn ffa_notification_unbind_spec(
    result: Result<(), FfaErrorCode>,
    old_s: S,
    new_s: S,
    sender_receiver_ids: UInt32,
    bitmap_lo: UInt32,
    bitmap_hi: UInt32,
) -> bool {
    let sender: u16 = ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as u16;
    let receiver: u16 = (sender_receiver_ids & 0xFFFFu32) as u16;
    let bitmap: u64 = ((bitmap_hi as u64) << 32u64) | (bitmap_lo as u64);
    let invalid_parameters =
        !FfaPartitionIdIsRecognized(old_s, sender)
        || !FfaPartitionIdIsRecognized(old_s, receiver)
        || bitmap == 0u64;
    let not_supported = !FfaNotificationUnbindIsImplemented(old_s);
    let denied =
        FfaNotificationBoundToOtherSender(old_s, receiver, sender, bitmap)
        || FfaNotificationIsPending(old_s, receiver, bitmap)
        || !FfaCallerMayInvokeNotificationUnbind(old_s, sender, receiver);
    let aborted = FfaPartitionHasAborted(old_s, sender);
    let any_failure = invalid_parameters || not_supported || denied || aborted;
    (invalid_parameters ==> result.is_Err())
    && (not_supported ==> result.is_Err())
    && (denied ==> result.is_Err())
    && (aborted ==> result.is_Err())
    && (result == Err::<(), FfaErrorCode>(INVALID_PARAMETERS) ==> invalid_parameters)
    && (result == Err::<(), FfaErrorCode>(NOT_SUPPORTED) ==> not_supported)
    && (result == Err::<(), FfaErrorCode>(DENIED) ==> denied)
    && (result == Err::<(), FfaErrorCode>(ABORTED) ==> aborted)
    && (result.is_Err() ==> new_s == old_s)
    && (!any_failure ==> (
        result.is_Ok()
        && (forall |r: u16, i: int| 0 <= i < 64 ==> (
            if r == receiver && ((bitmap >> (i as u64)) & 1u64) == 1u64 {
                !FfaNotificationIsBoundTo(new_s, r, sender, i)
            } else {
                forall |snd: u16|
                    FfaNotificationIsBoundTo(new_s, r, snd, i)
                        == FfaNotificationIsBoundTo(old_s, r, snd, i)
            }
        ))
        && FfaStateUnchangedExceptNotificationBindings(old_s, new_s)
    ))
}
