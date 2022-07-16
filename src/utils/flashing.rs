use actix_web_flash_messages::IncomingFlashMessages;

pub fn extract_flash_message(flashed_messages: &IncomingFlashMessages) -> Option<&str> {
    flashed_messages.iter().next().map(|e| e.content())
}
