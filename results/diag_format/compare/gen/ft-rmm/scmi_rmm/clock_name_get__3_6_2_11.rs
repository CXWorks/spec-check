pub open spec fn clock_name_get__3_6_2_11_spec(clock_id: UInt32, status: Int32, flags: UInt32, name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> flags == 0)
  && (ResultEqual(status, SUCCESS) ==> name == ClockExtendedName(new_s, clock_id))
  && (ResultEqual(status, SUCCESS) ==> IsNullTerminatedAscii(name, 64))
  && ((ClockExists(old_s, clock_id))
    ==> ResultEqual(status, SUCCESS))
}