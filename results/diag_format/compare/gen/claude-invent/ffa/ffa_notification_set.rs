pub open spec fn ffa_notification_set_spec(instance: FfaInstance, conduit: FfaConduit, w1: UInt32, w2: UInt32, w3: UInt32, w4: UInt32, w5: UInt32, w6: UInt32, w7: UInt32, result: FfaStatus, old_s: S, new_s: S) -> bool {
    let sender: UInt32 = (w1 >> 16u32) & 0xFFFFu32;
    let receiver: UInt32 = w1 & 0xFFFFu32;
    let per_vcpu: bool = (w2 & 1u32) == 1u32;
    let delay_sri: UInt32 = (w2 >> 1u32) & 1u32;
    let reserved_flags: UInt32 = (w2 >> 2u32) & 0x3FFFu32;
    let vcpu_id: UInt32 = (w2 >> 16u32) & 0xFFFFu32;
    let bitmap: UInt64 = ((w4 as u64) << 32u64) | (w3 as u64);
    let conduit_valid: bool =
        (instance == FFA_INSTANCE_NS_VIRTUAL && (conduit == FFA_CONDUIT_SMC || conduit == FFA_CONDUIT_HVC))
        || (instance == FFA_INSTANCE_S_VIRTUAL && (conduit == FFA_CONDUIT_SMC || conduit == FFA_CONDUIT_HVC || conduit == FFA_CONDUIT_SVC))
        || (instance == FFA_INSTANCE_NS_PHYSICAL && conduit == FFA_CONDUIT_SMC)
        || (instance == FFA_INSTANCE_S_PHYSICAL && conduit == FFA_CONDUIT_ERET);
    let not_supported: bool = !NotificationSetImplemented(old_s, instance);
    let invalid_ids: bool = !PartitionIdRecognized(old_s, instance, sender) || !PartitionIdRecognized(old_s, instance, receiver);
    let invalid_flags: bool = reserved_flags != 0u32 || (delay_sri != 0u32 && instance != FFA_INSTANCE_S_VIRTUAL);
    let global_vcpu_nonzero: bool = !per_vcpu && vcpu_id != 0u32;
    let global_with_per_vcpu_notif: bool = !per_vcpu && BitmapHasPerVcpuNotification(old_s, sender, receiver, bitmap);
    let per_vcpu_with_global_notif: bool = per_vcpu && BitmapHasGlobalNotification(old_s, sender, receiver, bitmap);
    let per_vcpu_unsupported: bool = per_vcpu && !PerVcpuNotificationsSupported(old_s, receiver);
    let invalid_params: bool = invalid_ids || invalid_flags || global_vcpu_nonzero || global_with_per_vcpu_notif || per_vcpu_with_global_notif || per_vcpu_unsupported;
    let denied: bool = !SenderPermittedToSignal(old_s, sender, receiver, bitmap) || !ReceiverSupportsNotifications(old_s, receiver);
    let aborted: bool = ReceiverAborted(old_s, receiver);
    (not_supported ==> result != FFA_SUCCESS)
    && (invalid_params ==> result != FFA_SUCCESS)
    && (denied ==> result != FFA_SUCCESS)
    && (aborted ==> result != FFA_SUCCESS)
    && (result == NOT_SUPPORTED ==> not_supported && new_s == old_s)
    && (result == INVALID_PARAMETERS ==> invalid_params && new_s == old_s)
    && (result == DENIED ==> denied && new_s == old_s)
    && (result == ABORTED ==> aborted)
    && ((conduit_valid && !not_supported && !invalid_params && !denied && !aborted) ==>
        (result == FFA_SUCCESS
         && NotificationsSignaled(old_s, new_s, sender, receiver, bitmap, per_vcpu, vcpu_id)))
    && (result == FFA_SUCCESS ==>
        (conduit_valid && !not_supported && !invalid_params && !denied && !aborted
         && NotificationsSignaled(old_s, new_s, sender, receiver, bitmap, per_vcpu, vcpu_id)))
}
