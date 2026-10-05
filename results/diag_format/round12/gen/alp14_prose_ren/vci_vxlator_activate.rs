pub open spec fn vci_vxlator_activate_spec(base: Address, top: Address, result: Vcicommandreturncode, new_base: Address, old_s: S, new_s: S) -> bool {
  ((!(base % extent_size(old_s) == 0)) ==> result == VCI_ERROR_INPUT)
  && ((!(top % extent_size(old_s) == 0)) ==> result == VCI_ERROR_INPUT)
  && ((!(top <= base)) ==> result == VCI_ERROR_INPUT)
  && (result == VCI_SUCCESS ==> LBAMODE(new_s, base, new_base) == DEV)
  && (result == VCI_SUCCESS && base == Vxlatorat(new_s, Bltwalk(new_s, Currentvault(new_s), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blte.addr).base && new_base != Vxlatorat(new_s, Bltwalk(new_s, Currentvault(new_s), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blte.addr).top ==> Vxlatorat(new_s, Bltwalk(new_s, Currentvault(new_s), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blte.addr).state == VXLATOR_ACTIVATING)
  && (result == VCI_SUCCESS && new_base == Vxlatorat(new_s, Bltwalk(new_s, Currentvault(new_s), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blte.addr).top ==> Vxlatorat(new_s, Bltwalk(new_s, Currentvault(new_s), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blte.addr).state == VXLATOR_ACTIVE)
  && ((result != VCI_SUCCESS && (base % extent_size(old_s) == 0) && (top % extent_size(old_s) == 0) && (top > base))
    ==> LBAMODE(new_s, base, new_base) == LBAMODE(old_s, base, new_base))
  && (result != VCI_SUCCESS
    ==> Vxlatorat(new_s, Bltwalk(new_s, Currentvault(new_s), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blte.addr).state == Vxlatorat(old_s, Bltwalk(old_s, Currentvault(old_s), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blte.addr).state)
}