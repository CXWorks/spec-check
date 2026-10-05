pub open spec fn powercap_measurements_get__3_10_3_14_spec(result: int32, power: uint32, mai: uint32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (power == old_s.powercap_measurements_get.power && mai == old_s.powercap_measurements_get.mai))
    && (result != 0 ==> (power == 0 && mai == 0))
}