pub open spec fn ffa_version_spec(input_version: UInt32, major_version: UInt32, minor_version: UInt32, flags: UInt32, result: Int32, old_s: S, new_s: S) -> bool {
  (!CalleeImplementsFfa(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (input_version[31] != 0 ==> ResultEqual(result, INVALID_PARAMETER))
  && (!IsValidVersionQueryType(flags[1:0]) ==> ResultEqual(result, INVALID_PARAMETER))
  && (result[31] == 0)
  && ((flags[1:0] == 0b00) && !CallerIsSpmcAtSeparateEl(old_s) && FfaInUse(caller(old_s)) ==> (result == NULL_VERSION) && (NegotiatedVersion(caller(new_s)) == NegotiatedVersion(caller(old_s))))
  && ((flags[1:0] == 0b00) && !CallerIsSpmcAtSeparateEl(old_s) && !FfaInUse(caller(old_s)) && VersionLessThan(input_version, NegotiatedVersion(caller(old_s))) && !CalleeAllowsDowngrade(old_s) ==> (result == NULL_VERSION) && (NegotiatedVersion(caller(new_s)) == NegotiatedVersion(caller(old_s))))
  && ((flags[1:0] == 0b00) && !CallerIsSpmcAtSeparateEl(old_s) && !FfaInUse(caller(old_s)) && !(VersionLessThan(input_version, NegotiatedVersion(caller(old_s))) && !CalleeAllowsDowngrade(old_s)) && CalleeImplementsCompatibleVersion(old_s, input_version) ==> IsCompatible(result, input_version) && !VersionLessThan(result, input_version) && (NegotiatedVersion(caller(new_s)) == input_version))
  && ((flags[1:0] == 0b00) && !CallerIsSpmcAtSeparateEl(old_s) && CalleeIncompatibleVersionsOnlyLower(old_s, input_version) ==> (result == HighestIncompatibleVersion(old_s, input_version)) && (NegotiatedVersion(caller(new_s)) == NegotiatedVersion(caller(old_s))))
  && ((flags[1:0] == 0b00) && !CallerIsSpmcAtSeparateEl(old_s) && CalleeIncompatibleVersionsOnlyHigher(old_s, input_version) ==> ((result == LowestIncompatibleVersion(old_s, input_version)) || ResultEqual(result, NOT_SUPPORTED)) && (NegotiatedVersion(caller(new_s)) == NegotiatedVersion(caller(old_s))))
  && ((flags[1:0] == 0b00) && !CallerIsSpmcAtSeparateEl(old_s) && CalleeIncompatibleVersionsHigherAndLower(old_s, input_version) ==> (result == HighestIncompatibleVersion(old_s, input_version)) && (NegotiatedVersion(caller(new_s)) == NegotiatedVersion(caller(old_s))))
  && ((flags[1:0] == 0b00) && CallerIsSpmcAtSeparateEl(old_s) ==> (result == NegotiatedVersionAtStart(old_s, SPMC)) && (NegotiatedVersion(SPMC(new_s)) == NegotiatedVersion(SPMC(old_s))))
  && ((flags[1:0] == 0b01) && CalleeImplementsCompatibleVersion(old_s, input_version) ==> IsCompatible(result, input_version) && !VersionLessThan(result, input_version))
  && ((flags[1:0] == 0b01) && CalleeIncompatibleVersionsOnlyLower(old_s, input_version) ==> (result == HighestIncompatibleVersion(old_s, input_version)))
  && ((flags[1:0] == 0b01) && CalleeIncompatibleVersionsOnlyHigher(old_s, input_version) ==> (result == LowestIncompatibleVersion(old_s, input_version)))
  && ((flags[1:0] == 0b01) && CalleeIncompatibleVersionsHigherAndLower(old_s, input_version) ==> (result == HighestIncompatibleVersion(old_s, input_version)))
  && ((flags[1:0] == 0b10) ==> (result == NegotiatedVersion(caller(new_s))))
  && ((flags[1:0] == 0b10) && !VersionNegotiated(caller(old_s)) ==> (result == NULL_VERSION))
  && ((flags[1:0] != 0b00) ==> (NegotiatedVersion(caller(new_s)) == NegotiatedVersion(caller(old_s))))
  && ((!(CalleeImplementsFfa(old_s)) &&
       !(input_version[31] != 0) &&
       IsValidVersionQueryType(flags[1:0]))
    ==> result == 0)
}