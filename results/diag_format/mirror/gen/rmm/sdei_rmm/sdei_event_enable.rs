pub open spec fn sdei_event_enable_spec(event: Int32, result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsKnownEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsEventRegisteredByClient(old_s, event) ==> ResultEqual(result, DENIED))
  && (EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING ==> ResultEqual(result, DENIED))
  && (result == SDEI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SDEI_SUCCESS && IsPrivateEvent(old_s, event) ==> IsEventEnabledForPe(new_s, event, CallingPe()))
  && (result == SDEI_SUCCESS && IsSharedEvent(old_s, event) ==> IsEventEnabledForClient(new_s, event, CallingClient()))
  && ((SdeiIsSupported(old_s) &&
       IsKnownEvent(old_s, event) &&
       IsEventRegisteredByClient(old_s, event) &&
       !(EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> IsEventEnabledForPe(new_s, event, CallingPe()) == IsEventEnabledForPe(old_s, event, CallingPe()))
  && (result != SDEI_SUCCESS
    ==> IsEventEnabledForClient(new_s, event, CallingClient()) == IsEventEnabledForClient(old_s, event, CallingClient()))
}