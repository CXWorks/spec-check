pub open spec fn ffa_version_spec(input_version_number: UInt32, input_flags: UInt32, output_version_number: int32, old_s: S, new_s: S) -> bool {
  ((input_version_number & 0x80000000) != 0 ==> R0216)
  && ((input_flags & 0x3) > 2 ==> R0216)
  && ((!( (input_version_number & 0x80000000) != 0) &&
       !( (input_flags & 0x3) > 2))
    ==> output_version_number >= 0)
}