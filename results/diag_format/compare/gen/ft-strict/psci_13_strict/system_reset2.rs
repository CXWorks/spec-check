pub open spec fn system_reset2_spec(reset_type: UInt32, cookie: UInt64, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsImplemented(old_s, SYSTEM_RESET2) ==> result == NOT_SUPPORTED)
  && (Bits(reset_type, 31, 31) == 0 && Bits(reset_type, 30, 0) != SYSTEM_WARM_RESET ==> result == INVALID_PARAMETERS)
  && (result == RSI_SUCCESS ==> ResetAppliesToCallerMachineView(new_s, reset_type))
  && (result == RSI_SUCCESS ==> Bits(reset_type, 31, 0) == SYSTEM_WARM_RESET ==> PreservedMainMemoryUnchanged(new_s))
  && (result == RSI_SUCCESS ==> Bits(reset_type, 31, 0) == SYSTEM_WARM_RESET ==> MemoryRequestersReset(new_s))
  && (result == RSI_SUCCESS ==> Bits(reset_type, 31, 0) == SYSTEM_WARM_RESET ==> AllCpusAndMmusReset(new_s))
  && (result == RSI_SUCCESS ==> Bits(reset_type, 31, 0) == SYSTEM_WARM_RESET ==> AllInterruptsDisabled(new_s))
  && (result == RSI_SUCCESS ==> Bits(reset_type, 31, 0) == SYSTEM_WARM_RESET ==> SmmusInColdResetState(new_s))
  && (result == RSI_SUCCESS ==> Bits(reset_type, 31, 0) == SYSTEM_WARM_RESET ==> CookieIgnored(new_s, cookie))
  && (result == RSI_SUCCESS ==> Bits(reset_type, 31, 31) == 1 ==> VendorResetPerformed(new_s, reset_type, cookie))
  && ((!(IsImplemented(old_s, SYSTEM_RESET2)) &&
       !(Bits(reset_type, 31, 31) == 0 && Bits(reset_type, 30, 0) != SYSTEM_WARM_RESET)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> ResetAppliesToCallerMachineView(new_s, reset_type))
  && (result != RSI_SUCCESS
    ==> Bits(reset_type, 31, 0) == SYSTEM_WARM_RESET ==> PreservedMainMemoryUnchanged(new_s))
  && (result != RSI_SUCCESS
    ==> Bits(reset_type, 31, 0) == SYSTEM_WARM_RESET ==> MemoryRequestersReset(new_s))
  && (result != RSI_SUCCESS
    ==> Bits(reset_type, 31, 0) == SYSTEM_WARM_RESET ==> AllCpusAndMmusReset(new_s))
  && (result != RSI_SUCCESS
    ==> Bits(reset_type, 31, 0) == SYSTEM_WARM_RESET ==> AllInterruptsDisabled(new_s))
  && (result != RSI_SUCCESS
    ==> Bits(reset_type, 31, 0) == SYSTEM_WARM_RESET ==> SmmusInColdResetState(new_s))
  && (result != RSI_SUCCESS
    ==> Bits(reset_type, 31, 0) == SYSTEM_WARM_RESET ==> CookieIgnored(new_s, cookie))
  && (result != RSI_SUCCESS
    ==> Bits(reset_type, 31, 31) == 1 ==> VendorResetPerformed(new_s, reset_type, cookie))
}