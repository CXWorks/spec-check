pub open spec fn pinctrl_release__3_11_2_10_spec(identifier: UInt32, flags: Flags, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPinOrGroup(old_s, identifier, flags.selector) ==> ResultEqual(status, NOT_FOUND))
  && (!AreValidParameters(old_s, identifier, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> !HasExclusiveControl(new_s, caller, identifier, flags.selector))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && ((IsValidPinOrGroup(old_s, identifier, flags.selector) &&
       AreValidParameters(old_s, identifier, flags))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> HasExclusiveControl(new_s, caller, identifier, flags.selector))
}