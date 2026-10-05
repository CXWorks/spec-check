pub open spec fn vci_volume_get_info_spec(volume_id: VolumeId, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (VciDeviceAssignmentEnabled(old_s) ==> result == RSI_SUCCESS)
  && (result == RSI_ERROR_STATE ==> VciDeviceAssignmentEnabled(old_s))
  && ((!(VciDeviceAssignmentEnabled(old_s)))
    ==> result == RSI_ERROR_STATE)
  && (result == RSI_SUCCESS
    ==> VciVolumeAssigned(old_s, volume_id))
  && (result == RSI_SUCCESS
    ==> (addr % 512) == 0)
  && (result == RSI_SUCCESS
    ==> addr within_protected_lba_space(old_s))
  && (result == RSI_SUCCESS
    ==> LbaMode(old_s, addr) != EMPTY)
  && ((!(VciDeviceAssignmentEnabled(old_s)))
    || !(VciVolumeAssigned(old_s, volume_id))
    || (addr % 512) != 0
    || !(addr within_protected_lba_space(old_s))
    || LbaMode(old_s, addr) == EMPTY
    ==> result == RSI_ERROR_INPUT)
  && (result != RSI_SUCCESS
    ==> VciVolumeAssigned(new_s, volume_id) == VciVolumeAssigned(old_s, volume_id))
  && (result != RSI_SUCCESS
    ==> LbaMode(new_s, addr) == LbaMode(old_s, addr))
}