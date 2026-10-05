pub open spec fn hci_vxlator_create_spec(vd: Address, vxlator_ptr: Address, params_ptr: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_ERROR_INPUT ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_ERROR_INPUT ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_ERROR_INPUT ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_ERROR_INPUT ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_ERROR_INPUT ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_ERROR_INPUT ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_ERROR_INPUT ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_ERROR_INPUT ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_ERROR_INPUT ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_ERROR_INPUT ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_ERROR_INPUT ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_ERROR_INPUT ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result == HCI_SUCCESS ==> Extentat(new_s, vxlator_ptr).state == VXLATOR)
  && (result == HCI_SUCCESS ==> Vxlatorat(new_s, vxlator_ptr).state == VXLATOR_INACTIVE)
  && (result == HCI_SUCCESS ==> Vxlatorat(new_s, vxlator_ptr).vault == Vaultat(new_s, vd))
  && (result == HCI_SUCCESS ==> Vxlatorat(new_s, vxlator_ptr).reg_base == Hcivxlatorparamsat(new_s, params_ptr).reg_base)
  && (result == HCI_SUCCESS ==> Vxlatorat(new_s, vxlator_ptr).reg_top == Hcivxlatorparamsat(new_s, params_ptr).reg_top)
  && (result == HCI_SUCCESS ==> Vxlatorat(new_s, vxlator_ptr).vidr == Hcivxlatorparamsat(new_s, params_ptr).vidr)
  && (result == HCI_SUCCESS ==> Vxlatorat(new_s, vxlator_ptr).idr[0] == Hcivxlatorparamsat(new_s, params_ptr).idr[0])
  && (result == HCI_SUCCESS ==> Vxlatorat(new_s, vxlator_ptr).idr[1] == Hcivxlatorparamsat(new_s, params_ptr).idr[1])
  && (result == HCI_SUCCESS ==> Vxlatorat(new_s, vxlator_ptr).idr[2] == Hcivxlatorparamsat(new_s, params_ptr).idr[2])
  && (result == HCI_SUCCESS ==> Vxlatorat(new_s, vxlator_ptr).idr[3] == Hcivxlatorparamsat(new_s, params_ptr).idr[3])
  && (result == HCI_SUCCESS ==> Vxlatorat(new_s, vxlator_ptr).idr[4] == Hcivxlatorparamsat(new_s, params_ptr).idr[4])
  && (result == HCI_SUCCESS ==> Vxlatorat(new_s, vxlator_ptr).idr[5] == Hcivxlatorparamsat(new_s, params_ptr).idr[5])
  && (result == HCI_SUCCESS ==> Vxlatorat(new_s, vxlator_ptr).idr[6] == Hcivxlatorparamsat(new_s, params_ptr).idr[6])
  && (result == HCI_SUCCESS ==> Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators + 1)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators + 1))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Extentat(new_s, vxlator_ptr).state == VXLATOR))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vxlatorat(new_s, vxlator_ptr).state == VXLATOR_INACTIVE))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vxlatorat(new_s, vxlator_ptr).vault == Vaultat(new_s, vd)))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vxlatorat(new_s, vxlator_ptr).reg_base == Hcivxlatorparamsat(new_s, params_ptr).reg_base))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vxlatorat(new_s, vxlator_ptr).reg_top == Hcivxlatorparamsat(new_s, params_ptr).reg_top))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vxlatorat(new_s, vxlator_ptr).vidr == Hcivxlatorparamsat(new_s, params_ptr).vidr))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[0] == Hcivxlatorparamsat(new_s, params_ptr).idr[0]))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[1] == Hcivxlatorparamsat(new_s, params_ptr).idr[1]))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[2] == Hcivxlatorparamsat(new_s, params_ptr).idr[2]))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[3] == Hcivxlatorparamsat(new_s, params_ptr).idr[3]))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[4] == Hcivxlatorparamsat(new_s, params_ptr).idr[4]))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[5] == Hcivxlatorparamsat(new_s, params_ptr).idr[5]))
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT)
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[6] == Hcivxlatorparamsat(new_s, params_ptr).idr[6]))
  && (result != HCI_SUCCESS
    ==> (Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators))
  && (result != HCI_SUCCESS
    ==> (Extentat(new_s, vxlator_ptr).state == Extentat(old_s, vxlator_ptr).state))
  && (result != HCI_SUCCESS
    ==> (Vxlatorat(new_s, vxlator_ptr).state == Vxlatorat(old_s, vxlator_ptr).state))
  && (result != HCI_SUCCESS
    ==> (Vxlatorat(new_s, vxlator_ptr).vault == Vxlatorat(old_s, vxlator_ptr).vault))
  && (result != HCI_SUCCESS
    ==> (Vxlatorat(new_s, vxlator_ptr).reg_base == Vxlatorat(old_s, vxlator_ptr).reg_base))
  && (result != HCI_SUCCESS
    ==> (Vxlatorat(new_s, vxlator_ptr).reg_top == Vxlatorat(old_s, vxlator_ptr).reg_top))
  && (result != HCI_SUCCESS
    ==> (Vxlatorat(new_s, vxlator_ptr).vidr == Vxlatorat(old_s, vxlator_ptr).vidr))
  && (result != HCI_SUCCESS
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[0] == Vxlatorat(old_s, vxlator_ptr).idr[0]))
  && (result != HCI_SUCCESS
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[1] == Vxlatorat(old_s, vxlator_ptr).idr[1]))
  && (result != HCI_SUCCESS
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[2] == Vxlatorat(old_s, vxlator_ptr).idr[2]))
  && (result != HCI_SUCCESS
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[3] == Vxlatorat(old_s, vxlator_ptr).idr[3]))
  && (result != HCI_SUCCESS
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[4] == Vxlatorat(old_s, vxlator_ptr).idr[4]))
  && (result != HCI_SUCCESS
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[5] == Vxlatorat(old_s, vxlator_ptr).idr[5]))
  && (result != HCI_SUCCESS
    ==> (Vxlatorat(new_s, vxlator_ptr).idr[6] == Vxlatorat(old_s, vxlator_ptr).idr[6]))
  && (!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_ERROR_INPUT
     ==> Vaultat(new_s, vd).num_vxlators == Vaultat(old_s, vd).num_vxlators + 1)
}