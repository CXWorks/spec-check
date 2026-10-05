pub open spec fn clock_name_get__3_6_2_11_spec(result: Int32, flags: UInt32, name: UInt8[64], old_s: S, new_s: S) -> bool {
    (!ClockExists(clock_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (ClockExists(clock_id(old_s)) ==> (ResultEqual(result, SUCCESS) && flags == 0 && name == ClockExtendedName(clock_id(old_s)) && IsNullTerminatedAscii(name, 64)))
    && (old_s == new_s)
}