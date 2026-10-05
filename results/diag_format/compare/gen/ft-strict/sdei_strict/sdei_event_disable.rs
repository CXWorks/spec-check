pub open spec fn sdei_event_disable_spec(event: Int32, result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsEventRegisteredByClient(old_s, event, CallingClient(old_s)) ==> ResultEqual(result, DENIED))
  && (EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING ==> ResultEqual(result, DENIED))
  && (result == SDEI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SDEI_SUCCESS && IsPrivateEvent(old_s, event) ==> !IsEventEnabledForPe(old_s, event, CallingPe(old_s)))
  && (result == SDEI_SUCCESS && IsSharedEvent(old_s, event) ==> !IsEventEnabledForClient(old_s, event, CallingClient(old_s)))
  && (result == SDEI_SUCCESS ==> RunningEventHandlerUnaffected(old_s, event))
  && (result == SDEI_SUCCESS ==> TriggeredEventsStayPendingUntilEnabled(old_s, event))
  && ((SdeiIsSupported(old_s) &&
       IsValidEventNumber(old_s, event) &&
       IsEventRegisteredByClient(old_s, event, CallingClient(old_s)) &&
       !(EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> IsEventEnabledForPe(new_s, event, CallingPe(new_s)) == IsEventEnabledForPe(old_s, event, CallingPe(old_s)))
  && (result != SDEI_SUCCESS
    ==> IsEventEnabledForClient(new_s, event, CallingClient(new_s)) == IsEventEnabledForClient(old_s, event, CallingClient(old_s)))
  && (result != SDEI_SUCCESS
    ==> RunningEventHandlerUnaffected(new_s, event))
  && (result != SDEI_SUCCESS
    ==> TriggeredEventsStayPendingUntilEnabled(new_s, event))
}