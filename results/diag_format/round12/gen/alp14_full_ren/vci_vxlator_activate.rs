pub open spec fn vci_vxlator_activate_spec(base: UInt64, top: UInt64, result: RsiCommandReturnCode, new_base: UInt64, old_s: S, new_s: S) -> bool {
  ((base % EXTENT_SIZE) != 0 ==> result == VCI_ERROR_INPUT)
  && ((top % EXTENT_SIZE) != 0 ==> result == VCI_ERROR_INPUT)
  && (top <= base ==> result == VCI_ERROR_INPUT)
  && (result == VCI_SUCCESS ==> LBAMODE(new_s, base, new_base) == DEV)
  && (result == VCI_SUCCESS && base == VXLATOR_REGISTER_REGION_BASE(old_s) && new_base != top ==> VXLATOR_STATE(new_s) == VXLATOR_ACTIVATING)
  && (result == VCI_SUCCESS && new_base == VXLATOR_REGISTER_REGION_TOP(old_s) ==> VXLATOR_STATE(new_s) == VXLATOR_ACTIVE)
  && ((!(base % EXTENT_SIZE) == 0 &&
       (top % EXTENT_SIZE) == 0 &&
       !(top <= base))
    ==> result == VCI_SUCCESS)
  && (result != VCI_SUCCESS
    ==> LBAMODE(new_s, base, new_base) == LBAMODE(old_s, base, new_base))
  && (result != VCI_SUCCESS
    ==> VXLATOR_STATE(new_s) == VXLATOR_STATE(old_s))
  && (!(result == VCI_SUCCESS && (base == VXLATOR_REGISTER_REGION_BASE(old_s) && new_base != top)) ==> VXLATOR_STATE(new_s) == VXLATOR_STATE(old_s))
}