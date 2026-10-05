pub open spec fn power_state_notify__3_3_2_8_spec(status: ScmiStatus, agent_id: u32, domain_id: u32, notify_enable: u32, old_s: S, new_s: S) -> bool {
    (!PowerDomainIsValid(old_s, domain_id) && (notify_enable & 0xFFFF_FFFEu32) == 0
        ==> status == NOT_FOUND)
    && (PowerDomainIsValid(old_s, domain_id) && (notify_enable & 0xFFFF_FFFEu32) != 0
        ==> status == INVALID_PARAMETERS)
    && (!PowerDomainIsValid(old_s, domain_id) && (notify_enable & 0xFFFF_FFFEu32) != 0
        ==> (status == NOT_FOUND || status == INVALID_PARAMETERS))
    && (status != SUCCESS ==> new_s == old_s)
    && (status == SUCCESS ==> (
        PowerStateNotifyImplemented(old_s)
        && PowerDomainIsValid(old_s, domain_id)
        && (notify_enable & 0xFFFF_FFFEu32) == 0
    ))
    && (PowerStateNotifyImplemented(old_s)
        && PowerDomainIsValid(old_s, domain_id)
        && (notify_enable & 0xFFFF_FFFEu32) == 0
        && status == SUCCESS
        ==> (
            PowerStateNotifyEnabled(new_s, agent_id, domain_id) == ((notify_enable & 1u32) == 1u32)
            && UnchangedExceptPowerStateNotify(old_s, new_s, agent_id, domain_id)
        ))
}
