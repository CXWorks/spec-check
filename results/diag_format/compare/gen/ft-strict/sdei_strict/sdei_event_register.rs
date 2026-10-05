pub open spec fn sdei_event_register_spec(event: Int32, entry_point_address: UInt64, ep_argument: UInt64, flags: UInt64, affinity: UInt64, result: ResultEqual(s, SUCCESS), old_s: S, new_s: S) -> bool {
  (!SdeiSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (DispatcherDetectsInvalidEntryPoint(old_s, entry_point_address, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsSharedEvent(old_s, event) && !IsValidRoutingMode(old_s, Bits(flags, 0, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsSharedEvent(old_s, event) && Bits(flags, 0, 0) == RM_PE && !IsValidAffinity(old_s, affinity) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsRegisteredByClient(old_s, event, CallingClient(old_s)) ==> ResultEqual(result, DENIED))
  && (EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS && IsSharedEvent(old_s, event) ==> IsRegisteredForClient(new_s, event, CallingClient(new_s)))
  && (result == SUCCESS && !IsSharedEvent(old_s, event) ==> IsRegisteredForPe(new_s, event, CallingPe(new_s)))
  && (result == SUCCESS && !EventEnabled(old_s, event))
  && (result == SUCCESS ==> EventEntryPoint(new_s, event) == ResolveEntryPoint(new_s, entry_point_address, Bits(flags, 1, 1)))
  && (result == SUCCESS ==> EventArgument(new_s, event) == ep_argument)
  && (result == SUCCESS && IsSharedEvent(old_s, event) ==> EventRoutingMode(new_s, event) == Bits(flags, 0, 0))
  && (result == SUCCESS && IsSharedEvent(old_s, event) && Bits(flags, 0, 0) == RM_PE ==> EventAffinity(new_s, event) == affinity)
  && ((SdeiSupported(old_s) &&
       IsValidEvent(old_s, event) &&
       !DispatcherDetectsInvalidEntryPoint(old_s, entry_point_address, flags) &&
       !(IsSharedEvent(old_s, event) && !IsValidRoutingMode(old_s, Bits(flags, 0, 0))) &&
       !(IsSharedEvent(old_s, event) && Bits(flags, 0, 0) == RM_PE && !IsValidAffinity(old_s, affinity)) &&
       !(IsRegisteredByClient(old_s, event, CallingClient(old_s))) &&
       !(EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> IsRegisteredForClient(new_s, event, CallingClient(new_s)) == IsRegisteredForClient(old_s, event, CallingClient(old_s)))
  && (result != SUCCESS
    ==> IsRegisteredForPe(new_s, event, CallingPe(new_s)) == IsRegisteredForPe(old_s, event, CallingPe(old_s)))
  && (result != SUCCESS
    ==> EventEntryPoint(new_s, event) == EventEntryPoint(old_s, event))
  && (result != SUCCESS
    ==> EventArgument(new_s, event) == EventArgument(old_s, event))
  && (result != SUCCESS
    ==> EventRoutingMode(new_s, event) == EventRoutingMode(old_s, event))
  && (result != SUCCESS
    ==> EventAffinity(new_s, event) == EventAffinity(old_s, event))
  && (!(result == SUCCESS && (IsSharedEvent(old_s, event) && Bits(flags, 0, 0) == RM_PE))
    ==> EventAffinity(new_s, event) == EventAffinity(old_s, event))
}