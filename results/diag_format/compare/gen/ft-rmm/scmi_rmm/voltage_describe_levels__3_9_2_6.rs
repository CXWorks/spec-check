pub open spec fn voltage_describe_levels__3_9_2_6_spec(domain_id: uint32, level_index: uint32, status: int32, flags: uint32, voltage: [int32; 3], old_s: S, new_s: S) -> bool {
  (!VoltageDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidVoltageLevelIndex(old_s, domain_id, level_index) ==> ResultEqual(status, OUT_OF_RANGE))
  && (!IsRequestSupported(old_s, VOLTAGE_DESCRIBE_LEVELS) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMayGetVoltageLevels(old_s, calling_agent, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> flags[15:13] == 0)
  && (ResultEqual(status, SUCCESS) ==>  == flags[11:0])
  && (ResultEqual(status, SUCCESS) ==> flags[12] == 1 ==> (flags[11:0] == 3 && flags[31:16] == 0 && voltage[0] == LowestVoltageLevel(old_s, domain_id) && voltage[1] == HighestVoltageLevel(old_s, domain_id) && voltage[2] == VoltageStepSize(old_s, domain_id)))
  && (ResultEqual(status, SUCCESS) ==> flags[12] == 0 ==> for each i < 3: IsSupportedVoltageLevel(old_s, domain_id, voltage[i]))
  && (ResultEqual(status, SUCCESS) ==> voltage[0..2] should be in ascending numeric order)
  && ((VoltageDomainExists(old_s, domain_id) &&
       IsValidVoltageLevelIndex(old_s, domain_id, level_index) &&
       IsRequestSupported(old_s, VOLTAGE_DESCRIBE_LEVELS) &&
       AgentMayGetVoltageLevels(old_s, calling_agent, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> flags[15:13] == 0)
  && (result != SUCCESS
    ==>  == flags[11:0])
  && (result != SUCCESS
    ==> flags[12] == 0 ==> for each i < 3: IsSupportedVoltageLevel(old_s, domain_id, voltage[i]))
  && (result != SUCCESS
    ==> voltage[0..2] should be in ascending numeric order)
}