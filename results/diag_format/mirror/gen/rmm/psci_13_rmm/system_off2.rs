pub open spec fn system_off2_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!IsFunctionImplemented(old_s, SYSTEM_OFF2) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!AreInputParametersValid(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == RSI_SUCCESS ==> SystemIsPoweredOffFromCallerView(new_s))
  && (result == RSI_SUCCESS ==> NextStartIsColdBoot(new_s))
  && (result == RSI_SUCCESS ==> SavedToNonVolatileStorage(new_s, BaseHardwareConfiguration))
  && ((IsFunctionImplemented(old_s, SYSTEM_OFF2) &&
       AreInputParametersValid(old_s))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> SystemIsPoweredOffFromCallerView(new_s))
  && (result != RSI_SUCCESS
    ==> NextStartIsColdBoot(new_s))
  && (result != RSI_SUCCESS
    ==> SavedToNonVolatileStorage(new_s, BaseHardwareConfiguration))
  && (result != RSI_SUCCESS
    ==> SystemIsPoweredOffFromCallerView(new_s))
  && (result != RSI_SUCCESS
    ==> NextStartIsColdBoot(new_s))
  && (result != RSI_SUCCESS
    ==> SavedToNonVolatileStorage(new_s, BaseHardwareConfiguration))
}