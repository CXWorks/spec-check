pub open spec fn sdei_event_get_info_spec(event: Int32, info: UInt32, result: Int64, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidEventInfo(old_s, info) ==> ResultEqual(result, INVALID_PARAMETERS))
  && ((info == EV_ROUTING_MODE || info == EV_ROUTING_AFF) && !IsSharedEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (info == EV_ROUTING_AFF && EventRoutingMode(old_s, event) != RM_PE ==> ResultEqual(result, INVALID_PARAMETERS))
  && ((info == EV_ROUTING_MODE || info == EV_ROUTING_AFF) && !IsEventRegistered(old_s, event) ==> ResultEqual(result, DENIED))
  && (result == 0 && (info == EV_TYPE) && IsPrivateEvent(old_s, event) ==> result == 0)
  && (result == 1 && (info == EV_TYPE) && IsSharedEvent(old_s, event) ==> result == 1)
  && (result == 0 && (info == EV_SIGNALED) && IsSoftwareSignalable(old_s, event) ==> result == 0)
  && (result == 1 && (info == EV_SIGNALED) && !IsSoftwareSignalable(old_s, event) ==> result == 1)
  && (result == 0 && (info == EV_PRIORITY) && EventPriority(old_s, event) == PRIORITY_NORMAL ==> result == 0)
  && (result == 1 && (info == EV_PRIORITY) && EventPriority(old_s, event) == PRIORITY_CRITICAL ==> result == 1)
  && (result == 0 && (info == EV_ROUTING_MODE) && EventRoutingMode(old_s, event) == RM_ANY ==> result == 0)
  && (result == 1 && (info == EV_ROUTING_MODE) && EventRoutingMode(old_s, event) == RM_PE ==> result == 1)
  && (result == EventRoutingAffinity(old_s, event) && (info == EV_ROUTING_AFF) ==> IsMpidrFormat(result))
  && ((SdeiIsSupported(old_s) &&
       IsValidEventNumber(old_s, event) &&
       IsValidEventInfo(old_s, info) &&
       !((info == EV_ROUTING_MODE || info == EV_ROUTING_AFF) && !IsSharedEvent(old_s, event)) &&
       !(info == EV_ROUTING_AFF && EventRoutingMode(old_s, event) != RM_PE) &&
       !((info == EV_ROUTING_MODE || info == EV_ROUTING_AFF) && !IsEventRegistered(old_s, event)))
    ==> result == 0)
}