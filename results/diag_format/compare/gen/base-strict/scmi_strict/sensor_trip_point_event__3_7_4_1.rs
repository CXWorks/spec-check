pub open spec fn sensor_trip_point_event__3_7_4_1_spec(agent_id: UInt32, sensor_id: UInt32, trip_point_desc: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS)
    && (agent_id == 0)
    && (TripPointEventRequested(old_s, sensor_id, Bits(trip_point_desc, 7, 0)))
    && (TripPointReachedOrCrossed(old_s, sensor_id, Bits(trip_point_desc, 7, 0)))
    && ((Bits(trip_point_desc, 16, 16) == 1) ==> TripPointCrossedInPositiveDirection(old_s, sensor_id, Bits(trip_point_desc, 7, 0)))
    && ((Bits(trip_point_desc, 16, 16) == 0) ==> TripPointCrossedInNegativeDirection(old_s, sensor_id, Bits(trip_point_desc, 7, 0)))
    && (MultipleTripPointCrossingsMayBeReportedByOneNotification(old_s, sensor_id))
    && (old_s == new_s)
}