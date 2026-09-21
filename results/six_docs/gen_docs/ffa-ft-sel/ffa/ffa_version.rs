pub open spec fn ffa_version_spec(input_version_number: UInt32, input_flags: UInt32, output_version_number: Int32, result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  ((input_version_number & 0x80000000) != 0 ==> R(FFA_INVALID_PARAMETERS))
  && ((input_flags & 0x3) > 2 ==> R(FFA_INVALID_PARAMETERS))
  && (result == FFA_SUCCESS ==> output_version_number >= 0)
  && ((!( (input_version_number & 0x80000000) != 0) &&
       !((input_flags & 0x3) > 2))
    ==> result == FFA_SUCCESS)
}