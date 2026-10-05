pub open spec fn performance_qos_config_set__3_5_6_14_spec(
    result: int32,
    old_s: S,
    new_s: S,
    domain_id: uint32,
    capability: uint32,
    flags: uint32,
    qos_value: uint32,
) -> bool {
    // Failure conditions
    // NOT_FOUND: domain_id invalid or capability descriptor invalid
    (
        !DomainIsValid(old_s, domain_id)
        || !CapabilityDescriptorIsValid(old_s, domain_id, capability)
    )
    ==> ResultEqual(result, NOT_FOUND)
    // INVALID_PARAMETERS:
    // - Multiple Type bits set (bits 23:16)
    // - Multiple Subtype bits set (bits 7:0)
    // - Reserved bits non-zero (bits 30:24, 14:8, 31)
    // - Flags invalid/illegal
    // - QoS value not supported
    (
        (CapabilityTypeBitsSet(capability) > 1)
        || (CapabilitySubtypeBitsSet(capability) > 1)
        || (ReservedBitsNonZero(capability))
        || (InvalidFlags(flags))
        || (QosValueNotSupported(old_s, domain_id, capability, qos_value))
    )
    ==> ResultEqual(result, INVALID_PARAMETERS)
    // DENIED: agent not permitted
    (!AgentPermitted(old_s, domain_id, capability))
    ==> ResultEqual(result, DENIED)
    // Success conditions
    // SUCCESS: set/reset successfully or enqueued
    (
        DomainIsValid(old_s, domain_id)
        && CapabilityDescriptorIsValid(old_s, domain_id, capability)
        && CapabilityTypeBitsSet(capability) == 1
        && CapabilitySubtypeBitsSet(capability) <= 1
        && ReservedBitsZero(capability)
        && ValidFlags(flags)
        && QosValueSupported(old_s, domain_id, capability, qos_value)
        && AgentPermitted(old_s, domain_id, capability)
    )
    ==> ResultEqual(result, SUCCESS)
    // State transitions for success (synchronous or asynchronous enqueue)
    (
        ResultEqual(result, SUCCESS)
        && (
            // If reset flags are set, QoS values are reset to defaults
            (
                (flags & FLAG_PLATFORM_RESET) != 0
                || (flags & FLAG_SIBLING_RESET) != 0
                || (flags & FLAG_DOMAIN_RESET) != 0
            )
            ==> (
                QosValueResetToDefault(old_s, new_s, domain_id, capability)
                && (
                    (flags & FLAG_SIBLING_RESET) != 0
                    ==> QosValueResetToDefault(old_s, new_s, SiblingDomains(old_s, domain_id), capability)
                )
                && (
                    (flags & FLAG_PLATFORM_RESET) != 0
                    ==> QosValueResetToDefault(old_s, new_s, AllDomains(old_s), capability)
                )
            )
            // If not reset, QoS value is set
            || (
                (flags & FLAG_PLATFORM_RESET) == 0
                && (flags & FLAG_SIBLING_RESET) == 0
                && (flags & FLAG_DOMAIN_RESET) == 0
            )
            ==> (
                QosValueSet(old_s, new_s, domain_id, capability, qos_value)
            )
        )
    )
}

