pub open spec fn hci_zone_set_shared_spec(zoneid: UInt64, result: HciCommandReturnCode, old_s: S, new_s: S) -> bool {
  (zoneid > ZONEID_MAX(old_s) ==> result == HCI_ERROR_INPUT)
  && (ZONE_STATE(RealmAt(old_s, 0).zones[zoneid as int]) != ZONE_STATE_PRIVATE_UNASSIGNED ==> result == HCI_ERROR_INPUT)
  && (result == HCI_SUCCESS ==> ZONE_STATE(RealmAt(new_s, 0).zones[zoneid as int]) == ZONE_STATE_SHARED)
  && ((!(zoneid > ZONEID_MAX(old_s)) &&
       ZONE_STATE(RealmAt(old_s, 0).zones[zoneid as int]) == ZONE_STATE_PRIVATE_UNASSIGNED)
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> ZONE_STATE(RealmAt(new_s, 0).zones[zoneid as int]) == ZONE_STATE(RealmAt(old_s, 0).zones[zoneid as int]))
}