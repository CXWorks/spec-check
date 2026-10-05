pub open spec fn hci_vault_destroy_spec(vd: Address, result: HciCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((vd % EXTENT_SIZE) != 0 ==> result == HCI_ERROR_INPUT)
  && (!CookieIsValid(old_s, vd) ==> result == HCI_ERROR_INPUT)
  && (!ExtentAt(old_s, vd).is_VD() ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_VAULT ==> VaultAt(old_s, vd).is_live())
  && (result == HCI_SUCCESS ==> ExtentAt(new_s, vd).is_ENROLLED())
  && (result == HCI_SUCCESS && VaultAt(old_s, vd).zone_policy == ZONE_POLICY_PRIVATE ==> VaultAt(new_s, vd).zone_id == ZONE_STATE_PRIVATE_UNASSIGNED)
  && (result == HCI_SUCCESS && VaultAt(old_s, vd).zone_policy == ZONE_POLICY_SHARED ==> RealmAt(new_s, VaultAt(new_s, vd).zone_id as int).members.len() == RealmAt(old_s, VaultAt(old_s, vd).zone_id as int).members.len() - 1)
  && ((!( (vd % EXTENT_SIZE) != 0) &&
       CookieIsValid(old_s, vd) &&
       ExtentAt(old_s, vd).is_VD())
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> ExtentAt(new_s, vd).is_VD())
  && (result != HCI_SUCCESS
    ==> ExtentAt(new_s, vd).is_ENROLLED())
  && (result != HCI_SUCCESS
    ==> VaultAt(new_s, vd).zone_id == VaultAt(old_s, vd).zone_id)
  && (result != HCI_SUCCESS
    ==> RealmAt(new_s, VaultAt(new_s, vd).zone_id as int).members.len() == RealmAt(old_s, VaultAt(old_s, vd).zone_id as int).members.len())
  && (result != HCI_SUCCESS
    ==> ExtentAt(new_s, VaultAt(new_s, vd).first_blt_base as int).is_ENROLLED())
}