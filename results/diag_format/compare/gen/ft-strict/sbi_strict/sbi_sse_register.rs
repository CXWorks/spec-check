pub open spec fn sbi_sse_register_spec(event_id: UInt32, handler_entry_pc: Address, handler_entry_arg: UInt64, result: SbiReturnCode, old_s: S, new_s: S) -> bool {
  (!IsValidEventId(old_s, event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && ((handler_entry_pc) % 2 != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (IsReservedEventId(old_s, event_id) && IsValidEventId(old_s, event_id) && !PlatformSupportsEvent(old_s, event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (EventState(old_s, event_id) != UNUSED ==> ResultEqual(result, SBI_ERR_INVALID_STATE))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS && IsLocalEvent(old_s, event_id) ==> EventStateForHart(new_s, event_id, CallingHart()) == REGISTERED)
  && (result == SBI_SUCCESS && IsGlobalEvent(old_s, event_id) ==> (forall (h: Hart), EventStateForHart(new_s, event_id, h) == REGISTERED))
  && (result == SBI_SUCCESS ==> EventAttribute(new_s, event_id, ENTRY_PC) == handler_entry_pc)
  && (result == SBI_SUCCESS ==> EventAttribute(new_s, event_id, ENTRY_ARG) == handler_entry_arg)
  && ((IsValidEventId(old_s, event_id) &&
       !( (handler_entry_pc) % 2 != 0) &&
       !(IsReservedEventId(old_s, event_id) && IsValidEventId(old_s, event_id) && !PlatformSupportsEvent(old_s, event_id)) &&
       !(EventState(old_s, event_id) != UNUSED))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> EventStateForHart(new_s, event_id, CallingHart()) == EventState(old_s, event_id))
  && (result != SBI_SUCCESS
    ==> (forall (h: Hart), EventStateForHart(new_s, event_id, h) == EventStateForHart(old_s, event_id, h)))
  && (result != SBI_SUCCESS
    ==> EventAttribute(new_s, event_id, ENTRY_PC) == EventAttribute(old_s, event_id, ENTRY_PC))
  && (result != SBI_SUCCESS
    ==> EventAttribute(new_s, event_id, ENTRY_ARG) == EventAttribute(old_s, event_id, ENTRY_ARG))
}