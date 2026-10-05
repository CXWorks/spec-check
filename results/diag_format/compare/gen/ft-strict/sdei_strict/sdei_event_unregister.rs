pub open spec fn sdei_event_unregister_spec(event: Int32, result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsSdeiSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsEventRegisteredByClient(old_s, event) ==> ResultEqual(result, DENIED))
  && (IsHandlerRunning(old_s, event) || IsUnregisterPending(old_s, event) ==> ResultEqual(result, PENDING))
  && (result == SdeiStatusCode::SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SdeiStatusCode::SUCCESS && IsSharedEvent(old_s, event) ==> !IsEventRegisteredGlobally(new_s, event))
  && (result == SdeiStatusCode::SUCCESS && IsPrivateEvent(old_s, event) ==> !IsEventRegisteredOnPe(new_s, event, CurrentPe()))
  && (result == SdeiStatusCode::SUCCESS ==> !IsEventDeliverableToClient(new_s, event))
  && ((IsSdeiSupported(old_s) &&
       IsValidEventNumber(old_s, event) &&
       IsEventRegisteredByClient(old_s, event) &&
       !(IsHandlerRunning(old_s, event) || IsUnregisterPending(old_s, event)))
    ==> result == SdeiStatusCode::SUCCESS)
  && (result != SdeiStatusCode::SUCCESS
    ==> IsEventRegisteredGlobally(new_s, event) == IsEventRegisteredGlobally(old_s, event))
  && (result != SdeiStatusCode::SUCCESS
    ==> IsEventRegisteredOnPe(new_s, event, CurrentPe()) == IsEventRegisteredOnPe(old_s, event, CurrentPe()))
  && (result != SdeiStatusCode::SUCCESS
    ==> IsEventDeliverableToClient(new_s, event) == IsEventDeliverableToClient(old_s, event))
}