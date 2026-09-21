pub open spec fn ffa_notification_bitmap_destroy_spec(vm_id: UInt32, result: int32, old_s: S, new_s: S) -> bool {
  (result == FFA_INVALID_PARAMETERS)
  && (result == FFA_NOT_SUPPORTED)
  && (result == FFA_DENIED)
  && (result == FFA_SUCCESS)
}