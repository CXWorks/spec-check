pub open spec fn sdei_event_signal_spec(event: Int32, target_pe: UInt64, result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (event != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidMpidr(old_s, target_pe) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == SDEI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SDEI_SUCCESS ==> EventIsPending(new_s, target_pe, event))
  && (result == SDEI_SUCCESS ==> EventIsPrivateTo(new_s, target_pe, event))
  && (result == SDEI_SUCCESS ==> ClientSharedDataObservableBeforeEvent(new_s, target_pe, event))
  && ((SdeiIsSupported(old_s) &&
       event == 0 &&
       IsValidMpidr(old_s, target_pe))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> !EventIsPending(new_s, target_pe, event))
  && (result != SDEI_SUCCESS
    ==> !EventIsPrivateTo(new_s, target_pe, event))
  && (result != SDEI_SUCCESS
    ==> !ClientSharedDataObservableBeforeEvent(new_s, target_pe, event))
}