pub open spec fn clock_name_get__3_6_2_11_spec(clock_id: UInt32, status: Int32, flags: UInt32, name: Seq<u8>, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> status == NOT_FOUND)
    && ((ClockExists(old_s, clock_id) && ClockExtendedNameSupported(old_s, clock_id)) ==> (
        status == SUCCESS
        && flags == 0
        && name.len() == 64
        && IsNullTerminatedAscii(name)
        && name == ClockExtendedName(old_s, clock_id)
    ))
    && new_s == old_s
}
