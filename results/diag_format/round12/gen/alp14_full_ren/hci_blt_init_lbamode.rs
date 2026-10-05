pub open spec fn hci_blt_init_lbamode_spec(vd: Vd, base: UInt64, top: UInt64, result: Result<(), HciStatusCode>, out_top: UInt64, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_INPUT && !(vd as Extent) can be enrolled(old_s) ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT && ExtentAt(old_s, vd as Extent).vd_state != VD_STATE ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT && top <= base ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_VAULT && VaultAt(old_s, vd as Extent).vault_state != VAULT_NEW ==> result == HCI_ERROR_VAULT)
  && (result == HCI_ERROR_BLT && !AddrIsBltLevelAligned(old_s, base, BltAt(old_s, BltWalk(old_s, base).level).level as int) ==> result == HCI_ERROR_BLT(BltWalk(new_s, base).level as int))
  && (result == HCI_ERROR_BLT && BltAt(old_s, BltWalk(old_s, base).level).state != UNASSIGNED ==> result == HCI_ERROR_BLT(BltWalk(new_s, base).level as int))
  && (result == HCI_ERROR_INPUT && !IsAlignedToExtentSize(old_s, top) ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_BLT && base == BltAt(old_s, BltWalk(old_s, base).level).top ==> result == HCI_ERROR_BLT(BltWalk(new_s, base).level as int))
  && ((!(result == HCI_ERROR_INPUT && !(vd as Extent) can be enrolled(old_s)) &&
       ExtentAt(old_s, vd as Extent).vd_state == VD_STATE &&
       result == HCI_SUCCESS)
    ==> BltAt(new_s, BltWalk(new_s, base).level).lbamode == RAM)
  && (result == HCI_SUCCESS
    ==> out_top == BltAt(new_s, BltWalk(new_s, base).level).top)
  && ((!(result == HCI_ERROR_INPUT && !(vd as Extent) can be enrolled(old_s)) &&
       ExtentAt(old_s, vd as Extent).vd_state == VD_STATE &&
       result != HCI_SUCCESS)
    ==> BltAt(new_s, BltWalk(new_s, base).level).lbamode == BltAt(old_s, BltWalk(old_s, base).level).lbamode)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).ifp == VaultAt(old_s, vd as Extent).ifp)
  && ((result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_VAULT &&
       result != HCI_ERROR_BLT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_BLT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_BLT)
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VAULT_NEW)
  && ((result == HCI_ERROR_INPUT &&
       result != HCI_ERROR_VAULT &&
       result != HCI_ERROR_BLT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_BLT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_BLT)
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)
  && (result == HCI_SUCCESS
    ==> VaultAt(new_s, vd as Extent).vault_state == VaultAt(old_s, vd as Extent).vault_state)