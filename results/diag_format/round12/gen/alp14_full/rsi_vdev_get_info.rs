pub open spec fn rsi_vdev_get_info_spec(vdev_id: UInt64, addr: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).device_assignment == FEATURE_TRUE ==> result == RSI_SUCCESS)
  && (result == RSI_SUCCESS && VdevAt(old_s, vdev_id).assigned == false ==> result == RSI_ERROR_INPUT)
  && ((addr) % 512 != 0 ==> result == RSI_ERROR_INPUT)
  && (!IsProtectedIpaSpace(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (RipasAt(old_s, addr) == EMPTY ==> result == RSI_ERROR_INPUT)
  && ((!(ImplFeatures(old_s).device_assignment == FEATURE_TRUE) &&
       VdevAt(old_s, vdev_id).assigned == true &&
       ((addr) % 512 == 0) &&
       IsProtectedIpaSpace(old_s, addr) &&
       !(RipasAt(old_s, addr) == EMPTY))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> VdevAt(new_s, vdev_id).assigned == VdevAt(old_s, vdev_id).assigned)
  && (result != RSI_SUCCESS
    ==> RipasAt(new_s, addr) == RipasAt(old_s, addr))
}