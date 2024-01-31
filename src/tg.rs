pub enum TgResponse {
    Hello,
    IncorrectRequest,
    ChosenTimezone(String),
    FailedSetTimezone(String),
}

impl From<TgResponse> for String {
    fn from(value: TgResponse) -> Self {
        match value {
            TgResponse::Hello => concat!(
            "Hello! I'm Rações bot. My purpose is to track how much ration you have given your pet! \n\n",
            "Before we start, please either send me your location or manually select the timezone using the /settimezone command first."
            ).to_owned(),
            TgResponse::IncorrectRequest => "Incorrect request".to_owned(),
            TgResponse::ChosenTimezone(tz_name) => format!("Selected timezone: {}!", tz_name),
            TgResponse::FailedSetTimezone(tz_name) => format!("Failed to set timezone {}", tz_name),
        }
    }
}
