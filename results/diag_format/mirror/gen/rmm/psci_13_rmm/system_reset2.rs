pub open spec fn system_reset2_spec(reset_type: UInt32, cookie: _, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!IsImplemented(old_s, SYSTEM_RESET2) ==> ResultEqual(result, NOT_SUPPORTED))
  && (reset_type[31] == 0 && reset_type[30:0] != SYSTEM_WARM_RESET ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == RSI_SUCCESS && reset_type == SYSTEM_WARM_RESET ==> MainMemoryPreserved() || FellBackToColdReset())
  && (result == RSI_SUCCESS && reset_type == SYSTEM_WARM_RESET ==> AllMemoryRequestersReset())
  && (result == RSI_SUCCESS && reset_type == SYSTEM_WARM_RESET ==> AllCpusAndMmusReset())
  && (result == RSI_SUCCESS && reset_type == SYSTEM_WARM_RESET ==> AllInterruptsDisabled())
  && (result == RSI_SUCCESS && reset_type == SYSTEM_WARM_RESET ==> SmmuStateEqualsColdResetState())
  && (result == RSI_SUCCESS && reset_type == SYSTEM_WARM_RESET ==> CookieIgnored(cookie))
  && ((!(IsImplemented(old_s, SYSTEM_RESET2)) &&
       !(reset_type[31] == 0 && reset_type[30:0] != SYSTEM_WARM_RESET))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> MainMemoryPreserved())
  && (result != RSI_SUCCESS
    ==> AllMemoryRequestersReset())
  && (result != RSI_SUCCESS
    ==> AllCpusAndMmusReset())
  && (result != RSI_SUCCESS
    ==> AllInterruptsDisabled())
  && (result != RSI_SUCCESS
    ==> SmmuStateEqualsColdResetState())
  && (result != RSI_SUCCESS
    ==> CookieIgnored(cookie))
}