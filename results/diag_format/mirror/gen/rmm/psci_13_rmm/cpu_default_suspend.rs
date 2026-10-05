pub open spec fn cpu_default_suspend_spec(entry_point_address: Address, context_id: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (IsKnownUnavailableToCaller(old_s, entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> ReturnedAtNextInstruction(new_s) || ResumedAt(new_s, entry_point_address))
  && (result == SUCCESS ==> ResumedAt(new_s, entry_point_address) ==> ContextIdPresented(new_s, context_id))
  && (result == SUCCESS ==> AllCoresInDefaultSuspend(new_s) ==> !ThermallyCritical(new_s, platform))
  && ((!IsKnownUnavailableToCaller(old_s, entry_point_address))
    ==> result == SUCCESS)
}