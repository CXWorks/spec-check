pub open spec fn hci_blt_map_unprotected_spec(vd: Address, lba: Address, level: Int64, desc: Bits64, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (Bltdescriptordecode(old_s, desc,Vaultat(old_s, vd).blt_xap_encoding) != valid BLT entry descriptor for an Unprotected LBA ==> result == HCI_ERROR_INPUT)
  && (vd % size of a Extent != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, vd) ==> result == HCI_ERROR_INPUT)
  && (Extent_at(old_s, vd) not in the VD state ==> result == HCI_ERROR_INPUT)
  && (!is_valid_BLT_level_for_Vault(old_s, level) || level < 1 ==> result == HCI_ERROR_INPUT)
  && (output address in desc (the addr field) % size of the address range that an BLT entry at level maps != 0 ==> result == HCI_ERROR_INPUT)
  && (!feat_bigaddr(old_s) == FEATURE_FALSE && output address in desc >= 2^48 ==> result == HCI_ERROR_INPUT)
  && (lba % size of the address range that an BLT entry at level maps != 0 ==> result == HCI_ERROR_INPUT)
  && (lba >= 2 to the power of the Vault's LBA width || lba is a Protected LBA of the Vault ==> result == HCI_ERROR_INPUT)
  && (Vaultat(old_s, vd).uses the XAP_INDIRECT stage 2 access permission encoding && indirect base index in desc not in [XAP_NO_ACCESS, XAP_RO, XAP_WO, XAP_RW] ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_BLT ==> Bltwalk(new_s, Vaultat(new_s, vd), lba, level as int,KEEPER_BLT_TREE_PRIMARY).level < level)
  && (result == HCI_ERROR_BLT ==> Bltdescriptordecode(new_s, Bltdescriptordecode(new_s, desc,Vaultat(new_s, vd).blt_xap_encoding),Vaultat(new_s, vd).blt_xap_encoding) not in the UNASSIGNED_PUB state)
  && (result == HCI_SUCCESS ==> Bltdescriptordecode(new_s, Bltdescriptordecode(new_s, desc,Vaultat(new_s, vd).blt_xap_encoding),Vaultat(new_s, vd).blt_xap_encoding) in the ASSIGNED_PUB state)
  && (result == HCI_SUCCESS ==> Unprotected attributes of that BLT entry == Unprotected attributes in desc)
  && (result == HCI_SUCCESS && Vaultat(old_s, vd).uses the XAP_DIRECT stage 2 access permission encoding ==> read and write permissions of the BLT entry == read and write permissions in desc)
  && (result == HCI_SUCCESS && Vaultat(old_s, vd).uses the XAP_INDIRECT stage 2 access permission encoding ==> indirect base index of the BLT entry == indirect base index in desc && overlay index of the BLT entry == 15)
  && (result == HCI_SUCCESS ==> output address of the BLT entry == output address in desc)
  && ((!(Bltdescriptordecode(old_s, desc,Vaultat(old_s, vd).blt_xap_encoding) != valid BLT entry descriptor for an Unprotected LBA) &&
       !(vd % size of a Extent != 0) &&
       is_enrollable_physical_address(old_s, vd) &&
       !(Extent_at(old_s, vd) not in the VD state) &&
       (is_valid_BLT_level_for_Vault(old_s, level) && level >= 1) &&
       !(output address in desc (the addr field) % size of the address range that an BLT entry at level maps != 0) &&
       (feat_bigaddr(old_s) || !(output address in desc >= 2^48)) &&
       !(lba % size of the address range that an BLT entry at level maps != 0) &&
       !((lba >= 2 to the power of the Vault's LBA width) || (lba is a Protected LBA of the Vault)) &&
       !((Vaultat(old_s, vd).uses the XAP_INDIRECT stage 2 access permission encoding) && (indirect base index in desc not in [XAP_NO_ACCESS, XAP_RO, XAP_WO, XAP_RW])))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> Bltdescriptordecode(new_s, Bltdescriptordecode(new_s, desc,Vaultat(new_s, vd).blt_xap_encoding),Vaultat(new_s, vd).blt_xap_encoding) == Bltdescriptordecode(old_s, desc,Vaultat(old_s, vd).blt_xap_encoding))
  && (result != HCI_SUCCESS
    ==> Bltwalk(new_s, Vaultat(new_s, vd), lba, level as int,KEEPER_BLT_TREE_PRIMARY).level == Bltwalk(old_s, Vaultat(old_s, vd), lba, level as int,KEEPER_BLT_TREE_PRIMARY).level)
}