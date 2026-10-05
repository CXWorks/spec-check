pub open spec fn clock_name_get__3_6_2_11_spec(result: Int32, flags: UInt32, name: UInt8[64], clock_id: UInt32, old_s: S, new_s: S) -> bool {
    (!ClockExists(clock_id) ==> ResultEqual(result, NOT_FOUND))
    && (ClockExists(clock_id) ==> (ResultEqual(result, SUCCESS) && Bits(flags, 31, 0) == 0 && IsClockExtendedName(name, clock_id) && IsNullTerminatedAscii(name, 64)))
    && (old_s == new_s)
}