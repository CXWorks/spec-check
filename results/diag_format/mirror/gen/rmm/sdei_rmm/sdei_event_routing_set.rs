pub open spec fn sdei_event_routing_set_spec(event: Int32, routing_mode: UInt64, affinity: UInt64, result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsSharedEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidRoutingMode(old_s, routing_mode) ==> ResultEqual(result, INVALID_PARAMETERS))
  && ((RoutingMode(routing_mode) == RM_PE) && !IsValidMpidr(old_s, affinity) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (EventAt(old_s, event).handler_state != HANDLER_REGISTERED ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> EventAt(new_s, event).routing_mode == RoutingMode(routing_mode))
  && (result == SUCCESS && (RoutingMode(routing_mode) == RM_PE) ==> EventAt(new_s, event).affinity == affinity)
  && ((!(SdeiIsSupported(old_s)) &&
       IsValidEventNumber(old_s, event) &&
       IsSharedEvent(old_s, event) &&
       IsValidRoutingMode(old_s, routing_mode) &&
       !((RoutingMode(routing_mode) == RM_PE) && !IsValidMpidr(old_s, affinity)) &&
       !(EventAt(old_s, event).handler_state != HANDLER_REGISTERED))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> EventAt(new_s, event).routing_mode == EventAt(old_s, event).routing_mode)
  && (result != SUCCESS
    ==> EventAt(new_s, event).affinity == EventAt(old_s, event).affinity)
}