pub open spec fn cpu_on_spec(result: PsciStatusCode, old_s: S, new_s: S) -> bool {
    (PsciStatusCode::INVALID_PARAMETERS == result ==> (old_s.target_cpu as int < 0 || old_s.target_cpu as int >= 0x1_0000_0000))
    && (PsciStatusCode::INVALID_ADDRESS == result ==> (old_s.entry_point as int < 0 || old_s.entry_point as int >= 0x1_0000_0000_0000))
    && (PsciStatusCode::ALREADY_ON == result ==> (old_s.core_state(old_s.target_cpu) == PsciCoreState::ON))
    && (PsciStatusCode::ON_PENDING == result ==> (old_s.core_state(old_s.target_cpu) == PsciCoreState::ON_PENDING))
    && (PsciStatusCode::INTERNAL_FAILURE == result ==> true)
    && (PsciStatusCode::DENIED == result ==> true)
    && (PsciStatusCode::SUCCESS == result ==> (new_s.core_state(old_s.target_cpu) == PsciCoreState::ON))
}