use serde_json;

use crate::ws::schemas::{
    EventTypeJson, 
    EventJson, 
    EventContentJson, 
    ExtractError, 
    NotificationJson, 
    MessageJson,
};


impl EventJson {
    pub fn extract_event(&self) -> Result<EventContentJson, ExtractError> {
        let res = match self.event_type {
            EventTypeJson::Message => {
                serde_json::from_str::<MessageJson>(&self.event_content)
                    .map(|res| EventContentJson::Message(res))
            },
            EventTypeJson::Notification => {
                serde_json::from_str::<NotificationJson>(&self.event_content)
                    .map(|res| EventContentJson::Notification(res))
            },
        };

        res.map_err(|err| ExtractError {
            message: format!("Failed to extract event: {}", err),
        })
    }
}