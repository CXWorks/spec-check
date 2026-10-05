pub open spec fn sdei_private_reset_spec(result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (AnyEventHandlerRunning(old_s, CallingPe(old_s)) ==> ResultEqual(result, DENIED))
  && (result == SdeiStatusCode::SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SdeiStatusCode::SUCCESS ==> (forall ev in PrivateEvents(new_s, CallingPe(new_s)): !EventIsRunning(ev) ==> EventState(ev) == UNREGISTERED))
  && (result == SdeiStatusCode::SUCCESS ==> (forall ev in PrivateEvents(new_s, CallingPe(new_s)): EventIsRunning(ev) ==> EventState(ev) == HANDLER_UNREGISTER_PENDING))
  && (result == SdeiStatusCode::SUCCESS ==> (forall ev in PrivateEvents(new_s, CallingPe(new_s)): EventAuxInfoIsCleared(ev) || EventAuxInfoIsWarmBootState(ev)))
  && (result == SdeiStatusCode::SUCCESS ==> (forall ev in SharedEvents(new_s): EventState(ev) == EventState(ev)))
  && ((SdeiIsSupported(old_s) &&
       !AnyEventHandlerRunning(old_s, CallingPe(old_s)))
    ==> result == SdeiStatusCode::SUCCESS)
  && (result != SdeiStatusCode::SUCCESS
    ==> (forall ev in PrivateEvents(new_s, CallingPe(new_s)): EventState(ev) == EventState(ev)))
  && (result != SdeiStatusCode::SUCCESS
    ==> (forall ev in PrivateEvents(new_s, CallingPe(new_s)): EventState(ev) == EventState(ev)))
  && (result != SdeiStatusCode::SUCCESS
    ==> (forall ev in PrivateEvents(new_s, CallingPe(new_s)): EventState(ev) == EventState(ev)))
  && (result != SdeiStatusCode::SUCCESS
    ==> (forall ev in SharedEvents(new_s): EventState(ev) == EventState(ev)))
}