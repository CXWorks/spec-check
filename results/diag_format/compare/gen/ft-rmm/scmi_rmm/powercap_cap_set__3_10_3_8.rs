pub open spec fn powercap_cap_set__3_10_3_8_spec(domain_id: uint32, cpli: uint32, flags: uint32, power_cap: uint32, result: Result<int32, PowercapCapSetReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(result, NOT_FOUND))
  && (!IsRequestSupported(old_s, domain_id, cpli, flags, power_cap) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsSupportedPowerCap(old_s, domain_id, cpli, power_cap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidCapSetFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AgentMaySetPowerCap(old_s, agent, domain_id) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS && flags[1] == 0 ==> RequestedPowerCap(new_s, agent, domain_id, cpli) == power_cap)
  && (result == SUCCESS && flags[1] == 1 ==> CommandQueued(new_s, POWERCAP_CAP_SET, domain_id, cpli, power_cap))
  && (result == SUCCESS && flags[1] == 1 && flags[0] == 0 ==> DelayedResponseSent(new_s, POWERCAP_CAP_SET_COMPLETE))
  && (result == SUCCESS && flags[1] == 1 && flags[0] == 1 ==> !DelayedResponseSent(new_s, POWERCAP_CAP_SET_COMPLETE))
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       IsValidCpli(old_s, domain_id, cpli) &&
       IsRequestSupported(old_s, domain_id, cpli, flags, power_cap) &&
       IsSupportedPowerCap(old_s, domain_id, cpli, power_cap) &&
       IsValidCapSetFlags(old_s, flags) &&
       AgentMaySetPowerCap(old_s, agent, domain_id))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> RequestedPowerCap(new_s, agent, domain_id, cpli) == RequestedPowerCap(old_s, agent, domain_id, cpli))
  && (result != SUCCESS
    ==> !CommandQueued(new_s, POWERCAP_CAP_SET, domain_id, cpli, power_cap))
  && (result != SUCCESS
    ==> !DelayedResponseSent(new_s, POWERCAP_CAP_SET_COMPLETE))
  && (!(result == SUCCESS && (flags[1] == 1 && flags[0] == 1))
    ==> DelayedResponseSent(new_s, POWERCAP_CAP_SET_COMPLETE) == DelayedResponseSent(old_s, POWERCAP_CAP_SET_COMPLETE))
}