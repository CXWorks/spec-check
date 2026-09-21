pub open spec fn 3.9.2.11_voltage_domain_name_get_spec(domain_id: UInt32, status: int, flags: UInt32, name: [uint8; 64], old_s: S, new_s: S) -> bool {
  (flags != 0 ==> false)
}