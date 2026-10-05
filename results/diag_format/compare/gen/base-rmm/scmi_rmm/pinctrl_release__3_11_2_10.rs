pub open spec fn pinctrl_release__3_11_2_10_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPinOrGroup(old_s, identifier(old_s), selector(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!AreValidParameters(old_s, identifier(old_s), flags(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> !HasExclusiveControl(old_s, caller(old_s), identifier(old_s), selector(old_s)))
    && (ResultEqual(result, SUCCESS) ==> !ExclusiveControl(old_s, identifier(old_s), selector(old_s)))
}