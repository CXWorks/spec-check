pub open spec fn mem_protect_spec(enable: UInt64, result: Int64, old_s: S, new_s: S) -> bool {
  (!MemProtectImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!Old(MemProtectEnabled(old_s)) ==> ResultEqual(result, 0))
  && (Old(MemProtectEnabled(old_s)) ==> ResultEqual(result, 1))
  && (MemProtectEnabled(new_s) == (enable != 0))
  && (MemProtectCheckRangeImplemented(new_s))
  && (MemProtectEnabled(new_s) ==> AllCallerAccessibleVolatileMemoryOverwrittenOnNextNonWarmResetBoot(new_s))
  && (MemProtectEnabled(new_s) ==> CallerMemoryPreservedOnSystemWarmReset(new_s))
  && (MemProtectEnabled(new_s) ==> MemProtectDisabledBeforeOsBootLoaderHandover(new_s))
  && ((MemProtectImplemented(old_s))
    ==> ResultEqual(result, 1))
}