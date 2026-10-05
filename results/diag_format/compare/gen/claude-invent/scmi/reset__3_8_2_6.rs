pub open spec fn reset__3_8_2_6_spec(status: i32, old_s: S, new_s: S, agent_id: u32, domain_id: u32, flags: u32, reset_state: u32) -> bool {
    (!ResetDomainExists(old_s, domain_id) ==> status != 0i32)
    && (status == -4i32 ==> !ResetDomainExists(old_s, domain_id))
    && (((flags & 0xFFFF_FFF8u32) != 0u32 || !ResetStateSupported(old_s, domain_id, reset_state)) ==> status != 0i32)
    && (status == -2i32 ==> ((flags & 0xFFFF_FFF8u32) != 0u32 || !ResetStateSupported(old_s, domain_id, reset_state)))
    && (!AgentMayResetDomain(old_s, agent_id, domain_id) ==> status != 0i32)
    && (status == -3i32 ==> !AgentMayResetDomain(old_s, agent_id, domain_id))
    && (status != 0i32 ==> new_s == old_s)
    && (status == 0i32 ==> (
        ResetDomainExists(old_s, domain_id)
        && (flags & 0xFFFF_FFF8u32) == 0u32
        && ResetStateSupported(old_s, domain_id, reset_state)
        && AgentMayResetDomain(old_s, agent_id, domain_id)
        && OtherResetDomainsUnchanged(old_s, new_s, domain_id)
        && (((flags & 1u32) == 1u32 && (flags & 4u32) == 4u32) ==> ResetCompletePending(new_s, agent_id, domain_id))
        && (((flags & 1u32) == 1u32 && (flags & 4u32) == 0u32) ==> DomainResetPerformed(old_s, new_s, domain_id, reset_state))
        && ((flags & 1u32) == 0u32 ==> (ResetSignalAsserted(new_s, domain_id) == ((flags & 2u32) == 2u32)))
    ))
}
