pub open spec fn ffa_notification_set2_spec(result: FfaReturnCode, sender_receiver_ids: UInt32, flags: UInt64, bitmap0: UInt64, bitmap1: UInt64, bitmap2: UInt64, bitmap3: UInt64, bitmap4: UInt64, bitmap5: UInt64, old_s: S, new_s: S) -> bool {
    let sender: int = ((sender_receiver_ids as int) / 0x10000) % 0x10000;
    let receiver: int = (sender_receiver_ids as int) % 0x10000;
    let per_vcpu: bool = (flags as int) % 2 == 1;
    let receiver_vcpu_id: int = (flags as int) / 0x10000;
    let reserved_flags_set: bool = ((flags as int) / 4) % 0x4000 != 0;
    let bitmap_empty: bool = bitmap0 == 0 && bitmap1 == 0 && bitmap2 == 0 && bitmap3 == 0 && bitmap4 == 0 && bitmap5 == 0;
    let fail_not_supported: bool = !FfaNotificationSet2ImplementedAtInstance(old_s);
    let fail_invalid_parameters: bool =
        !IsRecognizedPartitionId(old_s, sender)
        || !IsRecognizedPartitionId(old_s, receiver)
        || reserved_flags_set
        || (!per_vcpu && receiver_vcpu_id != 0)
        || (!per_vcpu && NotificationBitmapContainsPerVcpuNotification(old_s, receiver, bitmap0, bitmap1, bitmap2, bitmap3, bitmap4, bitmap5))
        || (per_vcpu && NotificationBitmapContainsGlobalNotification(old_s, receiver, bitmap0, bitmap1, bitmap2, bitmap3, bitmap4, bitmap5))
        || (per_vcpu && !PerVcpuNotificationsSupported(old_s))
        || NotificationBitmapExceedsSupportedNotifications(old_s, bitmap0, bitmap1, bitmap2, bitmap3, bitmap4, bitmap5)
        || bitmap_empty;
    let fail_denied: bool =
        !SenderPermittedToSignalNotifications(old_s, sender, receiver, bitmap0, bitmap1, bitmap2, bitmap3, bitmap4, bitmap5)
        || !ReceiverSupportsNotifications(old_s, receiver);
    let fail_aborted: bool = ReceiverPartitionAborted(old_s, receiver);
    let any_failure: bool = fail_not_supported || fail_invalid_parameters || fail_denied || fail_aborted;
    (fail_not_supported ==> result != FFA_SUCCESS)
    && (fail_invalid_parameters ==> result != FFA_SUCCESS)
    && (fail_denied ==> result != FFA_SUCCESS)
    && (fail_aborted ==> result != FFA_SUCCESS)
    && (result == NOT_SUPPORTED ==> fail_not_supported)
    && (result == INVALID_PARAMETERS ==> fail_invalid_parameters)
    && (result == DENIED ==> fail_denied)
    && (result == ABORTED ==> fail_aborted)
    && (result != FFA_SUCCESS ==> new_s == old_s)
    && (!any_failure ==> (
        result == FFA_SUCCESS
        && NotificationsSignaledToReceiver(old_s, new_s, sender, receiver, per_vcpu, receiver_vcpu_id, flags, bitmap0, bitmap1, bitmap2, bitmap3, bitmap4, bitmap5)
    ))
}
