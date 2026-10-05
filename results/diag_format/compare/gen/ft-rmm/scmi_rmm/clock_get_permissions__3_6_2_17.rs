pub open spec fn clock_get_permissions__3_6_2_17_spec(clock_id: uint32, status: int32, permissions: uint32, old_s: S, new_s: S) -> bool {
  (!IsValidClockId(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsRequestSupported(old_s, CLOCK_GET_PERMISSIONS) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> permissions[31] == (AgentCanChangeClockState(old_s, clock_id) ? 1 : 0))
  && (ResultEqual(status, SUCCESS) ==> permissions[31] == 0 ==> CLOCK_CONFIG_SET returns DENIED for attempts to change the state of clock_id)
  && (ResultEqual(status, SUCCESS) ==> permissions[30] == (AgentCanChangeClockParent(old_s, clock_id) ? 1 : 0))
  && (ResultEqual(status, SUCCESS) ==> permissions[30] == 0 ==> CLOCK_PARENT_SET returns DENIED for clock_id)
  && (ResultEqual(status, SUCCESS) ==> permissions[29] == (AgentCanChangeClockRate(old_s, clock_id) ? 1 : 0))
  && (ResultEqual(status, SUCCESS) ==> permissions[29] == 0 ==> CLOCK_RATE_SET returns DENIED for clock_id)
  && (ResultEqual(status, SUCCESS) ==> permissions[28:0] == 0)
  && ((!(IsValidClockId(old_s, clock_id)) &&
       IsRequestSupported(old_s, CLOCK_GET_PERMISSIONS))
    ==> ResultEqual(status, SUCCESS))
  && (result == SUCCESS
    ==> permissions[31] == (AgentCanChangeClockState(old_s, clock_id) ? 1 : 0))
  && (result == SUCCESS
    ==> permissions[30] == (AgentCanChangeClockParent(old_s, clock_id) ? 1 : 0))
  && (result == SUCCESS
    ==> permissions[29] == (AgentCanChangeClockRate(old_s, clock_id) ? 1 : 0))
  && (result == SUCCESS
    ==> permissions[28:0] == 0)
}