pub open spec fn pinctrl_request__3_11_2_9_spec(identifier: UInt32, flags: UInt32, result: Result<Int32, PsmmStatusCode>, old_s: S, new_s: S) -> bool {
  (flags[31..2] != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (flags[1..0] != 0 && flags[1..0] != 1 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidPinOrGroup(old_s, identifier, flags[1..0]) ==> ResultEqual(result, NOT_FOUND))
  && (!AgentMayRequestPinOrGroup(old_s, caller, identifier, flags[1..0]) ==> ResultEqual(result, DENIED))
  && (IsUnderExclusiveControl(old_s, identifier, flags[1..0]) && ExclusiveOwner(old_s, identifier, flags[1..0]) != caller ==> ResultEqual(result, IN_USE))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> ExclusiveOwner(new_s, identifier, flags[1..0]) == caller)
  && ((!(flags[31..2] != 0) &&
       !(flags[1..0] != 0 && flags[1..0] != 1) &&
       IsValidPinOrGroup(old_s, identifier, flags[1..0]) &&
       AgentMayRequestPinOrGroup(old_s, caller, identifier, flags[1..0]) &&
       !(IsUnderExclusiveControl(old_s, identifier, flags[1..0]) && ExclusiveOwner(old_s, identifier, flags[1..0]) != caller))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> ExclusiveOwner(new_s, identifier, flags[1..0]) == ExclusiveOwner(old_s, identifier, flags[1..0]))
}