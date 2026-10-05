pub open spec fn sdei_event_status_spec(event: Int32, result: Int64, old_s: S, new_s: S) -> bool {
  (!IsSdeiSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == SDEI_SUCCESS ==> result[63:3] == 0)
  && (result == SDEI_SUCCESS ==> result[2] == (EventHandlerIsRunning(old_s, event) ? 1 : 0))
  && (result == SDEI_SUCCESS ==> result[1] == (EventHandlerIsEnabled(old_s, event) ? 1 : 0))
  && (result == SDEI_SUCCESS ==> result[0] == (EventHandlerIsRegistered(old_s, event) ? 1 : 0))
  && ((IsSdeiSupported(old_s) &&
       IsValidEventNumber(old_s, event))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> result[63:3] == 0)
  && (result != SDEI_SUCCESS
    ==> result[2] == 0)
  && (result != SDEI_SUCCESS
    ==> result[1] == 0)
  && (result != SDEI_SUCCESS
    ==> result[0] == 0)
}