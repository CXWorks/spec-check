pub open spec fn voltage_describe_levels__3_9_2_6_spec(domain_id: UInt32, level_index: UInt32, status: Int32, flags: UInt32, voltage: [Int32; 4], old_s: S, new_s: S) -> bool {
  (!VoltageDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidVoltageLevelIndex(old_s, domain_id, level_index) ==> ResultEqual(status, OUT_OF_RANGE))
  && (!IsRequestSupported(old_s, domain_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMayGetVoltageLevels(old_s, caller, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 15, 13) == 0)
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 12, 12) == 1 ==> Bits(flags, 31, 16) == 0)
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 12, 12) == 1 ==> Bits(flags, 11, 0) == 3)
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 12, 12) == 1 ==> Elem(voltage, 0) == LowestVoltageLevel(old_s, domain_id))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 12, 12) == 1 ==> Elem(voltage, 1) == HighestVoltageLevel(old_s, domain_id))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 12, 12) == 1 ==> Elem(voltage, 2) == VoltageStepSize(old_s, domain_id))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 12, 12) == 0 ==> (forall i: UInt32| i < Bits(flags, 11, 0) ==> Elem(voltage, i) == VoltageLevelAt(old_s, domain_id, level_index + i)))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 12, 12) == 0 ==> Bits(flags, 31, 16) == RemainingVoltageLevels(old_s, domain_id, level_index, Bits(flags, 11, 0)))
  && (ResultEqual(status, SUCCESS) ==> forall i: UInt32| i + 1 < Bits(flags, 11, 0) ==> Elem(voltage, i) < Elem(voltage, i + 1))
  && ((VoltageDomainExists(old_s, domain_id) &&
       IsValidVoltageLevelIndex(old_s, domain_id, level_index) &&
       IsRequestSupported(old_s, domain_id) &&
       AgentMayGetVoltageLevels(old_s, caller, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> Bits(flags, 15, 13) == 0)
  && (result != SUCCESS
    ==> Bits(flags, 12, 12) == 1 ==> Bits(flags, 31, 16) == 0)
  && (result != SUCCESS
    ==> Bits(flags, 12, 12) == 1 ==> Bits(flags, 11, 0) == 3)
  && (result != SUCCESS
    ==> Bits(flags, 12, 12) == 1 ==> Elem(voltage, 0) == LowestVoltageLevel(old_s, domain_id))
  && (result != SUCCESS
    ==> Bits(flags, 12, 12) == 1 ==> Elem(voltage, 1) == HighestVoltageLevel(old_s, domain_id))
  && (result != SUCCESS
    ==> Bits(flags, 12, 12) == 1 ==> Elem(voltage, 2) == VoltageStepSize(old_s, domain_id))
  && (result != SUCCESS
    ==> Bits(flags, 12, 12) == 0 ==> (forall i: UInt32| i < Bits(flags, 11, 0) ==> Elem(voltage, i) == VoltageLevelAt(old_s, domain_id, level_index + i)))
  && (result != SUCCESS
    ==> Bits(flags, 12, 12) == 0 ==> Bits(flags, 31, 16) == RemainingVoltageLevels(old_s, domain_id, level_index, Bits(flags, 11, 0)))
  && (result != SUCCESS
    ==> forall i: UInt32| i + 1 < Bits(flags, 11, 0) ==> Elem(voltage, i) < Elem(voltage, i + 1))
}