pub open spec fn ffa_ns_res_info_get_spec(result: u32, error_code: i32, remaining_size: u32, written_size: u32, old_s: S, new_s: S) -> bool {
    // Failure conditions
    (!IsImplementedAtInstance(FFA_NS_RES_INFO_GET) ==> ResultEqual(result, NOT_SUPPORTED))
    && (target_id[63:16] != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags[0] == 0 && target_id[15:0] != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags[63:5] != 0 || flags[1] != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags[3:2] != 0b00 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!AreReservedRegistersZero(X3, X17) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags[0] == 1 && !IsValidSEndpointId(target_id[15:0]) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsRxBufferMappedInCallee(caller) ==> ResultEqual(result, RETRY))
    && (!IsRxBufferOwnedByCallee(caller) ==> ResultEqual(result, RETRY))
    && (IsCalleeBusy() ==> ResultEqual(result, RETRY))
    && (flags[4] == 1 && !CanContinueRetrieval() ==> ResultEqual(result, ABORTED))
    // Success conditions
    && (ResultEqual(result, FFA_SUCCESS) ==> (
        RxBuffer(caller) holds written_size bytes of the Resource information descriptor
        && remaining_size == number of bytes of the Resource information descriptor not yet sent to the caller
        && (flags[0] == 1 ==> the descriptor holds one AMD for each region in the NS PAS that S-Endpoint target_id[15:0] can access)
        && (flags[0] == 0 ==> the descriptor holds one AMD for each region in the NS PAS that each S-Endpoint can access)
        && (for each AMD describing a region with indirect access: AMD.Flags.AccessType == 1 && AMD.ComponentId == SpmcId() && AMD.RAP == MostPermissiveSpPermission(region))
        && (for each S-Endpoint that can access the entire NS PAS with Privileged and Unprivileged Read, Write and Execute permissions: AMD.Address == 0xFFFFFFFFFFFFFFFF && AMD.PageCount == 0 && AMD.RAP == 0x77)
        && (for each AMD describing a Static NS region of a physical S-EL1 SP running under an S-EL2 SPMC: AMD.RAP == Stage2BasePermsToRap(region), using the mapping in Table 13.65)
        && (for each AMD describing a Static NS region of a physical S-EL0 SP running under any SPMC: AMD.RAP holds the Unprivileged Stage 1 Base permissions in the EL1&0 or EL2&0 translation regime)
        && ((flags[3:2] == 0b00 && ((flags[0] == 1 && IsNsPasInaccessibleFrom(target_id[15:0])) || (flags[0] == 0 && IsNsPasInaccessibleFromAll()))) ==> written_size == 0 && remaining_size == 0)
    ))
}