pub open spec fn cpu_suspend_spec(power_state: PowerState, entry_point_address: Address, context_id: UInt32, result: Result<(), PsciStatusCode>, old_s: S, new_s: S) -> bool {
  (result == INVALID_PARAMETERS ==> result == INVALID_PARAMETERS)
  && (result == INVALID_ADDRESS ==> result == INVALID_ADDRESS)
  && (result == DENIED ==> result == DENIED)
  && ((result == SUCCESS || result == INVALID_PARAMETERS || result == INVALID_ADDRESS || result == DENIED)
    ==> CpuIsOn(new_s, 0))
  && (result == SUCCESS
    ==> CpuIsOn(new_s, 0))
}