pub open spec fn hci_blt_map_unprotected_spec(vd: Dpa, lba: Lba, level: int, desc: BlteDescriptor, result: Result<(), HciStatusCode>, old_s: S, new_s: S) -> bool {
  (VdIsEnrollablePhysicalAddress(old_s, vd) ==> result == HCI_SUCCESS)
  && (VdIsInVdState(old_s, vd) ==> result == HCI_SUCCESS)
  && (LevelIsValidBltLevel(old_s, vd, level) ==> result == HCI_SUCCESS)
  && (Level >= 1 ==> result == HCI_SUCCESS)
  && (LbaIsAligned(old_s, vd, lba, level) ==> result == HCI_SUCCESS)
  && (Lba < pow2(RealmAt(old_s, vd).lba_width as nat) ==> result == HCI_SUCCESS)
  && (!IsProtectedLba(old_s, vd, lba) ==> result == HCI_SUCCESS)
  && (DescIsValidBlteDescriptorForUnprotectedLba(old_s, desc) ==> result == HCI_SUCCESS)
  && (OutputAddressInDescIsAligned(old_s, vd, desc, level) ==> result == HCI_SUCCESS)
  && (!ImplFeatures(old_s).feat_bigaddr == FEATURE_TRUE || OutputAddressInDescIsLessThan2to48(old_s, desc) ==> result == HCI_SUCCESS)
  && (VaultUsesXapIndirect(old_s, vd) ==> (IndirectBaseIndexInDesc(old_s, desc) == XAP_NO_ACCESS || IndirectBaseIndexInDesc(old_s, desc) == XAP_RO || IndirectBaseIndexInDesc(old_s, desc) == XAP_WO || IndirectBaseIndexInDesc(old_s, desc) == XAP_RW) ==> result == HCI_SUCCESS)
  && (BltWalkToLbaReachesLevel(old_s, vd, lba, level) ==> result == HCI_SUCCESS)
  && (BltEntryReachedIsInUnassignedPubState(old_s, vd, lba, level) ==> result == HCI_SUCCESS)
  && (result == HCI_SUCCESS ==> BltEntryReachedIsInAssignedPubState(new_s, vd, lba, level))
  && (result == HCI_SUCCESS ==> UnprotectedAttributesInBltEntryEqualDesc(new_s, vd, lba, level, desc))
  && (result == HCI_SUCCESS && VaultUsesXapDirect(old_s, vd) ==> ReadWritePermissionsInBltEntryEqualDesc(new_s, vd, lba, level, desc))
  && (result == HCI_SUCCESS && VaultUsesXapIndirect(old_s, vd) ==> (IndirectBaseIndexInBltEntryEqualDesc(new_s, vd, lba, level, desc) && OverlayIndexInBltEntryIs15(new_s, vd, lba, level)))
  && (result == HCI_SUCCESS ==> OutputAddressInBltEntryEqualDesc(new_s, vd, lba, level, desc))
  && ((!(VdIsEnrollablePhysicalAddress(old_s, vd)) ||
       !(VdIsInVdState(old_s, vd)) ||
       !(LevelIsValidBltLevel(old_s, vd, level)) ||
       !(Level >= 1) ||
       !(LbaIsAligned(old_s, vd, lba, level)) ||
       !(Lba < pow2(RealmAt(old_s, vd).lba_width as nat)) ||
       IsProtectedLba(old_s, vd, lba) ||
       !(DescIsValidBlteDescriptorForUnprotectedLba(old_s, desc)) ||
       !(OutputAddressInDescIsAligned(old_s, vd, desc, level)) ||
       (ImplFeatures(old_s).feat_bigaddr == FEATURE_TRUE) ||
       !(OutputAddressInDescIsLessThan2to48(old_s, desc)) ||
       !((IndirectBaseIndexInDesc(old_s, desc) == XAP_NO_ACCESS || IndirectBaseIndexInDesc(old_s, desc) == XAP_RO || IndirectBaseIndexInDesc(old_s, desc) == XAP_WO || IndirectBaseIndexInDesc(old_s, desc) == XAP_RW)) ||
       !(BltWalkToLbaReachesLevel(old_s, vd, lba, level)) ||
       !(BltEntryReachedIsInUnassignedPubState(old_s, vd, lba, level)))
    ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_BLT ==> BltEntryReachedIsInUnassignedPubState(new_s, vd, lba, level))
  && (result != HCI_SUCCESS
    ==> BltEntryReached(new_s, vd, lba, level) == BltEntryReached(old_s, vd, lba, level))
  && (result != HCI_SUCCESS
    ==> UnprotectedAttributesInBltEntry(new_s, vd, lba, level) == UnprotectedAttributesInBltEntry(old_s, vd, lba, level))
  && (result != HCI_SUCCESS
    ==> ReadWritePermissionsInBltEntry(new_s, vd, lba, level) == ReadWritePermissionsInBltEntry(old_s, vd, lba, level))
  && (result != HCI_SUCCESS
    ==> IndirectBaseIndexInBltEntry(new_s, vd, lba, level) == IndirectBaseIndexInBltEntry(old_s, vd, lba, level))
  && (result != HCI_SUCCESS
    ==> OverlayIndexInBltEntry(new_s, vd, lba, level) == OverlayIndexInBltEntry(old_s, vd, lba, level))
  && (result != HCI_SUCCESS
    ==> OutputAddressInBltEntry(new_s, vd, lba, level) == OutputAddressInBltEntry(old_s, vd, lba, level))
}