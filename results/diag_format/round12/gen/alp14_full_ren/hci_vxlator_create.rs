pub open spec fn hci_vxlator_create_spec(vd: Address, vxlator_ptr: Address, params_ptr: Address, result: Result<(), HciStatusCode>, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> ExtentAt(new_s, vxlator_ptr).state == ENROLLED)
  && (result == HCI_ERROR_NOT_SUPPORTED ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators)
  && (result == HCI_ERROR_INPUT ==> ExtentAt(new_s, vxlator_ptr).state == ENROLLED)
  && (result == HCI_ERROR_INPUT ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators)
  && (result == HCI_ERROR_INPUT ==> ExtentAt(new_s, vxlator_ptr).state == ENROLLED)
  && (result == HCI_ERROR_INPUT ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators)
  && (result == HCI_ERROR_INPUT ==> ExtentAt(new_s, vxlator_ptr).state == ENROLLED)
  && (result == HCI_ERROR_INPUT ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators)
  && (result == HCI_ERROR_INPUT ==> ExtentAt(new_s, vxlator_ptr).state == ENROLLED)
  && (result == HCI_ERROR_INPUT ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators)
  && (result == HCI_ERROR_INPUT ==> ExtentAt(new_s, vxlator_ptr).state == ENROLLED)
  && (result == HCI_ERROR_INPUT ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators)
  && (result == HCI_ERROR_INPUT ==> ExtentAt(new_s, vxlator_ptr).state == ENROLLED)
  && (result == HCI_ERROR_INPUT ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators)
  && (result == HCI_ERROR_INPUT ==> ExtentAt(new_s, vxlator_ptr).state == ENROLLED)
  && (result == HCI_ERROR_INPUT ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators)
  && (result == HCI_ERROR_INPUT ==> ExtentAt(new_s, vxlator_ptr).state == ENROLLED)
  && (result == HCI_ERROR_INPUT ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators)
  && (result == HCI_ERROR_INPUT ==> ExtentAt(new_s, vxlator_ptr).state == ENROLLED)
  && (result == HCI_ERROR_INPUT ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators)
  && (result.is_Ok() ==> ExtentAt(new_s, vxlator_ptr).state == VXLATOR)
  && (result.is_Ok() ==> VXLATORAt(new_s, vxlator_ptr).state == VXLATOR_INACTIVE)
  && (result.is_Ok() ==> VXLATORAt(new_s, vxlator_ptr).vault == VaultAt(new_s, vd).id)
  && (result.is_Ok() ==> VXLATORAt(new_s, vxlator_ptr).reg_base == VXLATORParamsAt(new_s, params_ptr).reg_base)
  && (result.is_Ok() ==> VXLATORAt(new_s, vxlator_ptr).reg_top == VXLATORParamsAt(new_s, params_ptr).reg_top)
  && (result.is_Ok() ==> VXLATORAt(new_s, vxlator_ptr).vidr == VXLATORParamsAt(new_s, params_ptr).vidr)
  && (result.is_Ok() ==> VXLATORAt(new_s, vxlator_ptr).idr[0] == VXLATORParamsAt(new_s, params_ptr).idr[0])
  && (result.is_Ok() ==> VXLATORAt(new_s, vxlator_ptr).idr[1] == VXLATORParamsAt(new_s, params_ptr).idr[1])
  && (result.is_Ok() ==> VXLATORAt(new_s, vxlator_ptr).idr[2] == VXLATORParamsAt(new_s, params_ptr).idr[2])
  && (result.is_Ok() ==> VXLATORAt(new_s, vxlator_ptr).idr[3] == VXLATORParamsAt(new_s, params_ptr).idr[3])
  && (result.is_Ok() ==> VXLATORAt(new_s, vxlator_ptr).idr[4] == VXLATORParamsAt(new_s, params_ptr).idr[4])
  && (result.is_Ok() ==> VXLATORAt(new_s, vxlator_ptr).idr[5] == VXLATORParamsAt(new_s, params_ptr).idr[5])
  && (result.is_Ok() ==> VXLATORAt(new_s, vxlator_ptr).idr[6] == VXLATORParamsAt(new_s, params_ptr).idr[6])
  && (result.is_Ok() ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators + 1)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> ExtentAt(new_s, vxlator_ptr).state == VXLATOR)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).state == VXLATOR_INACTIVE)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).vault == VaultAt(new_s, vd).id)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).reg_base == VXLATORParamsAt(new_s, params_ptr).reg_base)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).reg_top == VXLATORParamsAt(new_s, params_ptr).reg_top)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).vidr == VXLATORParamsAt(new_s, params_ptr).vidr)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[0] == VXLATORParamsAt(new_s, params_ptr).idr[0])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[1] == VXLATORParamsAt(new_s, params_ptr).idr[1])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[2] == VXLATORParamsAt(new_s, params_ptr).idr[2])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[3] == VXLATORParamsAt(new_s, params_ptr).idr[3])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[4] == VXLATORParamsAt(new_s, params_ptr).idr[4])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[5] == VXLATORParamsAt(new_s, params_ptr).idr[5])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[6] == VXLATORParamsAt(new_s, params_ptr).idr[6])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators + 1)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> ExtentAt(new_s, vxlator_ptr).state == ExtentAt(old_s, vxlator_ptr).state)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).state == VXLATORAt(old_s, vxlator_ptr).state)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).vault == VXLATORAt(old_s, vxlator_ptr).vault)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).reg_base == VXLATORAt(old_s, vxlator_ptr).reg_base)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).reg_top == VXLATORAt(old_s, vxlator_ptr).reg_top)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).vidr == VXLATORAt(old_s, vxlator_ptr).vidr)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[0] == VXLATORAt(old_s, vxlator_ptr).idr[0])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[1] == VXLATORAt(old_s, vxlator_ptr).idr[1])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[2] == VXLATORAt(old_s, vxlator_ptr).idr[2])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[3] == VXLATORAt(old_s, vxlator_ptr).idr[3])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[4] == VXLATORAt(old_s, vxlator_ptr).idr[4])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[5] == VXLATORAt(old_s, vxlator_ptr).idr[5])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VXLATORAt(new_s, vxlator_ptr).idr[6] == VXLATORAt(old_s, vxlator_ptr).idr[6])
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators)
  && (result.is_Err()
    ==> ExtentAt(new_s, vxlator_ptr).state == ExtentAt(old_s, vxlator_ptr).state)
  && (result.is_Err()
    ==> VaultAt(new_s, vd).num_vxlators == VaultAt(old_s, vd).num_vxlators)
}