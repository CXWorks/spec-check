pub open spec fn pinctrl_name_get__3_11_2_11_spec(result: Int32, flags: UInt32, name: UInt8[64], old_s: S, new_s: S) -> bool {
    (!PinctrlEntityExists(Bits(flags, 1, 0), Bits(identifier, 15, 0)) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (name == ExtendedName(Bits(flags, 1, 0), Bits(identifier, 15, 0)) && IsNullTerminatedAscii(name, 64) && flags == 0))
    && (old_s == new_s)
}