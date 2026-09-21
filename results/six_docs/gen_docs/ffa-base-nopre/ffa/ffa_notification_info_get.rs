pub open spec fn ffa_notification_info_get_spec(result: u32, old_s: S, new_s: S) -> bool {
    (result == 0x84000083 || result == 0xC4000083)
    && (result == 0x84000083 ==> (old_s.ffa_instance == FfaInstance::FFA_INSTANCE_NON_SECURE_VIRTUAL && (old_s.ffa_conduit == FfaConduit::FFA_CONDUIT_SMC || old_s.ffa_conduit == FfaConduit::FFA_CONDUIT_HVC)))
    && (result == 0xC4000083 ==> (old_s.ffa_instance == FfaInstance::FFA_INSTANCE_NON_SECURE_PHYSICAL && old_s.ffa_conduit == FfaConduit::FFA_CONDUIT_SMC))
    && (result == 0x84000083 || result == 0xC4000083)
}