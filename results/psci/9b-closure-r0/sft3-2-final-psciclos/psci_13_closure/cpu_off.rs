pub open spec fn cpu_off_spec(old_s: S, new_s: S) -> bool {
  true
}

pub open spec fn cpu_on_spec(target_cpu: UInt32, entry_point_address: UInt32, context_id: UInt32, old_s: S, new_s: S) -> bool {
  true
}