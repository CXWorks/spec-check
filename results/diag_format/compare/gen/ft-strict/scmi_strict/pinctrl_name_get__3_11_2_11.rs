pub open spec fn pinctrl_name_get__3_11_2_11_spec(identifier: UInt32, flags: UInt32, status: Int32, flags: UInt32, name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (!PinctrlEntityExists(new_s, (flags & 0x3) as int, (identifier & 0xFFFF) as int) ==> ResultEqual(status, NOT_FOUND))
  && (result ==> ResultEqual(status, SUCCESS))
  && (result ==> name == ExtendedName(new_s, (flags & 0x3) as int, (identifier & 0xFFFF) as int))
  && (result ==> IsNullTerminatedAscii(name, 64))
  && (result ==> flags == 0)
  && ((!(PinctrlEntityExists(new_s, (flags & 0x3) as int, (identifier & 0xFFFF) as int)))
    ==> ResultEqual(status, SUCCESS))
  && (result
    ==> flags == 0)
}