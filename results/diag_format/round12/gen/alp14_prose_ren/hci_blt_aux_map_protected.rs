pub open spec fn hci_blt_aux_map_protected_spec(vd: Address, lba: Address, index: UInt64, result: Hcicommandreturncode, state: Hcibltentrystate, lbamode: Hcilbamode, old_s: S, new_s: S) -> bool {
  ((!(Vaultat(old_s, vd).extent_addr % EXTENT_SIZE == 0) ==> result == HCI_ERROR_INPUT)
   ==> result == HCI_ERROR_INPUT)
  && ((!(is_enrollable_physical_address(old_s, vd)) ==> result == HCI_ERROR_INPUT)
   ==> result == HCI_ERROR_INPUT)
  && ((!(Extentat(old_s, vd).state == VD_STATE) ==> result == HCI_ERROR_INPUT)
   ==> result == HCI_ERROR_INPUT)
  && ((!(lba % EXTENT_SIZE == 0) ==> result == HCI_ERROR_INPUT)
   ==> result == HCI_ERROR_INPUT)
  && ((!(is_protected_lba(old_s, vd, lba)) ==> result == HCI_ERROR_INPUT)
   ==> result == HCI_ERROR_INPUT)
  && (((!(Vaultat(old_s, vd).blt_tree_per_tier == FEATURE_FALSE) &&
        !(index == KEEPER_BLT_TREE_PRIMARY) &&
        !(index > Vaultat(old_s, vd).aux_tier_count))
    ==> result == HCI_ERROR_INPUT)
   ==> result == HCI_ERROR_INPUT)
  && ((Bltwalk(old_s, Vaultat(old_s, vd), lba,
               KEEPER_BLT_PAGE_LEVEL as int,
               KEEPER_BLT_TREE_PRIMARY as int).result == KEEPER_BLT_ENTRY_ASSIGNED ||
       Bltwalk(old_s, Vaultat(old_s, vd), lba,
               KEEPER_BLT_PAGE_LEVEL as int,
               KEEPER_BLT_TREE_PRIMARY as int).result == KEEPER_BLT_ENTRY_ASSIGNED_DEV ||
       Bltwalk(old_s, Vaultat(old_s, vd), lba,
               KEEPER_BLT_PAGE_LEVEL as int,
               KEEPER_BLT_TREE_PRIMARY as int).result == KEEPER_BLT_ENTRY_ASSIGNED_VXLATOR)
    ==> result == HCI_ERROR_BLT)
   ==> result == HCI_ERROR_BLT)
  && ((Bltwalk(old_s, Vaultat(old_s, vd), lba,
               KEEPER_BLT_PAGE_LEVEL as int,
               KEEPER_BLT_TREE_PRIMARY as int).result == KEEPER_BLT_ENTRY_ASSIGNED &&
       Bltwalk(old_s, Vaultat(old_s, vd), lba,
               KEEPER_BLT_PAGE_LEVEL as int,
               KEEPER_BLT_TREE_PRIMARY as int).LBAMODE != RAM)
    ==> result == HCI_ERROR_BLT)
   ==> result == HCI_ERROR_BLT)
  && ((Bltwalk(old_s, Vaultat(old_s, vd), lba,
               KEEPER_BLT_PAGE_LEVEL as int,
               KEEPER_BLT_TREE_PRIMARY as int).result == KEEPER_BLT_ENTRY_ASSIGNED_DEV &&
       Bltwalk(old_s, Vaultat(old_s, vd), lba,
               KEEPER_BLT_PAGE_LEVEL as int,
               KEEPER_BLT_TREE_PRIMARY as int).LBAMODE != DEV)
    ==> result == HCI_ERROR_BLT)
   ==> result == HCI_ERROR_BLT)
  && ((Bltwalk(old_s, Vaultat(old_s, vd), lba,
               KEEPER_BLT_PAGE_LEVEL as int,
               index as int).result == KEEPER_BLT_ENTRY_AUX_DESTROYED)
    ==> result == HCI_ERROR_BLT_AUX)
   ==> result == HCI_ERROR_BLT_AUX)
  && ((Bltwalk(old_s, Vaultat(old_s, vd), lba,
               KEEPER_BLT_PAGE_LEVEL as int,
               KEEPER_BLT_TREE_PRIMARY as int).result.level <
        Bltwalk(old_s, Vaultat(old_s, vd), lba,
               KEEPER_BLT_PAGE_LEVEL as int,
               index as int).result.level)
    ==> result == HCI_ERROR_BLT_AUX)
   ==> result == HCI_ERROR_BLT_AUX)
  && (result == HCI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                               KEEPER_BLT_PAGE_LEVEL as int,
                               index as int).blt_addr,
                 Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                  KEEPER_BLT_PAGE_LEVEL as int,
                                                  index as int).result.level)).state == KEEPER_BLT_ENTRY_ASSIGNED)
  && (result == HCI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                               KEEPER_BLT_PAGE_LEVEL as int,
                               index as int).blt_addr,
                 Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                  KEEPER_BLT_PAGE_LEVEL as int,
                                                  index as int).result.level)).protection_attributes == Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                                                                                        KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                        KEEPER_BLT_TREE_PRIMARY as int).blt_addr,
                                                                  Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                                                                   KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                   KEEPER_BLT_TREE_PRIMARY as int).result.level)).protection_attributes)
  && (result == HCI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                               KEEPER_BLT_PAGE_LEVEL as int,
                               index as int).blt_addr,
                 Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                  KEEPER_BLT_PAGE_LEVEL as int,
                                                  index as int).result.level)).shareability_attribute == Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                                                                                        KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                        KEEPER_BLT_TREE_PRIMARY as int).blt_addr,
                                                                  Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                                                                   KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                   KEEPER_BLT_TREE_PRIMARY as int).result.level)).shareability_attribute)
  && ((!(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_BLT) &&
       !(result == HCI_ERROR_BLT_AUX))
    ==> Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                               KEEPER_BLT_PAGE_LEVEL as int,
                               index as int).blt_addr,
                 Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                  KEEPER_BLT_PAGE_LEVEL as int,
                                                  index as int).result.level)).state == Bltentry(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                 KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                 index as int).blt_addr,
                                                                               Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                index as int).result.level)).state)
  && (result == HCI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                               KEEPER_BLT_PAGE_LEVEL as int,
                               index as int).blt_addr,
                 Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                  KEEPER_BLT_PAGE_LEVEL as int,
                                                  index as int).result.level)).LBAMODE == Bltentry(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                 KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                 index as int).blt_addr,
                                                                               Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                index as int).result.level)).LBAMODE)
  && (result == HCI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                               KEEPER_BLT_PAGE_LEVEL as int,
                               index as int).blt_addr,
                 Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                  KEEPER_BLT_PAGE_LEVEL as int,
                                                  index as int).result.level)).output_address == Bltentry(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                 KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                 index as int).blt_addr,
                                                                               Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                index as int).result.level)).output_address)
  && (result != HCI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                               KEEPER_BLT_PAGE_LEVEL as int,
                               index as int).blt_addr,
                 Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                  KEEPER_BLT_PAGE_LEVEL as int,
                                                  index as int).result.level)).state == Bltentry(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                 KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                 index as int).blt_addr,
                                                                               Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                index as int).result.level)).state)
  && (result != HCI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                               KEEPER_BLT_PAGE_LEVEL as int,
                               index as int).blt_addr,
                 Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                  KEEPER_BLT_PAGE_LEVEL as int,
                                                  index as int).result.level)).protection_attributes == Bltentry(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                 KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                 index as int).blt_addr,
                                                                               Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                index as int).result.level)).protection_attributes)
  && (result != HCI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                               KEEPER_BLT_PAGE_LEVEL as int,
                               index as int).blt_addr,
                 Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                  KEEPER_BLT_PAGE_LEVEL as int,
                                                  index as int).result.level)).shareability_attribute == Bltentry(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                 KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                 index as int).blt_addr,
                                                                               Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                index as int).result.level)).shareability_attribute)
  && (result != HCI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                               KEEPER_BLT_PAGE_LEVEL as int,
                               index as int).blt_addr,
                 Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                  KEEPER_BLT_PAGE_LEVEL as int,
                                                  index as int).result.level)).output_address == Bltentry(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                 KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                 index as int).blt_addr,
                                                                               Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,
                                                                                                                KEEPER_BLT_PAGE_LEVEL as int,
                                                                                                                index as int).result.level)).output_address)
  && (result == HCI_SUCCESS
    ==> state == Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                         KEEPER_BLT_PAGE_LEVEL as int,
                                         KEEPER_BLT_TREE_PRIMARY as int).blt_addr,
                          Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                           KEEPER_BLT_PAGE_LEVEL as int,
                                                           KEEPER_BLT_TREE_PRIMARY as int).result.level)).state)
  && (result == HCI_SUCCESS
    ==> lbamode == Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                           KEEPER_BLT_PAGE_LEVEL as int,
                                           KEEPER_BLT_TREE_PRIMARY as int).blt_addr,
                            Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,
                                                             KEEPER_BLT_PAGE_LEVEL as int,
                                                             KEEPER_BLT_TREE_PRIMARY as int).result.level)).LBAMODE)
  && ((!(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_BLT) &&
       !(result == HCI_ERROR_BLT_AUX))
    ==> Vaultat(new_s, vd).blt_tree_per_tier == Vaultat(old_s, vd).blt_tree_per_tier)
  && ((!(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_BLT) &&
       !(result == HCI_ERROR_BLT_AUX))
    ==> Vaultat(new_s, vd).aux_tier_count == Vaultat(old_s, vd).aux_tier_count)
}