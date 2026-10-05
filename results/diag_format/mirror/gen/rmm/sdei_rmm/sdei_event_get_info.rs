pub open spec fn sdei_event_get_info_spec(event: Int32, info: UInt32, result: Int64, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidInfo(old_s, info) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (info == EV_ROUTING_MODE && !IsSharedEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (info == EV_ROUTING_MODE && !IsEventRegistered(old_s, event) ==> ResultEqual(result, DENIED))
  && (info == EV_ROUTING_AFF && !IsSharedEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (info == EV_ROUTING_AFF && !RoutingModeHasAffinity(old_s, EventRoutingMode(event)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (info == EV_ROUTING_AFF && !IsEventRegistered(old_s, event) ==> ResultEqual(result, DENIED))
  && (result == EV_TYPE ==> result == (IsSharedEvent(old_s, event) ? 1 : 0))
  && (result == EV_SIGNALED ==> result == (CanBeSoftwareSignaled(old_s, event) ? 0 : 1))
  && (result == EV_PRIORITY ==> result == (IsCriticalPriority(old_s, event) ? 1 : 0))
  && (result == EV_ROUTING_MODE ==> result == EventRoutingMode(old_s, event))
  && (result == EV_ROUTING_AFF ==> result == EventRoutingAffinity(old_s, event))
  && ((SdeiIsSupported(old_s) &&
       IsValidEventNumber(old_s, event) &&
       IsValidInfo(old_s, info) &&
       !(info == EV_ROUTING_MODE && !IsSharedEvent(old_s, event)) &&
       !(info == EV_ROUTING_MODE && !IsEventRegistered(old_s, event)) &&
       !(info == EV_ROUTING_AFF && !IsSharedEvent(old_s, event)) &&
       !(info == EV_ROUTING_AFF && !RoutingModeHasAffinity(old_s, EventRoutingMode(event))) &&
       !(info == EV_ROUTING_AFF && !IsEventRegistered(old_s, event)))
    ==> result != NOT_SUPPORTED &&
        result != INVALID_PARAMETERS &&
        result != INVALID_PARAMETERS &&
        result != INVALID_PARAMETERS &&
        result != DENIED &&
        result != INVALID_PARAMETERS &&
        result != INVALID_PARAMETERS &&
        result != DENIED)
}