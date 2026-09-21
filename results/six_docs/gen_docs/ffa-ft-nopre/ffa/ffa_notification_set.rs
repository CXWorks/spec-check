pub open spec fn ffa_notification_set_spec(sender: UInt32, receiver: UInt32, flags: UInt32, notification_bitmap: UInt32, notification_bitmap: UInt32, result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  (result == FFA_SUCCESS)
  && (result == FFA_ERROR_INVALID_PARAMETERS && (flags & 1 == 0 && (receiver & 0xFFFF) != 0))
  && (result == FFA_ERROR_INVALID_PARAMETERS && (flags & 1 == 0 && ((notification_bitmap | notification_bitmap) != 0)))
  && (result == FFA_ERROR_INVALID_PARAMETERS && (flags & 1 == 1 && ((notification_bitmap | notification_bitmap) == 0)))
  && (result == FFA_ERROR_INVALID_PARAMETERS && (flags & 1 == 1 && !(PerVcpuNotificationsSupported(old_s))))
  && (result == FFA_ERROR_NOT_SUPPORTED)
  && (result == FFA_ERROR_DENIED)
  && (result == FFA_ERROR_ABORTED)
  && ((!(result == FFA_ERROR_INVALID_PARAMETERS && (flags & 1 == 0 && (receiver & 0xFFFF) != 0)) &&
       !(result == FFA_ERROR_INVALID_PARAMETERS && (flags & 1 == 0 && ((notification_bitmap | notification_bitmap) != 0))) &&
       !(result == FFA_ERROR_INVALID_PARAMETERS && (flags & 1 == 1 && ((notification_bitmap | notification_bitmap) == 0))) &&
       !(result == FFA_ERROR_INVALID_PARAMETERS && (flags & 1 == 1 && !(PerVcpuNotificationsSupported(old_s)))) &&
       !(result == FFA_ERROR_NOT_SUPPORTED) &&
       !(result == FFA_ERROR_DENIED) &&
       !(result == FFA_ERROR_ABORTED))
    ==> result == FFA_SUCCESS)
}