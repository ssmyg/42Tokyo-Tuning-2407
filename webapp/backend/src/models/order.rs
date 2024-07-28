use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow, Clone, Debug)]
pub struct Order {
    pub id: i32,
    pub client_id: i32,
    pub dispatcher_id: Option<i32>,
    pub tow_truck_id: Option<i32>,
    pub status: String,
    pub node_id: i32,
    pub car_value: f64,
    pub order_time: DateTime<Utc>,
    pub completed_time: Option<DateTime<Utc>>,
}

#[derive(FromRow, Clone, Debug)]
pub struct Order2 {
    pub id: i32,
    pub client_id: i32,
    pub dispatcher_id: Option<i32>,
    pub tow_truck_id: Option<i32>,
    pub status: String,
    pub node_id: i32,
    pub car_value: f64,
    pub order_time: DateTime<Utc>,
    pub completed_time: Option<DateTime<Utc>>,
    pub client_username: Option<String>,
    // pub area_id: i32,
    pub dispatcher_user_id: Option<i32>,
    pub dispatcher_username: Option<String>,
    // pub driver_user_id: Option<i32>,
    // pub driver_username: Option<String>,
}
// order_area_id
// pub struct OrderDto {
//     pub id: i32,
//     pub client_id: i32,
//     pub client_username: Option<String>,
//     pub dispatcher_id: Option<i32>,
//     pub dispatcher_user_id: Option<i32>,
//     pub dispatcher_username: Option<String>,
//     pub tow_truck_id: Option<i32>,
//     pub driver_user_id: Option<i32>,
//     pub driver_username: Option<String>,
//     pub status: String,
//     pub node_id: i32,
//     pub area_id: i32,
//     pub car_value: f64,
//     pub order_time: DateTime<Utc>,
//     pub completed_time: Option<DateTime<Utc>>,
// }


#[derive(FromRow, Clone, Debug)]
pub struct CompletedOrder {
    pub id: i32,
    pub order_id: i32,
    pub tow_truck_id: i32,
    pub order_time: Option<DateTime<Utc>>,
    pub completed_time: DateTime<Utc>,
    pub car_value: f64,
}
