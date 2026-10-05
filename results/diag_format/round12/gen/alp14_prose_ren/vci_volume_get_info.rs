pub open spec fn vci_volume_get_info_spec(volume_id: Bits64, addr: Address, result: Vcicommandreturncode, old_s: S, new_s: S) -> bool {
  (Currentvault(old_s).device_assignment_feature_enabled == false ==> result == VCI_ERROR_STATE)
  && (Volumefromvolumeid(old_s, Currentvault(old_s),volume_id).is_none() ==> result == VCI_ERROR_INPUT)
  && ((addr) % 512 != 0 ==> result == VCI_ERROR_INPUT)
  && (!is_within_protected_lba_space(old_s, Currentvault(old_s), addr) ==> result == VCI_ERROR_INPUT)
  && (Driveat(old_s, Volumefromvolumeid(old_s, Currentvault(old_s),volume_id).unwrap().drive).LBAMODE == EMPTY ==> result == VCI_ERROR_INPUT)
  && ((!(Currentvault(old_s).device_assignment_feature_enabled == false) &&
       Volumefromvolumeid(old_s, Currentvault(old_s),volume_id).is_some() &&
       ((addr) % 512 == 0) &&
       is_within_protected_lba_space(old_s, Currentvault(old_s), addr) &&
       !(Driveat(old_s, Volumefromvolumeid(old_s, Currentvault(old_s),volume_id).unwrap().drive).LBAMODE == EMPTY))
    ==> result == VCI_SUCCESS)
  && (result == VCI_ERROR_INPUT ==> Currentvault(new_s) == Currentvault(old_s))
  && (result == VCI_ERROR_INPUT ==> Volumefromvolumeid(new_s, Currentvault(new_s),volume_id) == Volumefromvolumeid(old_s, Currentvault(old_s),volume_id))
  && (result == VCI_ERROR_INPUT ==> Driveat(new_s, Volumefromvolumeid(new_s, Currentvault(new_s),volume_id).unwrap().drive) == Driveat(old_s, Volumefromvolumeid(old_s, Currentvault(old_s),volume_id).unwrap().drive))
  && (result == VCI_ERROR_INPUT ==> Vcivolumeinfoat(new_s, addr) == Vcivolumeinfoat(old_s, addr))
  && (result == VCI_ERROR_STATE ==> Currentvault(new_s) == Currentvault(old_s))
  && (result == VCI_ERROR_STATE ==> Volumefromvolumeid(new_s, Currentvault(new_s),volume_id) == Volumefromvolumeid(old_s, Currentvault(old_s),volume_id))
  && (result == VCI_ERROR_STATE ==> Driveat(new_s, Volumefromvolumeid(new_s, Currentvault(new_s),volume_id).unwrap().drive) == Driveat(old_s, Volumefromvolumeid(old_s, Currentvault(old_s),volume_id).unwrap().drive))
  && (result == VCI_ERROR_STATE ==> Vcivolumeinfoat(new_s, addr) == Vcivolumeinfoat(old_s, addr))
  && (result == VCI_SUCCESS ==> Currentvault(new_s) == Currentvault(old_s))
  && (result == VCI_SUCCESS ==> Volumefromvolumeid(new_s, Currentvault(new_s),volume_id) == Volumefromvolumeid(old_s, Currentvault(old_s),volume_id))
  && (result == VCI_SUCCESS ==> Driveat(new_s, Volumefromvolumeid(new_s, Currentvault(new_s),volume_id).unwrap().drive) == Driveat(old_s, Volumefromvolumeid(old_s, Currentvault(old_s),volume_id).unwrap().drive))
  && (result == VCI_SUCCESS ==> Vcivolumeinfoat(new_s, addr) == Vcivolumeinfoat(old_s, addr))
  && (result != VCI_SUCCESS ==> Vcivolumeinfoat(new_s, addr) == Vcivolumeinfoat(old_s, addr))
}