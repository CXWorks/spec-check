pub open spec fn clock_attributes__3_6_2_5_spec(clock_id: UInt32, status: Int32, attributes: UInt32, clock_name: [UInt8; 16], clock_enable_delay: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidClockId(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> attributes[31] == (ClockSupportsRateChangeNotifications(old_s, clock_id) ? 1 : 0))
  && (ResultEqual(status, SUCCESS) ==> attributes[30] == (ClockSupportsRateChangeRequestedNotifications(old_s, clock_id) ? 1 : 0))
  && (ResultEqual(status, SUCCESS) ==> attributes[29] == (ClockNameLength(old_s, clock_id) > 16 ? 1 : 0))
  && (ResultEqual(status, SUCCESS) ==> attributes[28] == (ClockAdvertisesParentIds(old_s, clock_id) ? 1 : 0))
  && (ResultEqual(status, SUCCESS) ==> attributes[27] == (ClockSupportsExtendedConfig(old_s, clock_id) ? 1 : 0))
  && (ResultEqual(status, SUCCESS) ==> attributes[26:2] == 0)
  && (ResultEqual(status, SUCCESS) ==> attributes[1] == (ClockHasRestrictions(old_s, clock_id) && IsClockGetPermissionsImplemented(old_s) ? 1 : 0))
  && (ResultEqual(status, SUCCESS) ==> attributes[0] == (ClockIsEnabled(old_s, clock_id) ? 1 : 0))
  && (ResultEqual(status, SUCCESS) ==> clock_name == ClockNameString(old_s, clock_id))
  && (ResultEqual(status, SUCCESS) ==> clock_enable_delay == ClockWorstCaseEnableDelayUs(old_s, clock_id))
  && ((IsValidClockId(old_s, clock_id))
    ==> ResultEqual(status, SUCCESS))
  && (result == SUCCESS
    ==> attributes[31] == (ClockSupportsRateChangeNotifications(old_s, clock_id) ? 1 : 0))
  && (result == SUCCESS
    ==> attributes[30] == (ClockSupportsRateChangeRequestedNotifications(old_s, clock_id) ? 1 : 0))
  && (result == SUCCESS
    ==> attributes[29] == (ClockNameLength(old_s, clock_id) > 16 ? 1 : 0))
  && (result == SUCCESS
    ==> attributes[28] == (ClockAdvertisesParentIds(old_s, clock_id) ? 1 : 0))
  && (result == SUCCESS
    ==> attributes[27] == (ClockSupportsExtendedConfig(old_s, clock_id) ? 1 : 0))
  && (result == SUCCESS
    ==> attributes[26:2] == 0)
  && (result == SUCCESS
    ==> attributes[1] == (ClockHasRestrictions(old_s, clock_id) && IsClockGetPermissionsImplemented(old_s) ? 1 : 0))
  && (result == SUCCESS
    ==> attributes[0] == (ClockIsEnabled(old_s, clock_id) ? 1 : 0))
  && (result == SUCCESS
    ==> clock_name == ClockNameString(old_s, clock_id))
  && (result == SUCCESS
    ==> clock_enable_delay == ClockWorstCaseEnableDelayUs(old_s, clock_id))
  && (((!(IsValidClockId(old_s, clock_id)))
       ==> ResultEqual(status, NOT_FOUND))
    ==> attributes[31] == (ClockSupportsRateChangeNotifications(old_s, clock_id) ? 1 : 0))
  && (((!(IsValidClockId(old_s, clock_id)))
       ==> ResultEqual(status, NOT_FOUND))
    ==> attributes[30] == (ClockSupportsRateChangeRequestedNotifications(old_s, clock_id) ? 1 : 0))
  && (((!(IsValidClockId(old_s, clock_id)))
       ==> ResultEqual(status, NOT_FOUND))
    ==> attributes[29] == (ClockNameLength(old_s, clock_id) > 16 ? 1 : 0))
  && (((!(IsValidClockId(old_s, clock_id)))
       ==> ResultEqual(status, NOT_FOUND))
    ==> attributes[28] == (ClockAdvertisesParentIds(old_s, clock_id) ? 1 : 0))
  && (((!(IsValidClockId(old_s, clock_id)))
       ==> ResultEqual(status, NOT_FOUND))
    ==> attributes[27] == (ClockSupportsExtendedConfig(old_s, clock_id) ? 1 : 0))
  && (((!(IsValidClockId(old_s, clock_id)))
       ==> ResultEqual(status, NOT_FOUND))
    ==> attributes[26:2] == 0)
  && (((!(IsValidClockId(old_s, clock_id)))
       ==> ResultEqual(status, NOT_FOUND))
    ==> attributes[1] == (ClockHasRestrictions(old_s, clock_id) && IsClockGetPermissionsImplemented(old_s) ? 1 : 0))
  && (((!(IsValidClockId(old_s, clock_id)))
       ==> ResultEqual(status, NOT_FOUND))
    ==> attributes[0] == (ClockIsEnabled(old_s, clock_id) ? 1 : 0))
  && (((!(IsValidClockId(old_s, clock_id)))
       ==> ResultEqual(status, NOT_FOUND))
    ==> clock_name == ClockNameString(old_s, clock_id))
  && (((!(IsValidClockId(old_s, clock_id)))
       ==> ResultEqual(status, NOT_FOUND))
    ==> clock_enable_delay == ClockWorstCaseEnableDelayUs(old_s, clock_id))
  && (result != SUCCESS
    ==> attributes[31] == (ClockSupportsRateChangeNotifications(old_s, clock_id) ? 1 : 0))
  && (result != SUCCESS
    ==> attributes[30] == (ClockSupportsRateChangeRequestedNotifications(old_s, clock_id) ? 1 : 0))
  && (result != SUCCESS
    ==> attributes[29] == (ClockNameLength(old_s, clock_id) > 16 ? 1 : 0))
  && (result != SUCCESS
    ==> attributes[28] == (ClockAdvertisesParentIds(old_s, clock_id) ? 1 : 0))
  && (result != SUCCESS
    ==> attributes[27] == (ClockSupportsExtendedConfig(old_s, clock_id) ? 1 : 0))
  && (result != SUCCESS
    ==> attributes[26:2] == 0)
  && (result != SUCCESS
    ==> attributes[1] == (ClockHasRestrictions(old_s, clock_id) && IsClockGetPermissionsImplemented(old_s) ? 1 : 0))
  && (result != SUCCESS
    ==> attributes[0] == (ClockIsEnabled(old_s, clock_id) ? 1 : 0))
  && (result != SUCCESS
    ==> clock_name == ClockNameString(old_s, clock_id))
  && (result != SUCCESS
    ==> clock_enable_delay == ClockWorstCaseEnableDelayUs(old_s, clock_id))
}