use serde::Deserialize;

#[derive(Deserialize)]
pub struct AnalyticsResponse {
    /// Groups of member and/or custom front ids based on timings when they fronted
    pub timings: AnalyticsTimings,
    /// Values for the timeframe
    pub values: AnalyticsValues,
}

#[derive(Deserialize)]
pub struct AnalyticsTimings {
    #[serde(rename = "morningFronters")]
    pub morning_fronters: Vec<AnalyticsValue>,
    #[serde(rename = "dayFronters")]
    pub day_fronters: Vec<AnalyticsValue>,
    #[serde(rename = "eveningFronters")]
    pub evening_fronters: Vec<AnalyticsValue>,
    #[serde(rename = "nightFronters")]
    pub night_fronters: Vec<AnalyticsValue>,
}

#[derive(Deserialize)]
pub struct AnalyticsValues {
    /// Total length they fronted
    pub sums: Vec<AnalyticsValue>,
    /// Average length they fronted
    pub averages: Vec<AnalyticsValue>,
    /// Maximum length they fronted
    pub maxes: Vec<AnalyticsValue>,
    /// Minimum length they fronted
    pub mins: Vec<AnalyticsValue>,
    /// Amount of times they fronted
    pub nums: Vec<AnalyticsValue>,
}

#[derive(Deserialize)]
pub struct AnalyticsValue {
    pub id: String,
    pub value: u64
}