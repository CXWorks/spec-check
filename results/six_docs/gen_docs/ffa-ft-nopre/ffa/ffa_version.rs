pub open spec fn ffa_version_spec(input_version_number: UInt32, input_flags: UInt32, output_version_number: Int32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((input_version_number & 0x80000000) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((input_flags & 0x3) > 2 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> output_version_number >= 0)
  && ((!( (input_version_number & 0x80000000) != 0) &&
       !((input_flags & 0x3) > 2))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> output_version_number == 0)
}