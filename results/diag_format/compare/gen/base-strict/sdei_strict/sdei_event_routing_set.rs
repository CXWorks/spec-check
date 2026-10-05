pub open spec fn sdei_event_routing_set_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidEventNumber(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsSharedEvent(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidRoutingMode(routing_mode) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (Bits(routing_mode, 0, 0) == RM_PE && !IsValidAffinity(affinity) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (EventHandlerState(event) != HANDLER_REGISTERED ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> true)
    && (ResultEqual(result, SUCCESS) ==> EventRoutingMode(event) == Bits(routing_mode, 0, 0))
    && (ResultEqual(result, SUCCESS) ==> (Bits(routing_mode, 0, 0) == RM_PE ==> EventRoutingAffinity(event) == affinity))
}