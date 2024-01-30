use teloxide::utils::markdown::escape;

pub enum TgResponse {
    Hello,
    IncorrectRequest,
    ChosenTimezone(String),
    FailedSetTimezone(String),
}

impl TgResponse {
    pub fn to_unescaped_string(&self) -> String {
        match self {
            Self::Hello => concat!(
            "Hello! I'm Rações bot. My purpose is to track how much ration you have given your pet! \n\n",
            "Before we start, please either send me your location or manually select the timezone using the /settimezone command first."
            ).to_owned(),
            Self::IncorrectRequest => "Incorrect request".to_owned(),
            Self::ChosenTimezone(tz_name) => format!("Selected timezone: {tz_name}!"),
            Self::FailedSetTimezone(tz_name) => format!("Failed to set timezone {}", tz_name),
        }
    }
}

impl From<TgResponse> for String {
    fn from(value: TgResponse) -> Self {
        escape(&value.to_unescaped_string())
    }
}
