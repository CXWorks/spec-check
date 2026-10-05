pub open spec fn sdei_event_disable_spec(event: Int32, result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsKnownEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!EventIsRegisteredByClient(old_s, event) ==> ResultEqual(result, DENIED))
  && (EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING ==> ResultEqual(result, DENIED))
  && (result == SDEI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SDEI_SUCCESS && EventIsPrivate(old_s, event) ==> !EventIsEnabledForPe(new_s, event, CallingPe()))
  && (result == SDEI_SUCCESS && EventIsShared(old_s, event) ==> !EventIsEnabledForClient(new_s, event, CallingClient()))
  && ((SdeiIsSupported(old_s) &&
       IsKnownEvent(old_s, event) &&
       EventIsRegisteredByClient(old_s, event) &&
       !(EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> EventIsEnabledForPe(new_s, event, CallingPe()) == EventIsEnabledForPe(old_s, event, CallingPe()))
  && (result != SDEI_SUCCESS
    ==> EventIsEnabledForClient(new_s, event, CallingClient()) == EventIsEnabledForClient(old_s, event, CallingClient()))
  && (result != SDEI_SUCCESS
    ==> EventIsPrivate(new_s, event) == EventIsPrivate(old_s, event))
  && (result != SDEI_SUCCESS
    ==> EventIsShared(new_s, event) == EventIsShared(old_s, event))
}