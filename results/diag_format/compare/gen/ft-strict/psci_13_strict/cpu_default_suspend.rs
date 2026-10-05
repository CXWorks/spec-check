pub open spec fn cpu_default_suspend_spec(entry_point_address: Address, context_id: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (IsKnownUnavailableAddress(old_s, entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
  && (ReturnedAtNextInstruction(old_s, calling_cpu) ==> ResultEqual(result, SUCCESS))
  && (ResumedAtEntryPoint(old_s, calling_cpu) ==> CoreRestartsAtEntryPoint(new_s, calling_cpu, entry_point_address))
  && (ResumedAtEntryPoint(old_s, calling_cpu) ==> ContextIdPresented(new_s, calling_cpu, context_id))
  && (CacheAndCoherencyManagedByImplementation(old_s, calling_cpu))
  && (AllCoresInDefaultSuspend(old_s) ==> !PlatformThermallyCritical(new_s))
  && ((!IsKnownUnavailableAddress(old_s, entry_point_address))
    ==> ResultEqual(result, SUCCESS))
  && (!(ReturnedAtNextInstruction(old_s, calling_cpu))
    ==> ResultEqual(result, SUCCESS))
  && (!(ResumedAtEntryPoint(old_s, calling_cpu))
    ==> ResultEqual(result, SUCCESS))
  && (!(CacheAndCoherencyManagedByImplementation(old_s, calling_cpu))
    ==> ResultEqual(result, SUCCESS))
  && (!(AllCoresInDefaultSuspend(old_s))
    ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS
    ==> PowerState(new_s, calling_cpu) == PowerState(old_s, calling_cpu))
  && (result == SUCCESS
    ==> PowerState(new_s, TopologyAncestors(new_s, calling_cpu)) == PowerState(old_s, TopologyAncestors(old_s, calling_cpu)))
}