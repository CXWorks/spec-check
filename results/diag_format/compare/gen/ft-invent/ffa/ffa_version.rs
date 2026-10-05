pub open spec fn ffa_version_spec(input_version_number: UInt32, input_flags: UInt32, output_version_number: int32, result: Result<(), SmcStatusCode>, old_s: S, new_s: S) -> bool {
  ((input_version_number & 0x80000000) != 0 ==> RsiCommandReturnCode::RSI_ERROR_INPUT)
  && ((input_flags & 0x3) > 2 ==> RsiCommandReturnCode::RSI_ERROR_INPUT)
  && (result == RsiCommandReturnCode::RSI_SUCCESS ==> output_version_number >= 0)
  && (result == RsiCommandReturnCode::RSI_SUCCESS ==> (output_version_number & 0x80000000) == 0)
  && ((!( (input_version_number & 0x80000000) != 0) &&
       !( (input_flags & 0x3) > 2))
    ==> result == RsiCommandReturnCode::RSI_SUCCESS)
  && (result != RsiCommandReturnCode::RSI_SUCCESS
    ==> output_version_number == 0)
}