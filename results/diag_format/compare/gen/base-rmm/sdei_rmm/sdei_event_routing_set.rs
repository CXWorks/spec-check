pub open spec fn sdei_event_routing_set_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidEventNumber(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsSharedEvent(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidRoutingMode(routing_mode) ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((RoutingMode(routing_mode) == RM_PE) && !IsValidMpidr(affinity) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (EventAt(event).handler_state != HANDLER_REGISTERED ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (EventAt(event).routing_mode == RoutingMode(routing_mode)))
    && ((RoutingMode(routing_mode) == RM_PE) ==> (EventAt(event).affinity == affinity))
}