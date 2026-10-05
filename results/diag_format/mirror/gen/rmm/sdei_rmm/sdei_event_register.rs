pub open spec fn sdei_event_register_spec(event: Int32, entry_point_address: Address, ep_argument: UInt64, flags: UInt64, affinity: UInt64, result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (DispatcherCanDetermineInvalidAddress(old_s, entry_point_address) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsSharedEvent(old_s, event) && !IsValidRoutingMode(old_s, flags.routing_mode) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsSharedEvent(old_s, event) && flags.routing_mode == RM_PE && !IsValidMpidr(old_s, affinity) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsRegisteredByClient(old_s, event) ==> ResultEqual(result, DENIED))
  && (EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> IsRegisteredByClient(new_s, event))
  && (result == SUCCESS ==> EventHandler(new_s, event).entry_point_address == entry_point_address && EventHandler(new_s, event).relative_mode == flags.relative_mode)
  && (result == SUCCESS ==> EventHandler(new_s, event).ep_argument == ep_argument)
  && (result == SUCCESS ==> !IsEnabled(new_s, event))
  && (result == SUCCESS && IsSharedEvent(old_s, event) ==> HandlerRegisteredGloballyForClient(new_s, event))
  && (result == SUCCESS && IsPrivateEvent(old_s, event) ==> HandlerRegisteredForCallingPe(new_s, event))
  && (result == SUCCESS && IsSharedEvent(old_s, event) ==> RoutingMode(new_s, event) == flags.routing_mode)
  && (result == SUCCESS && IsSharedEvent(old_s, event) && flags.routing_mode == RM_PE ==> RoutingAffinity(new_s, event) == affinity)
  && ((SdeiIsSupported(old_s) &&
       IsValidEvent(old_s, event) &&
       !DispatcherCanDetermineInvalidAddress(old_s, entry_point_address) &&
       !(IsSharedEvent(old_s, event) && !IsValidRoutingMode(old_s, flags.routing_mode)) &&
       !(IsSharedEvent(old_s, event) && flags.routing_mode == RM_PE && !IsValidMpidr(old_s, affinity)) &&
       !IsRegisteredByClient(old_s, event) &&
       !(EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> EventHandler(new_s, event).entry_point_address == EventHandler(old_s, event).entry_point_address)
  && (result != SUCCESS
    ==> EventHandler(new_s, event).relative_mode == EventHandler(old_s, event).relative_mode)
  && (result != SUCCESS
    ==> EventHandler(new_s, event).ep_argument == EventHandler(old_s, event).ep_argument)
  && (result != SUCCESS
    ==> RoutingMode(new_s, event) == RoutingMode(old_s, event))
  && (result != SUCCESS
    ==> RoutingAffinity(new_s, event) == RoutingAffinity(old_s, event))
  && (!(result == SUCCESS && (IsSharedEvent(old_s, event) && flags.routing_mode == RM_PE))
    ==> RoutingAffinity(new_s, event) == RoutingAffinity(old_s, event))
}