pub open spec fn pinctrl_request__3_11_2_9_spec(identifier: UInt32, flags: UInt32, status: Int32, owner: AgentId, old_s: S, new_s: S) -> bool {
  ((flags & 7) != 0 ==> ResultEqual(status, INVALID_PARAMETERS))
  && ((flags & 3) != 0 && (flags & 3) != 1 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsValidPinOrGroup(old_s, identifier, (flags & 3) as int) ==> ResultEqual(status, NOT_FOUND))
  && (!AgentMayRequestPinOrGroup(old_s, CallingAgent(old_s), identifier, (flags & 3) as int) ==> ResultEqual(status, DENIED))
  && (IsUnderExclusiveControlOfOtherAgent(old_s, identifier, (flags & 3) as int, CallingAgent(old_s)) ==> ResultEqual(status, IN_USE))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> PinOrGroupAt(new_s, identifier, (flags & 3) as int).owner == CallingAgent(new_s))
  && (ResultEqual(status, SUCCESS) ==> (forall (a: AgentId), a != CallingAgent(new_s) ==> !PinOrGroupAvailableTo(new_s, a, identifier, (flags & 3) as int)))
  && ((!( (flags & 7) != 0) &&
       !((flags & 3) != 0 && (flags & 3) != 1) &&
       IsValidPinOrGroup(old_s, identifier, (flags & 3) as int) &&
       AgentMayRequestPinOrGroup(old_s, CallingAgent(old_s), identifier, (flags & 3) as int) &&
       !IsUnderExclusiveControlOfOtherAgent(old_s, identifier, (flags & 3) as int, CallingAgent(old_s)))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PinOrGroupAt(new_s, identifier, (flags & 3) as int).owner == PinOrGroupAt(old_s, identifier, (flags & 3) as int).owner)
  && (result != SUCCESS
    ==> PinOrGroupAt(new_s, identifier, (flags & 3) as int).owner == CallingAgent(new_s))
}