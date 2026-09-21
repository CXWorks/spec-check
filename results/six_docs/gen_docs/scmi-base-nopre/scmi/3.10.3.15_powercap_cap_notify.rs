pub open spec fn 3.10.3.15_powercap_cap_notify_spec(result: int32, old_s: S, new_s: S) -> bool {
    (old_s.powercap_notify_enabled == 0 ==> result == 0)
    && (result != 0 ==> (result == -1 || result == -2))
    && (result == 0 ==> (old_s.powercap_notify_enabled == 1))
    && (result == -1 ==> (old_s.powercap_notify_enabled == 0))
    && (result == -2 ==> (old_s.powercap_notify_enabled == 0))
    && (old_s.powercap_notify_enabled == 1 ==> new_s.powercap_notify_enabled == 1)
    && (old_s.powercap_notify_enabled == 0 ==> new_s.powercap_notify_enabled == 0)
}