pub open spec fn sdei_event_register_spec(event: int32, entry_point_address: UInt64, ep_argument: UInt64, flags: UInt64, affinity: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INPUT ==> (event < 0))
  && (result == RSI_ERROR_INPUT ==> ((flags & 3) != 0))
  && (result == RSI_ERROR_INPUT ==> ((affinity & 0xFFFF_FFFF_0000_0000) != 0))
  && (result == RSI_SUCCESS ==> SdeiEventAt(new_s, event as int).handler_entry_point_address == entry_point_address)
  && (result == RSI_SUCCESS ==> SdeiEventAt(new_s, event as int).handler_ep_argument == ep_argument)
  && (result == RSI_SUCCESS ==> SdeiEventAt(new_s, event as int).handler_flags == flags)
  && (result == RSI_SUCCESS ==> SdeiEventAt(new_s, event as int).handler_affinity == affinity)
  && ((!(result == RSI_ERROR_INPUT) &&
       result != RSI_ERROR_INPUT)
    ==> SdeiEventAt(new_s, event as int).handler_entry_point_address == SdeiEventAt(old_s, event as int).handler_entry_point_address)
  && ((!(result == RSI_ERROR_INPUT) &&
       result != RSI_ERROR_INPUT)
    ==> SdeiEventAt(new_s, event as int).handler_ep_argument == SdeiEventAt(old_s, event as int).handler_ep_argument)
  && ((!(result == RSI_ERROR_INPUT) &&
       result != RSI_ERROR_INPUT)
    ==> SdeiEventAt(new_s, event as int).handler_flags == SdeiEventAt(old_s, event as int).handler_flags)
  && ((!(result == RSI_ERROR_INPUT) &&
       result != RSI_ERROR_INPUT)
    ==> SdeiEventAt(new_s, event as int).handler_affinity == SdeiEventAt(old_s, event as int).handler_affinity)
}