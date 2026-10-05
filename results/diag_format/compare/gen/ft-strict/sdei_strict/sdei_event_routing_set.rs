pub open spec fn sdei_event_routing_set_spec(event: Int32, routing_mode: UInt64, affinity: UInt64, result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsSharedEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidRoutingMode(old_s, routing_mode) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(routing_mode, 0, 0) == RM_PE && !IsValidAffinity(old_s, affinity) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (EventHandlerState(old_s, event) != HANDLER_REGISTERED ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> EventRoutingMode(new_s, event) == Bits(routing_mode, 0, 0))
  && (result == SUCCESS && Bits(routing_mode, 0, 0) == RM_PE ==> EventRoutingAffinity(new_s, event) == affinity)
  && ((SdeiIsSupported(old_s) &&
       IsValidEventNumber(old_s, event) &&
       IsSharedEvent(old_s, event) &&
       IsValidRoutingMode(old_s, routing_mode) &&
       !(Bits(routing_mode, 0, 0) == RM_PE && !IsValidAffinity(old_s, affinity)) &&
       !(EventHandlerState(old_s, event) != HANDLER_REGISTERED))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> EventRoutingMode(new_s, event) == EventRoutingMode(old_s, event))
  && (result != SUCCESS
    ==> EventRoutingAffinity(new_s, event) == EventRoutingAffinity(old_s, event))
}