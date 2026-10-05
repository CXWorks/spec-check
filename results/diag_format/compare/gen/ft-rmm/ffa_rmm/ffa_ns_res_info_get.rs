pub open spec fn ffa_ns_res_info_get_spec(target_id: UInt16, flags: UInt64, result: Result<FFAReturnCode, Int32>, remaining_size: UInt32, written_size: UInt32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NS_RES_INFO_GET) ==> ResultEqual(result, NOT_SUPPORTED))
  && (target_id[63:16] != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (flags[0] == 0 && target_id[15:0] != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (flags[63:5] != 0 || flags[1] != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (flags[3:2] != 0b00 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AreReservedRegistersZero(old_s, 3, 17) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (flags[0] == 1 && !IsValidSEndpointId(old_s, target_id[15:0]) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsRxBufferMappedInCallee(old_s, caller) ==> ResultEqual(result, RETRY))
  && (!IsRxBufferOwnedByCallee(old_s, caller) ==> ResultEqual(result, RETRY))
  && (IsCalleeBusy(old_s) ==> ResultEqual(result, RETRY))
  && (flags[4] == 1 && !CanContinueRetrieval(old_s) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> RxBuffer(new_s, caller) holds written_size bytes of the Resource information descriptor)
  && (result == FFA_SUCCESS ==> remaining_size == number of bytes of the Resource information descriptor not yet sent to the caller)
  && (result == FFA_SUCCESS ==> flags[0] == 1 ==> the descriptor holds one AMD for each region in the NS PAS that S-Endpoint target_id[15:0] can access)
  && (result == FFA_SUCCESS ==> flags[0] == 0 ==> the descriptor holds one AMD for each region in the NS PAS that each S-Endpoint can access)
  && (result == FFA_SUCCESS ==> for each AMD describing a region with indirect access: AMD.Flags.AccessType == 1 && AMD.ComponentId == SpmcId() && AMD.RAP == MostPermissiveSpPermission(new_s, region))
  && (result == FFA_SUCCESS ==> for each S-Endpoint that can access the entire NS PAS with Privileged and Unprivileged Read, Write and Execute permissions: AMD.Address == 0xFFFFFFFFFFFFFFFF && AMD.PageCount == 0 && AMD.RAP == 0x77)
  && (result == FFA_SUCCESS ==> for each AMD describing a Static NS region of a physical S-EL1 SP running under an S-EL2 SPMC: AMD.RAP == Stage2BasePermsToRap(new_s, region))
  && (result == FFA_SUCCESS ==> for each AMD describing a Static NS region of a physical S-EL0 SP running under any SPMC: AMD.RAP holds the Unprivileged Stage 1 Base permissions in the EL1&0 or EL2&0 translation regime)
  && (result == FFA_SUCCESS ==> (flags[3:2] == 0b00 && ((flags[0] == 1 && IsNsPasInaccessibleFrom(new_s, target_id[15:0])) || (flags[0] == 0 && IsNsPasInaccessibleFromAll(new_s)))) ==> written_size == 0 && remaining_size == 0)
  && ((!(IsImplementedAtInstance(old_s, FFA_NS_RES_INFO_GET)) &&
       !(target_id[63:16] != 0) &&
       !(flags[0] == 0 && target_id[15:0] != 0) &&
       !(flags[63:5] != 0 || flags[1] != 0) &&
       !(flags[3:2] != 0b00) &&
       AreReservedRegistersZero(old_s, 3, 17) &&
       !(flags[0] == 1 && !IsValidSEndpointId(old_s, target_id[15:0])) &&
       IsRxBufferMappedInCallee(old_s, caller) &&
       IsRxBufferOwnedByCallee(old_s, caller) &&
       !IsCalleeBusy(old_s) &&
       !(flags[4] == 1 && !CanContinueRetrieval(old_s)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> remaining_size == ResourceInfoRetrievalState(new_s, caller))
  && (result != FFA_SUCCESS
    ==> written_size == 0)
}