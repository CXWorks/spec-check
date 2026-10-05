pub open spec fn hci_vault_destroy_spec(vd: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  ((!(vd % extent_size(old_s)) ==> HCI_ERROR_INPUT(result))
   ==> true)
  && ((!(is_physical_address_that_can_be_enrolled(old_s, vd)) ==> HCI_ERROR_INPUT(result))
   ==> true)
  && ((!(Extentat(old_s, vd).state == EXTENT_STATE_VD) ==> HCI_ERROR_INPUT(result))
   ==> true)
  && ((Vaultat(old_s, vd).live ==> HCI_ERROR_VAULT(result))
   ==> true)
  && (result == HCI_SUCCESS
    ==> Bltsextentstate(new_s, Vaultat(new_s, vd).blt_base[0], Vaultat(new_s, vd).blt_num_start as int) == ENROLLED)
  && (result == HCI_SUCCESS
    ==> Extentat(new_s, vd).state == ENROLLED)
  && (result == HCI_SUCCESS
    ==> TENANTID(new_s, Vaultat(new_s, vd).tenantid) == FREE)
  && (result == HCI_SUCCESS
    && Vaultat(old_s, vd).zone_policy == ZONE_POLICY_PRIVATE
    ==> Vaultat(new_s, vd).zone_id == ZONE_STATE_PRIVATE_UNASSIGNED)
  && (result == HCI_SUCCESS
    && Vaultat(old_s, vd).zone_policy == ZONE_POLICY_SHARED
    ==> Zonemembers(new_s, Vaultat(new_s, vd).zoneid) == Zonemembers(old_s, Vaultat(old_s, vd).zoneid) - 1)
  && ((!(result == HCI_ERROR_INPUT(result)) &&
       !(result == HCI_ERROR_VAULT(result)))
    ==> Vaultat(new_s, vd).live == false)
  && (result != HCI_SUCCESS
    ==> Extentat(new_s, vd).state == Extentat(old_s, vd).state)
  && (result != HCI_SUCCESS
    ==> Vaultat(new_s, vd).zone_id == Vaultat(old_s, vd).zone_id)
  && (result != HCI_SUCCESS
    ==> Zonemembers(new_s, Vaultat(new_s, vd).zoneid) == Zonemembers(old_s, Vaultat(old_s, vd).zoneid))
  && (result != HCI_SUCCESS
    ==> Vaultat(new_s, vd).zone_policy == Vaultat(old_s, vd).zone_policy)
}