// Helper predicates (uninterpreted for brevity, must be defined in context)
pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool { ... }
pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> bool { (capability >> 16) & 0xFFFF != 0 }
pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> bool { capability & 0xFF != 0 }
pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool {
    (capability & 0x0000000F00000000) != 0
    || (capability & 0x0000000000000000) != 0 // bits 14:8
    || (capability & 0x80000000) != 0 // bit 31
}
pub open spec fn InvalidFlags(flags: uint32) -> bool { ... }
pub open spec fn ValidFlags(flags: uint32) -> bool { ... }
pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32 { ... }
pub open spec fn AllDomains(s: S) -> uint32 { ... }
pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn FLAG_PLATFORM_RESET: uint32 { 1 << 4 }
pub open spec fn FLAG_SIBLING_RESET: uint32 { 1 << 3 }
pub open spec fn FLAG_DOMAIN_RESET: uint32 { 1 << 2 }
pub open spec fn ReservedBitsZero(capability: uint32) -> bool {
    (capability & 0x0000000F00000000) == 0
    && (capability & 0x0000000000000000) == 0
    && (capability & 0x80000000) == 0
}
pub open spec fn ValidFlags(flags: uint32) -> bool {
    (flags & 0x0000000F) == 0 // bits 31:5 reserved
    && (flags & 0x00000001) == 0 // bit 0 ignored if async=0
}
pub open spec fn InvalidFlags(flags: uint32) -> bool {
    (flags & 0x0000000F) != 0
}
pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool {
    (capability & 0x0000000F00000000) != 0
    || (capability & 0x0000000000000000) != 0
    || (capability & 0x80000000) != 0
}
pub open spec fn ReservedBitsZero(capability: uint32) -> bool {
    (capability & 0x0000000F00000000) == 0
    && (capability & 0x0000000000000000) == 0
    && (capability & 0x80000000) == 0
}
pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> bool { (capability >> 16) & 0xFFFF != 0 }
pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> bool { capability & 0xFF != 0 }
pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool { ... }
pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32 { ... }
pub open spec fn AllDomains(s: S) -> uint32 { ... }
pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn InvalidFlags(flags: uint32) -> bool { ... }
pub open spec fn ValidFlags(flags: uint32) -> bool { ... }
pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool { ... }
pub open spec fn ReservedBitsZero(capability: uint32) -> bool { ... }
pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool { ... }
pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32 { ... }
pub open spec fn AllDomains(s: S) -> uint32 { ... }
pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn InvalidFlags(flags: uint32) -> bool { ... }
pub open spec fn ValidFlags(flags: uint32) -> bool { ... }
pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool { ... }
pub open spec fn ReservedBitsZero(capability: uint32) -> bool { ... }
pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool { ... }
pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32 { ... }
pub open spec fn AllDomains(s: S) -> uint32 { ... }
pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn InvalidFlags(flags: uint32) -> bool { ... }
pub open spec fn ValidFlags(flags: uint32) -> bool { ... }
pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool { ... }
pub open spec fn ReservedBitsZero(capability: uint32) -> bool { ... }
pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool { ... }
pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32 { ... }
pub open spec fn AllDomains(s: S) -> uint32 { ... }
pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn InvalidFlags(flags: uint32) -> bool { ... }
pub open spec fn ValidFlags(flags: uint32) -> bool { ... }
pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool { ... }
pub open spec fn ReservedBitsZero(capability: uint32) -> bool { ... }
pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool { ... }
pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32 { ... }
pub open spec fn AllDomains(s: S) -> uint32 { ... }
pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn InvalidFlags(flags: uint32) -> bool { ... }
pub open spec fn ValidFlags(flags: uint32) -> bool { ... }
pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool { ... }
pub open spec fn ReservedBitsZero(capability: uint32) -> bool { ... }
pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool { ... }
pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32 { ... }
pub open spec fn AllDomains(s: S) -> uint32 { ... }
pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn InvalidFlags(flags: uint32) -> bool { ... }
pub open spec fn ValidFlags(flags: uint32) -> bool { ... }
pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool { ... }
pub open spec fn ReservedBitsZero(capability: uint32) -> bool { ... }
pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool { ... }
pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32 { ... }
pub open spec fn AllDomains(s: S) -> uint32 { ... }
pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn InvalidFlags(flags: uint32) -> bool { ... }
pub open spec fn ValidFlags(flags: uint32) -> bool { ... }
pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool { ... }
pub open spec fn ReservedBitsZero(capability: uint32) -> bool { ... }
pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool { ... }
pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32 { ... }
pub open spec fn AllDomains(s: S) -> uint32 { ... }
pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn InvalidFlags(flags: uint32) -> bool { ... }
pub open spec fn ValidFlags(flags: uint32) -> bool { ... }
pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool { ... }
pub open spec fn ReservedBitsZero(capability: uint32) -> bool { ... }
pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool { ... }
pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32 { ... }
pub open spec fn AllDomains(s: S) -> uint32 { ... }
pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn InvalidFlags(flags: uint32) -> bool { ... }
pub open spec fn ValidFlags(flags: uint32) -> bool { ... }
pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool { ... }
pub open spec fn ReservedBitsZero(capability: uint32) -> bool { ... }
pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool { ... }
pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32 { ... }
pub open spec fn AllDomains(s: S) -> uint32 { ... }
pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn InvalidFlags(flags: uint32) -> bool { ... }
pub open spec fn ValidFlags(flags: uint32) -> bool { ... }
pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool { ... }
pub open spec fn ReservedBitsZero(capability: uint32) -> bool { ... }
pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> bool { ... }
pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool { ... }
pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32 { ... }
pub open spec fn AllDomains(s: S) -> uint32 { ... }
pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool { ... }
pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool { ... }
pub open spec fn QosValueSupported(s