pub open spec fn cpu_default_suspend_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (IsKnownUnavailableToCaller(old_s, entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
    && (ResultEqual(result, SUCCESS) ==> (ReturnedAtNextInstruction() || ResumedAt(entry_point_address)))
    && (ResumedAt(entry_point_address) ==> ContextIdPresented(context_id))
    && (AllCoresInDefaultSuspend() ==> !ThermallyCritical(platform))
}