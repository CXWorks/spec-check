pub open spec fn sdei_event_unregister_spec(event: Int32, result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!EventIsRegisteredByClient(old_s, event) ==> ResultEqual(result, DENIED))
  && ((EventHandler(old_s, event).handler_running == true || EventHandler(old_s, event).state == HANDLER_UNREGISTER_PENDING) ==> ResultEqual(result, PENDING))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS && IsSharedEvent(old_s, event) ==> !EventIsRegisteredByClient(new_s, event))
  && (result == SUCCESS && IsPrivateEvent(old_s, event) ==> !EventIsRegisteredByClient(new_s, event))
  && ((!(SdeiIsSupported(old_s)) &&
       IsValidEventNumber(old_s, event) &&
       EventIsRegisteredByClient(old_s, event) &&
       !((EventHandler(old_s, event).handler_running == true) || (EventHandler(old_s, event).state == HANDLER_UNREGISTER_PENDING)))
    ==> ResultEqual(result, SUCCESS))
  && (result != SUCCESS
    ==> EventIsRegisteredByClient(new_s, event))
  && (result != SUCCESS
    ==> EventIsRegisteredByClient(new_s, event))
}