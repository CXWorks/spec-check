pub open spec fn sensor_trip_point_event__3_7_4_1_spec(agent_id: u32, sensor_id: u32, trip_point_desc: u32, result: ()) -> bool {
    (TripPointEventRequested(sensor_id, trip_point_desc as int) ==> true)
    && (SensorCrossedOrReachedTripPoint(sensor_id, trip_point_desc as int) ==> true)
    && (agent_id == 0)
    && ((trip_point_desc as int & 0x10000) == 1 ==> TripPointCrossedPositive(sensor_id, trip_point_desc as int))
    && ((trip_point_desc as int & 0x10000) == 0 ==> TripPointCrossedNegative(sensor_id, trip_point_desc as int))
}