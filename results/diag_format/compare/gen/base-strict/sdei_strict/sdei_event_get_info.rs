pub open spec fn sdei_event_get_info_spec(result: Int64, event: Int32, info: UInt32, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidEventNumber(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidEventInfo(info) ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((info == EV_ROUTING_MODE || info == EV_ROUTING_AFF) && !IsSharedEvent(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (info == EV_ROUTING_AFF && EventRoutingMode(event) != RM_PE ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((info == EV_ROUTING_MODE || info == EV_ROUTING_AFF) && !IsEventRegistered(event) ==> ResultEqual(result, DENIED))
    && (info == EV_TYPE ==> ((IsPrivateEvent(event) ==> result == 0) && (IsSharedEvent(event) ==> result == 1)))
    && (info == EV_SIGNALED ==> ((IsSoftwareSignalable(event) ==> result == 0) && (!IsSoftwareSignalable(event) ==> result == 1)))
    && (info == EV_PRIORITY ==> ((EventPriority(event) == PRIORITY_NORMAL ==> result == 0) && (EventPriority(event) == PRIORITY_CRITICAL ==> result == 1)))
    && (info == EV_ROUTING_MODE ==> ((EventRoutingMode(event) == RM_ANY ==> result == 0) && (EventRoutingMode(event) == RM_PE ==> result == 1)))
    && (info == EV_ROUTING_AFF ==> (result == EventRoutingAffinity(event) && IsMpidrFormat(result)))
    && (old_s == new_s)
}