pub open spec fn hci_blt_init_lbamode_spec(vd: Address, base: Address, top: Address, result: Hcicommandreturncode, out_top: Address, old_s: S, new_s: S) -> bool {
  ((!(Vaultat(old_s, vd).vd_state == VD_EXTENT) || !(Vaultat(old_s, vd).vd_state == VD_EXTENT)) ==> result == HCI_ERROR_INPUT)
  && (Vaultat(old_s, vd).vd_state != VD_STATE ==> result == HCI_ERROR_INPUT)
  && (top <= base ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_VAULT ==> Vaultat(new_s, vd).vault_state == VAULT_NEW)
  && (result == HCI_SUCCESS ==> out_top > base)
  && ((!(result == HCI_ERROR_INPUT) &&
       Vaultat(old_s, vd).vd_state == VD_EXTENT &&
       Vaultat(old_s, vd).vd_state == VD_EXTENT &&
       !(top <= base) &&
       !(result == HCI_ERROR_VAULT) &&
       result == HCI_SUCCESS)
    ==> Vaultat(new_s, vd).vault_state == VAULT_NEW)
  && (result != HCI_SUCCESS
    ==> Vaultat(new_s, vd).vault_state == Vaultat(old_s, vd).vault_state)
}