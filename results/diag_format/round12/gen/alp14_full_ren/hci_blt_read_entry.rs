pub open spec fn hci_blt_read_entry_spec(vd: UInt64, lba: UInt64, level: int, result: Result<(), HciStatusCode>, walk_level: int, state: UInt8, desc: [UInt8; 64], lbamode: UInt8, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_INPUT && (vd % SIZE_OF_EXTENT(old_s) != 0))
  && (result == HCI_ERROR_INPUT && !is_enrollable_physical_address(old_s, vd))
  && (result == HCI_ERROR_INPUT && !ExtentAt(old_s, vd).in_vd_state())
  && (result == HCI_ERROR_INPUT && !(level >= 0 && level < VdAt(old_s, vd).blt_level_count))
  && (result == HCI_ERROR_INPUT && (lba % (1 << (VdAt(old_s, vd).lba_width as nat)) != 0))
  && (result == HCI_ERROR_INPUT && lba >= pow2(VdAt(old_s, vd).lba_width))
  && ((!(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT))
    ==> walk_level == VdAt(new_s, vd).blt_level_count)
  && (result == HCI_SUCCESS
    ==> desc[0] == 0)
  && (result == HCI_SUCCESS
    ==> desc[1] == 0)
  && (result == HCI_SUCCESS
    ==> desc[2] == 0)
  && (result == HCI_SUCCESS
    ==> desc[3] == 0)
  && (result == HCI_SUCCESS
    ==> desc[4] == 0)
  && (result == HCI_SUCCESS
    ==> desc[5] == 0)
  && (result == HCI_SUCCESS
    ==> desc[6] == 0)
  && (result == HCI_SUCCESS
    ==> desc[7] == 0)
  && (result == HCI_SUCCESS
    ==> desc[8] == 0)
  && (result == HCI_SUCCESS
    ==> desc[9] == 0)
  && (result == HCI_SUCCESS
    ==> desc[10] == 0)
  && (result == HCI_SUCCESS
    ==> desc[11] == 0)
  && (result == HCI_SUCCESS
    ==> desc[12] == 0)
  && (result == HCI_SUCCESS
    ==> desc[13] == 0)
  && (result == HCI_SUCCESS
    ==> desc[14] == 0)
  && (result == HCI_SUCCESS
    ==> desc[15] == 0)
  && (result == HCI_SUCCESS
    ==> desc[16] == 0)
  && (result == HCI_SUCCESS
    ==> desc[17] == 0)
  && (result == HCI_SUCCESS
    ==> desc[18] == 0)
  && (result == HCI_SUCCESS
    ==> desc[19] == 0)
  && (result == HCI_SUCCESS
    ==> desc[20] == 0)
  && (result == HCI_SUCCESS
    ==> desc[21] == 0)
  && (result == HCI_SUCCESS
    ==> desc[22] == 0)
  && (result == HCI_SUCCESS
    ==> desc[23] == 0)
  && (result == HCI_SUCCESS
    ==> desc[24] == 0)
  && (result == HCI_SUCCESS
    ==> desc[25] == 0)
  && (result == HCI_SUCCESS
    ==> desc[26] == 0)
  && (result == HCI_SUCCESS
    ==> desc[27] == 0)
  && (result == HCI_SUCCESS
    ==> desc[28] == 0)
  && (result == HCI_SUCCESS
    ==> desc[29] == 0)
  && (result == HCI_SUCCESS
    ==> desc[30] == 0)
  && (result == HCI_SUCCESS
    ==> desc[31] == 0)
  && (result == HCI_SUCCESS
    ==> desc[32] == 0)
  && (result == HCI_SUCCESS
    ==> desc[33] == 0)
  && (result == HCI_SUCCESS
    ==> desc[34] == 0)
  && (result == HCI_SUCCESS
    ==> desc[35] == 0)
  && (result == HCI_SUCCESS
    ==> desc[36] == 0)
  && (result == HCI_SUCCESS
    ==> desc[37] == 0)
  && (result == HCI_SUCCESS
    ==> desc[38] == 0)
  && (result == HCI_SUCCESS
    ==> desc[39] == 0)
  && (result == HCI_SUCCESS
    ==> desc[40] == 0)
  && (result == HCI_SUCCESS
    ==> desc[41] == 0)
  && (result == HCI_SUCCESS
    ==> desc[42] == 0)
  && (result == HCI_SUCCESS
    ==> desc[43] == 0)
  && (result == HCI_SUCCESS
    ==> desc[44] == 0)
  && (result == HCI_SUCCESS
    ==> desc[45] == 0)
  && (result == HCI_SUCCESS
    ==> desc[46] == 0)
  && (result == HCI_SUCCESS
    ==> desc[47] == 0)
  && (result == HCI_SUCCESS
    ==> desc[48] == 0)
  && (result == HCI_SUCCESS
    ==> desc[49] == 0)
  && (result == HCI_SUCCESS
    ==> desc[50] == 0)
  && (result == HCI_SUCCESS
    ==> desc[51] == 0)
  && (result == HCI_SUCCESS
    ==> desc[52] == 0)
  && (result == HCI_SUCCESS
    ==> desc[53] == 0)
  && (result == HCI_SUCCESS
    ==> desc[54] == 0)
  && (result == HCI_SUCCESS
    ==> desc[55] == 0)
  && (result == HCI_SUCCESS
    ==> desc[56] == 0)
  && (result == HCI_SUCCESS
    ==> desc[57] == 0)
  && (result == HCI_SUCCESS
    ==> desc[58] == 0)
  && (result == HCI_SUCCESS
    ==> desc[59] == 0)
  && (result == HCI_SUCCESS
    ==> desc[60] == 0)
  && (result == HCI_SUCCESS
    ==> desc[61] == 0)
  && (result == HCI_SUCCESS
    ==> desc[62] == 0)
  && (result == HCI_SUCCESS
    ==> desc[63] == 0)
  && (result == HCI_SUCCESS
    ==> lbamode == 0)
  && ((!(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT))
    ==> VdAt(new_s, vd) == VdAt(old_s, vd))
  && (result == HCI_SUCCESS
    ==> (state == UNASSIGNED || state == UNASSIGNED_PUB)
     ==> BltEntryAt(new_s, vd, walk_level as int).attr_unprot == 0)
  && (result == HCI_SUCCESS
    ==> (state == UNASSIGNED || state == UNASSIGNED_PUB)
     ==> BltEntryAt(new_s, vd, walk_level as int).xap_base_index == XAP_NO_ACCESS)
  && (result == HCI_SUCCESS
    ==> (state == UNASSIGNED || state == UNASSIGNED_PUB)
     ==> BltEntryAt(new_s, vd, walk_level as int).xap_overlay_index == 0)
  && (result == HCI_SUCCESS
    ==> (state == UNASSIGNED || state == UNASSIGNED_PUB)
     ==> BltEntryAt(new_s, vd, walk_level as int).s2_read_perm == KEEPER_FALSE)
  && (result == HCI_SUCCESS
    ==> (state == UNASSIGNED || state == UNASSIGNED_PUB)
     ==> BltEntryAt(new_s, vd, walk_level as int).s2_write_perm == KEEPER_FALSE)
  && (result == HCI_SUCCESS
    ==> (state == UNASSIGNED || state == UNASSIGNED_PUB)
     ==> BltEntryAt(new_s, vd, walk_level as int).address == 0)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED || state == ASSIGNED_DEV || state == ASSIGNED_VXLATOR || state == TABLE)
     ==> BltEntryAt(new_s, vd, walk_level as int).attr_unprot == 0)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED || state == ASSIGNED_DEV || state == ASSIGNED_VXLATOR || state == TABLE)
     ==> BltEntryAt(new_s, vd, walk_level as int).xap_base_index == XAP_NO_ACCESS)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED || state == ASSIGNED_DEV || state == ASSIGNED_VXLATOR || state == TABLE)
     ==> BltEntryAt(new_s, vd, walk_level as int).xap_overlay_index == 0)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED || state == ASSIGNED_DEV || state == ASSIGNED_VXLATOR || state == TABLE)
     ==> BltEntryAt(new_s, vd, walk_level as int).s2_read_perm == KEEPER_FALSE)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED || state == ASSIGNED_DEV || state == ASSIGNED_VXLATOR || state == TABLE)
     ==> BltEntryAt(new_s, vd, walk_level as int).s2_write_perm == KEEPER_FALSE)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED || state == ASSIGNED_DEV || state == ASSIGNED_VXLATOR || state == TABLE)
     ==> BltEntryAt(new_s, vd, walk_level as int).address == BltEntryAt(new_s, vd, walk_level as int).address)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_PUB)
     ==> BltEntryAt(new_s, vd, walk_level as int).attr_unprot == BltEntryAt(new_s, vd, walk_level as int).attr_unprot)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_PUB)
     ==> BltEntryAt(new_s, vd, walk_level as int).xap_base_index == BltEntryAt(new_s, vd, walk_level as int).xap_base_index)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_PUB)
     ==> BltEntryAt(new_s, vd, walk_level as int).xap_overlay_index == 0)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_PUB)
     ==> BltEntryAt(new_s, vd, walk_level as int).s2_read_perm == BltEntryAt(new_s, vd, walk_level as int).s2_read_perm)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_PUB)
     ==> BltEntryAt(new_s, vd, walk_level as int).s2_write_perm == BltEntryAt(new_s, vd, walk_level as int).s2_write_perm)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_PUB)
     ==> BltEntryAt(new_s, vd, walk_level as int).address == BltEntryAt(new_s, vd, walk_level as int).address)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_DEV)
     ==> BltEntryAt(new_s, vd, walk_level as int).attr_unprot == 0)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_DEV)
     ==> BltEntryAt(new_s, vd, walk_level as int).xap_base_index == XAP_NO_ACCESS)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_DEV)
     ==> BltEntryAt(new_s, vd, walk_level as int).xap_overlay_index == 0)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_DEV)
     ==> BltEntryAt(new_s, vd, walk_level as int).s2_read_perm == KEEPER_FALSE)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_DEV)
     ==> BltEntryAt(new_s, vd, walk_level as int).s2_write_perm == KEEPER_FALSE)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_DEV)
     ==> BltEntryAt(new_s, vd, walk_level as int).address == BltEntryAt(new_s, vd, walk_level as int).address)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_VXLATOR)
     ==> BltEntryAt(new_s, vd, walk_level as int).attr_unprot == 0)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_VXLATOR)
     ==> BltEntryAt(new_s, vd, walk_level as int).xap_base_index == XAP_NO_ACCESS)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_VXLATOR)
     ==> BltEntryAt(new_s, vd, walk_level as int).xap_overlay_index == 0)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_VXLATOR)
     ==> BltEntryAt(new_s, vd, walk_level as int).s2_read_perm == KEEPER_FALSE)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_VXLATOR)
     ==> BltEntryAt(new_s, vd, walk_level as int).s2_write_perm == KEEPER_FALSE)
  && (result == HCI_SUCCESS
    ==> (state == ASSIGNED_VXLATOR)
     ==> BltEntryAt(new_s, vd, walk_level as int).address == BltEntryAt(new_s, vd, walk_level as int).address)
  && (result == HCI_SUCCESS
    ==> (state == UNASSIGNED || state == ASSIGNED)
     ==> lbamode == BltEntryAt(new_s, vd, walk_level as int).lbamode)
  && (result == HCI_SUCCESS
    ==> (state == UNASSIGNED_PUB || state == ASSIGNED_PUB)
     ==> lbamode == HCI_EMPTY)
  && ((!(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT))
    ==> BltEntryAt(new_s, vd, walk_level as int) == BltEntryAt(old_s, vd, walk_level as int))
  && (result != HCI_SUCCESS
    ==> VdAt(new_s, vd) == VdAt(old_s, vd))
  && (result != HCI_SUCCESS
    ==> BltEntryAt(new_s, vd, walk_level as int) == BltEntryAt(old_s, vd, walk_level as int))
}