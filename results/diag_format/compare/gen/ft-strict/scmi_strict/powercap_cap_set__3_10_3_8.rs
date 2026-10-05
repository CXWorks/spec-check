pub open spec fn powercap_cap_set__3_10_3_8_spec(domain_id: UInt32, cpli: UInt32, flags: UInt32, power_cap: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(status, NOT_FOUND))
  && (!IsPowercapCapSetSupported(old_s, domain_id, cpli) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!IsSupportedPowerCap(old_s, domain_id, cpli, power_cap) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (Bits(flags, 31, 2) != 0 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsValidPowercapCapSetFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!AgentMaySetPowerCap(old_s, caller, domain_id) ==> ResultEqual(status, DENIED))
  && (result == SUCCESS ==> AgentRequestedPowerCap(new_s, caller, domain_id, cpli) == power_cap)
  && (result == SUCCESS && power_cap == 0 ==> !AgentPowerCapEnabled(new_s, caller, domain_id, cpli))
  && (result == SUCCESS && Bits(flags, 1, 1) == 0 ==> PowerCapSettingCompleted(new_s, domain_id, cpli, power_cap))
  && (result == SUCCESS && Bits(flags, 1, 1) == 1 ==> PowerCapSetRequestQueued(new_s, domain_id, cpli, power_cap))
  && (result == SUCCESS && Bits(flags, 1, 1) == 1 && Bits(flags, 0, 0) == 0 ==> DelayedResponsePending(new_s, POWERCAP_CAP_SET_COMPLETE, domain_id))
  && (result == SUCCESS && Bits(flags, 1, 1) == 1 && Bits(flags, 0, 0) == 1 ==> !DelayedResponsePending(new_s, POWERCAP_CAP_SET_COMPLETE, domain_id))
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       IsValidCpli(old_s, domain_id, cpli) &&
       IsPowercapCapSetSupported(old_s, domain_id, cpli) &&
       IsSupportedPowerCap(old_s, domain_id, cpli, power_cap) &&
       !(Bits(flags, 31, 2) != 0) &&
       IsValidPowercapCapSetFlags(old_s, flags) &&
       AgentMaySetPowerCap(old_s, caller, domain_id))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> AgentRequestedPowerCap(new_s, caller, domain_id, cpli) == AgentRequestedPowerCap(old_s, caller, domain_id, cpli))
  && (result != SUCCESS
    ==> AgentPowerCapEnabled(new_s, caller, domain_id, cpli) == AgentPowerCapEnabled(old_s, caller, domain_id, cpli))
  && (result != SUCCESS
    ==> PowerCapSettingCompleted(new_s, domain_id, cpli, power_cap) == PowerCapSettingCompleted(old_s, domain_id, cpli, power_cap))
  && (result != SUCCESS
    ==> PowerCapSetRequestQueued(new_s, domain_id, cpli, power_cap) == PowerCapSetRequestQueued(old_s, domain_id, cpli, power_cap))
  && (result != SUCCESS
    ==> DelayedResponsePending(new_s, POWERCAP_CAP_SET_COMPLETE, domain_id) == DelayedResponsePending(old_s, POWERCAP_CAP_SET_COMPLETE, domain_id))
  && (!(result == SUCCESS && (Bits(flags, 1, 1) == 1 && Bits(flags, 0, 0) == 1))
    ==> DelayedResponsePending(new_s, POWERCAP_CAP_SET_COMPLETE, domain_id) == DelayedResponsePending(old_s, POWERCAP_CAP_SET_COMPLETE, domain_id))
}