pub open spec fn system_off2_spec(hibernate_type: UInt32, cookie: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!IsSystemOff2Implemented(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((IsSystemOff2Implemented(old_s) && !SystemOff2ParamsValid(old_s, hibernate_type, cookie)) ==> (result == INVALID_PARAMETERS && new_s == old_s))
    && ((IsSystemOff2Implemented(old_s) && SystemOff2ParamsValid(old_s, hibernate_type, cookie)) ==> (SystemHibernateStateSaved(old_s, new_s) && SystemPoweredOff(new_s)))
}
