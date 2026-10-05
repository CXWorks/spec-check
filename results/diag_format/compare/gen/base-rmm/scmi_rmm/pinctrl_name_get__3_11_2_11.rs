pub open spec fn pinctrl_name_get__3_11_2_11_spec(result: Int32, name: UInt8[64], flags: UInt32, old_s: S, new_s: S) -> bool {
    (!PinctrlEntityExists(selector, identifier) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (ResultEqual(result, SUCCESS) && name == PinctrlExtendedName(selector, identifier) && IsNullTerminatedAscii(name, 64) && flags == 0))
    && (old_s == new_s)
}