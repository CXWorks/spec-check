pub open spec fn cpu_default_suspend_spec(result: PsciReturnCode, entry_point_address: Address, context_id: UInt64, old_s: S, new_s: S) -> bool {
    (IsKnownUnavailableToCaller(entry_point_address) ==> result == INVALID_ADDRESS)
    && (!IsKnownUnavailableToCaller(entry_point_address) ==> result == SUCCESS)
}
