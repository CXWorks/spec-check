pub open spec fn drtm_unprotect_memory_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == DENIED && (there_are_memory_protections_in_place(old_s)) ==> result == DENIED)
  && ((!(there_are_memory_protections_in_place(old_s))) ==> result == SUCCESS)
  && (result == SUCCESS && (there_are_memory_protections_in_place(old_s))) ==> result == SUCCESS
  && ((!(result == DENIED) && !(result == SUCCESS)) ==> result == NOT_SUPPORTED)
  && ((result == SUCCESS) ==> SmmuConfig(new_s) == SmmuConfig(old_s))
}