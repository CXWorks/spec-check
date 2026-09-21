pub open spec fn 3.10.3.16_powercap_measurements_notify_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (result == SCMI_NOT_FOUND ==> old_s.domain_id_is_valid(old_s, old_s.domain_id) == false)
    && (result == SCMI_INVALID_PARAMETERS ==> (old_s.notify_enable != 0 && old_s.notify_enable != 1) || old_s.power_thresh_low < 0 || old_s.power_thresh_high < 0)
}