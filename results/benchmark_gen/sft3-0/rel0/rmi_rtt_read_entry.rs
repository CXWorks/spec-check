pub open spec fn rmi_rtt_read_entry_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, walk_level: UInt64, state: RmiRttEntryState, desc: Bits64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttLevelIsValid(old_s, rd, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> state == RttEntryState(new_s, RttEntryFromDescriptor(new_s, desc,→).state))
  && (result.is_Ok() && (RttEntryFromDescriptor(old_s, RttEntryFromDescriptor(old_s, desc,→).state == UNASSIGNED || RttEntryFromDescriptor(old_s, RttEntryFromDescriptor(old_s, desc,→).state == UNASSIGNED_NS) ==> (RttEntryFromDescriptor(new_s, desc,→).MemAttr == 0 && RttEntryFromDescriptor(new_s, desc,→).S2AP == 0 && RttEntryFromDescriptor(new_s, desc,→).addr == 0)))
  && (result.is_Ok() && (RttEntryFromDescriptor(old_s, RttEntryFromDescriptor(old_s, desc,→).state == ASSIGNED || RttEntryFromDescriptor(old_s, RttEntryFromDescriptor(old_s, desc,→).state == TABLE) ==> (RttEntryFromDescriptor(new_s, desc,→).MemAttr == 0 && RttEntryFromDescriptor(new_s, desc,→).S2AP == 0 && RttEntryFromDescriptor(new_s, desc,→).addr == RttEntryFromDescriptor(new_s, desc,→).addr)))
  && (result.is_Ok() && RttEntryFromDescriptor(old_s, RttEntryFromDescriptor(old_s, desc,→).state == ASSIGNED_NS ==> (RttEntryFromDescriptor(new_s, desc,→).MemAttr == RttEntryFromDescriptor(new_s, desc,→).MemAttr && RttEntryFromDescriptor(new_s, desc,→).S2AP == RttEntryFromDescriptor(new_s, desc,→).S2AP && RttEntryFromDescriptor(new_s, desc,→).addr == RttEntryFromDescriptor(new_s, desc,→).addr)))
  && (result.is_Ok() && (RttEntryFromDescriptor(old_s, RttEntryFromDescriptor(old_s, desc,→).state == UNASSIGNED || RttEntryFromDescriptor(old_s, RttEntryFromDescriptor(old_s, desc,→).state == ASSIGNED) ==> ripas == RipasToRmi(new_s, RttEntryFromDescriptor(new_s, desc,→).ripas))
  && (result.is_Ok() && (RttEntryFromDescriptor(old_s, RttEntryFromDescriptor(old_s, desc,→).state == UNASSIGNED_NS || RttEntryFromDescriptor(old_s, RttEntryFromDescriptor(old_s, desc,→).state == ASSIGNED_NS) ==> ripas == RMI_EMPTY))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       RttLevelIsValid(old_s, rd, level as int) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       !((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> state == RttEntryFromDescriptor(new_s, desc,→).state)
  && (result.is_Err()
    ==> desc == RttEntryFromDescriptor(new_s, desc,→).desc)
  && (result.is_Err()
    ==> ripas == RttEntryFromDescriptor(new_s, desc,→).ripas)
}