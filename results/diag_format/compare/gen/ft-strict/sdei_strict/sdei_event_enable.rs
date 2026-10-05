pub open spec fn sdei_event_enable_spec(event: Int32, result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsKnownEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsEventRegisteredByClient(old_s, event, CallingClient(old_s)) ==> ResultEqual(result, DENIED))
  && (EventHandlerState(old_s, event, CallingClient(old_s)) == HANDLER_UNREGISTER_PENDING ==> ResultEqual(result, DENIED))
  && (result == SDEI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SDEI_SUCCESS && IsPrivateEvent(old_s, event) ==> IsEventEnabledForPe(old_s, event, CallingPe(old_s)))
  && (result == SDEI_SUCCESS && IsSharedEvent(old_s, event) ==> IsEventEnabledForClient(old_s, event, CallingClient(old_s)))
  && (result == SDEI_SUCCESS && IsEventPending(old_s, event) ==> EventDeliveredWhenEnabled(old_s, event, CallingClient(old_s)))
  && ((SdeiIsSupported(old_s) &&
       IsKnownEventNumber(old_s, event) &&
       IsEventRegisteredByClient(old_s, event, CallingClient(old_s)) &&
       !(EventHandlerState(old_s, event, CallingClient(old_s)) == HANDLER_UNREGISTER_PENDING))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> IsEventEnabledForPe(new_s, event, CallingPe(new_s)) == IsEventEnabledForPe(old_s, event, CallingPe(old_s)))
  && (result != SDEI_SUCCESS
    ==> IsEventEnabledForClient(new_s, event, CallingClient(new_s)) == IsEventEnabledForClient(old_s, event, CallingClient(old_s)))
  && (result != SDEI_SUCCESS
    ==> EventDeliveredWhenEnabled(new_s, event, CallingClient(new_s)) == EventDeliveredWhenEnabled(old_s, event, CallingClient(old_s)))
}