pub open spec fn sdei_event_status_spec(event: Int32, result: Int64, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsKnownEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == SDEI_SUCCESS ==> Bits(result, 63, 3) == 0)
  && (result == SDEI_SUCCESS ==> (Bits(result, 2, 2) == 1) == EventHandlerIsRunning(new_s, event))
  && (result == SDEI_SUCCESS ==> (Bits(result, 1, 1) == 1) == EventHandlerIsEnabled(new_s, event))
  && (result == SDEI_SUCCESS ==> (Bits(result, 0, 0) == 1) == EventHandlerIsRegistered(new_s, event))
  && (result == SDEI_SUCCESS ==> EventStatusMapsToHandlerState(new_s, result, event))
  && ((SdeiIsSupported(old_s) &&
       IsKnownEventNumber(old_s, event))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> Bits(result, 63, 3) == 0)
  && (result != SDEI_SUCCESS
    ==> (Bits(result, 2, 2) == 1) == EventHandlerIsRunning(new_s, event))
  && (result != SDEI_SUCCESS
    ==> (Bits(result, 1, 1) == 1) == EventHandlerIsEnabled(new_s, event))
  && (result != SDEI_SUCCESS
    ==> (Bits(result, 0, 0) == 1) == EventHandlerIsRegistered(new_s, event))
  && (result != SDEI_SUCCESS
    ==> EventStatusMapsToHandlerState(new_s, result, event))
}