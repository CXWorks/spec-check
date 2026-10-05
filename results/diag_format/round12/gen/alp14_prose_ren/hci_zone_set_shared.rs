pub open spec fn hci_zone_set_shared_spec(zoneid: Bits64, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_INPUT && zoneid > max_zone_id(old_s) ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT && Zonestate(old_s, zoneid) != ZONE_STATE_PRIVATE_UNASSIGNED ==> result == HCI_ERROR_INPUT)
  && (result == HCI_SUCCESS ==> Zonestate(new_s, zoneid) == ZONE_STATE_SHARED)
  && ((!(result == HCI_ERROR_INPUT && zoneid > max_zone_id(old_s)) &&
       !(result == HCI_ERROR_INPUT && Zonestate(old_s, zoneid) != ZONE_STATE_PRIVATE_UNASSIGNED))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> Zonestate(new_s, zoneid) == Zonestate(old_s, zoneid))
}