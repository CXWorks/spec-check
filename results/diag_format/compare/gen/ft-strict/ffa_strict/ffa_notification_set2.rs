pub open spec fn ffa_notification_set2_spec(ids: UInt32, flags: UInt64, bitmap: [UInt64; 6], result: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_SET2, CurrentFfaInstance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsRecognizedPartitionId(old_s, Bits(ids, 31, 16)) || !IsRecognizedPartitionId(old_s, Bits(ids, 15, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidNotificationSetFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 0, 0) == 0 && Bits(flags, 63, 16) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 0, 0) == 0 && BitmapContainsPerVcpuNotification(old_s, Bits(ids, 15, 0), bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 0, 0) == 1 && BitmapContainsGlobalNotification(old_s, Bits(ids, 15, 0), bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 0, 0) == 1 && !PerVcpuNotificationsSupported(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (exists|i: UInt64| i >= NumSupportedNotifications(old_s) && i < 384 && BitmapBit(bitmap, i) == 1 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (forall|i: UInt64| i < 384 ==> BitmapBit(bitmap, i) == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (exists|i: UInt64| BitmapBit(bitmap, i) == 1 && !SenderMaySignalNotification(old_s, Bits(ids, 31, 16), Bits(ids, 15, 0), i) ==> ResultEqual(result, DENIED))
  && (!ReceiverSupportsNotifications(old_s, Bits(ids, 15, 0)) ==> ResultEqual(result, DENIED))
  && (PartitionHasAborted(old_s, Bits(ids, 15, 0)) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> Bits(flags, 0, 0) == 0 ==> (forall|i: UInt64| BitmapBit(bitmap, i) == 1 ==> GlobalNotificationSignaled(Bits(ids, 15, 0), i)))
  && (result == FFA_SUCCESS ==> Bits(flags, 0, 0) == 1 ==> (forall|i: UInt64| BitmapBit(bitmap, i) == 1 ==> PerVcpuNotificationSignaled(Bits(ids, 15, 0), Bits(flags, 63, 16), i)))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_SET2, CurrentFfaInstance(old_s)) &&
       IsRecognizedPartitionId(old_s, Bits(ids, 31, 16)) &&
       IsRecognizedPartitionId(old_s, Bits(ids, 15, 0)) &&
       IsValidNotificationSetFlags(old_s, flags) &&
       !(Bits(flags, 0, 0) == 0 && Bits(flags, 63, 16) != 0) &&
       !(Bits(flags, 0, 0) == 0 && BitmapContainsPerVcpuNotification(old_s, Bits(ids, 15, 0), bitmap)) &&
       !(Bits(flags, 0, 0) == 1 && BitmapContainsGlobalNotification(old_s, Bits(ids, 15, 0), bitmap)) &&
       !(Bits(flags, 0, 0) == 1 && !PerVcpuNotificationsSupported(old_s)) &&
       !(exists|i: UInt64| i >= NumSupportedNotifications(old_s) && i < 384 && BitmapBit(bitmap, i) == 1) &&
       !(forall|i: UInt64| i < 384 ==> BitmapBit(bitmap, i) == 0) &&
       !(exists|i: UInt64| BitmapBit(bitmap, i) == 1 && !SenderMaySignalNotification(old_s, Bits(ids, 31, 16), Bits(ids, 15, 0), i)) &&
     ReceiverSupportsNotifications(old_s, Bits(ids, 15, 0)) &&
     !PartitionHasAborted(old_s, Bits(ids, 15, 0)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> GlobalNotificationSignaled(new_s, Bits(ids, 15, 0), 0 as int) == GlobalNotificationSignaled(old_s, Bits(ids, 15, 0), 0 as int))
  && (result != FFA_SUCCESS
    ==> PerVcpuNotificationSignaled(new_s, Bits(ids, 15, 0), Bits(flags, 63, 16), 0 as int) == PerVcpuNotificationSignaled(old_s, Bits(ids, 15, 0), Bits(flags, 63, 16), 0 as int))
}