pub open spec fn pinctrl_release__3_11_2_10_spec(identifier: uint32, flags: uint32, status: int32, released: bool, old_s: S, new_s: S) -> bool {
  ((flags & 7) != 0 ==> ResultEqual(status, INVALID_PARAMETERS))
  && ((flags & 3) > 1 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsValidPinOrGroup(old_s, identifier, (flags & 3)) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> !HasExclusiveControl(new_s, CallingAgent(new_s), identifier, (flags & 3)))
  && ((!( (flags & 7) != 0) &&
       !( (flags & 3) > 1) &&
       IsValidPinOrGroup(old_s, identifier, (flags & 3)))
    ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, INVALID_PARAMETERS)
    ==> HasExclusiveControl(new_s, CallingAgent(new_s), identifier, (flags & 3)))
  && (ResultEqual(status, NOT_FOUND)
    ==> HasExclusiveControl(new_s, CallingAgent(new_s), identifier, (flags & 3)))
  && (result != SUCCESS
    ==> HasExclusiveControl(new_s, CallingAgent(new_s), identifier, (flags & 3)))
}