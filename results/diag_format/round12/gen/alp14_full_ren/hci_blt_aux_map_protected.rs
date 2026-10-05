pub open spec fn hci_blt_aux_map_protected_spec(vd: Address, lba: UInt64, index: UInt64, result: Result<(), HciStatusCode>, state: UInt8, lbamode: UInt8, old_s: S, new_s: S) -> bool {
  (vd % EXTENT_SIZE == 0 ==> result == HCI_SUCCESS)
  && (is_enrollable_address(vd) ==> result == HCI_SUCCESS)
  && (ExtentAt(old_s, vd).vd_state == VD_STATE ==> result == HCI_SUCCESS)
  && (lba % EXTENT_SIZE == 0 ==> result == HCI_SUCCESS)
  && (is_protected_lba(old_s, vd, lba) ==> result == HCI_SUCCESS)
  && (ImplFeatures(old_s).blt_tree_per_tier == FEATURE_FALSE ==> result == HCI_ERROR_INPUT)
  && (result == HCI_SUCCESS && index == KEEPER_BLT_TREE_PRIMARY ==> result == HCI_ERROR_INPUT)
  && (result == HCI_SUCCESS && index > VaultAt(old_s, vd).aux_tier_count ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_BLT && RttWalk(old_s, VaultAt(old_s, vd).keeper_blt_tree_primary, lba as int).state != ASSIGNED && RttWalk(old_s, VaultAt(old_s, vd).keeper_blt_tree_primary, lba as int).state != ASSIGNED_DEV && RttWalk(old_s, VaultAt(old_s, vd).keeper_blt_tree_primary, lba as int).state != ASSIGNED_VXLATOR ==> result == HCI_SUCCESS)
  && (result == HCI_ERROR_BLT && RttWalk(old_s, VaultAt(old_s, vd).keeper_blt_tree_primary, lba as int).state == ASSIGNED && RttWalk(old_s, VaultAt(old_s, vd).keeper_blt_tree_primary, lba as int).lbamode != RAM ==> result == HCI_SUCCESS)
  && (result == HCI_ERROR_BLT && RttWalk(old_s, VaultAt(old_s, vd).keeper_blt_tree_primary, lba as int).state == ASSIGNED_DEV && RttWalk(old_s, VaultAt(old_s, vd).keeper_blt_tree_primary, lba as int).lbamode != DEV ==> result == HCI_SUCCESS)
  && (result == HCI_ERROR_BLT_AUX && RttWalk(old_s, VaultAt(old_s, vd).aux_blt_trees[index], lba as int).state == AUX_DESTROYED ==> result == HCI_SUCCESS)
  && (result == HCI_ERROR_BLT_AUX && RttWalk(old_s, VaultAt(old_s, vd).keeper_blt_tree_primary, lba as int).level > RttWalk(old_s, VaultAt(old_s, vd).aux_blt_trees[index], lba as int).level ==> result == HCI_SUCCESS)
  && (result == HCI_SUCCESS ==> RttWalk(new_s, VaultAt(new_s, vd).aux_blt_trees[index], lba as int).state == ASSIGNED)
  && (result == HCI_SUCCESS ==> RttWalk(new_s, VaultAt(new_s, vd).aux_blt_trees[index], lba as int).protection == RttWalk(new_s, VaultAt(new_s, vd).keeper_blt_tree_primary, lba as int).protection)
  && (result == HCI_SUCCESS ==> RttWalk(new_s, VaultAt(new_s, vd).aux_blt_trees[index], lba as int).shareability == RttWalk(new_s, VaultAt(new_s, vd).keeper_blt_tree_primary, lba as int).shareability)
  && ((!(vd % EXTENT_SIZE == 0) ||
       !(is_enrollable_address(vd)) ||
       !(ExtentAt(old_s, vd).vd_state == VD_STATE) ||
       !(lba % EXTENT_SIZE == 0) ||
       !(is_protected_lba(old_s, vd, lba)) ||
       !(ImplFeatures(old_s).blt_tree_per_tier == FEATURE_FALSE) ||
       !(result == HCI_SUCCESS && index == KEEPER_BLT_TREE_PRIMARY) ||
       !(result == HCI_SUCCESS && index > VaultAt(old_s, vd).aux_tier_count))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> RttWalk(new_s, VaultAt(new_s, vd).aux_blt_trees[index], lba as int).state == RttWalk(old_s, VaultAt(old_s, vd).aux_blt_trees[index], lba as int).state)
  && (result != HCI_SUCCESS
    ==> RttWalk(new_s, VaultAt(new_s, vd).aux_blt_trees[index], lba as int).protection == RttWalk(old_s, VaultAt(old_s, vd).aux_blt_trees[index], lba as int).protection)
  && (result != HCI_SUCCESS
    ==> RttWalk(new_s, VaultAt(new_s, vd).aux_blt_trees[index], lba as int).shareability == RttWalk(old_s, VaultAt(old_s, vd).aux_blt_trees[index], lba as int).shareability)
  && (RttWalk(new_s, VaultAt(new_s, vd).aux_blt_trees[index], lba as int).output_addr == RttWalk(new_s, VaultAt(new_s, vd).keeper_blt_tree_primary, lba as int).output_addr ==> result == HCI_SUCCESS)
}