use std::collections::HashMap;

use project_tomoyo::Room;
use serde_json::Value;
use ws::Sender;

pub fn ws_send(sender: &Sender, json: &Value) {
    sender.send(json.to_string()).unwrap();

    // println!("===== MESSAGE SENT BY SERVER START =====");
    // println!("{}", serde_json::to_string_pretty(json).unwrap());
    // println!("=====  MESSAGE SENT BY SERVER END  =====");
}

pub fn room_kinds_count(rooms: &Vec<Room>) -> HashMap<String, u32> {
    let mut count: HashMap<String, u32> = HashMap::new();

    for room in rooms.iter() {
        count
            .entry(room.kind.clone())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    count
}
