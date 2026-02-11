use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://businessprofileperformance.googleapis.com/v1";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyMetricTimeSeries {
    pub daily_metric: Option<String>,
    pub time_series: Option<TimeSeries>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeries {
    pub daily_values: Option<Vec<DailyValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyValue {
    pub date: Option<DateValue>,
    pub value: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateValue {
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDailyMetricsTimeSeriesResponse {
    pub time_series: Option<TimeSeries>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiDailyMetricTimeSeries {
    pub daily_metric_time_series: Option<Vec<DailyMetricTimeSeries>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchMultiDailyMetricsTimeSeriesResponse {
    pub multi_daily_metric_time_series: Option<Vec<MultiDailyMetricTimeSeries>>,
}

pub async fn get_daily_metrics_time_series(
    client: &reqwest::Client,
    access_token: &str,
    location_id: &str,
    daily_metric: &str,
    start_year: i32,
    start_month: i32,
    start_day: i32,
    end_year: i32,
    end_month: i32,
    end_day: i32,
) -> Result<GetDailyMetricsTimeSeriesResponse, reqwest::Error> {
    let url = format!(
        "{}/locations/{}:getDailyMetricsTimeSeries",
        BASE_URL, location_id
    );

    client
        .get(&url)
        .bearer_auth(access_token)
        .query(&[
            ("dailyMetric", daily_metric),
        ])
        .query(&[
            ("dailyRange.startDate.year", &start_year.to_string()),
            ("dailyRange.startDate.month", &start_month.to_string()),
            ("dailyRange.startDate.day", &start_day.to_string()),
            ("dailyRange.endDate.year", &end_year.to_string()),
            ("dailyRange.endDate.month", &end_month.to_string()),
            ("dailyRange.endDate.day", &end_day.to_string()),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<GetDailyMetricsTimeSeriesResponse>()
        .await
}

pub async fn fetch_multi_daily_metrics_time_series(
    client: &reqwest::Client,
    access_token: &str,
    location_id: &str,
    daily_metrics: &[&str],
    start_year: i32,
    start_month: i32,
    start_day: i32,
    end_year: i32,
    end_month: i32,
    end_day: i32,
) -> Result<FetchMultiDailyMetricsTimeSeriesResponse, reqwest::Error> {
    let url = format!(
        "{}/locations/{}:fetchMultiDailyMetricsTimeSeries",
        BASE_URL, location_id
    );

    let metrics_params: Vec<(&str, &str)> = daily_metrics
        .iter()
        .map(|m| ("dailyMetrics", *m))
        .collect();

    client
        .get(&url)
        .bearer_auth(access_token)
        .query(&metrics_params)
        .query(&[
            ("dailyRange.startDate.year", &start_year.to_string()),
            ("dailyRange.startDate.month", &start_month.to_string()),
            ("dailyRange.startDate.day", &start_day.to_string()),
            ("dailyRange.endDate.year", &end_year.to_string()),
            ("dailyRange.endDate.month", &end_month.to_string()),
            ("dailyRange.endDate.day", &end_day.to_string()),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<FetchMultiDailyMetricsTimeSeriesResponse>()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_get_daily_metrics_response() {
        let json = r#"{
            "timeSeries": {
                "dailyValues": [
                    {
                        "date": { "year": 2024, "month": 1, "day": 15 },
                        "value": 42
                    },
                    {
                        "date": { "year": 2024, "month": 1, "day": 16 },
                        "value": 58
                    }
                ]
            }
        }"#;

        let resp: GetDailyMetricsTimeSeriesResponse = serde_json::from_str(json).unwrap();
        let ts = resp.time_series.unwrap();
        let values = ts.daily_values.unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0].value, Some(42));
        assert_eq!(values[0].date.as_ref().unwrap().year, Some(2024));
        assert_eq!(values[1].value, Some(58));
    }

    #[test]
    fn test_deserialize_fetch_multi_response() {
        let json = r#"{
            "multiDailyMetricTimeSeries": [
                {
                    "dailyMetricTimeSeries": [
                        {
                            "dailyMetric": "WEBSITE_CLICKS",
                            "timeSeries": {
                                "dailyValues": [
                                    {
                                        "date": { "year": 2024, "month": 3, "day": 1 },
                                        "value": 100
                                    }
                                ]
                            }
                        }
                    ]
                }
            ]
        }"#;

        let resp: FetchMultiDailyMetricsTimeSeriesResponse = serde_json::from_str(json).unwrap();
        let multi = resp.multi_daily_metric_time_series.unwrap();
        assert_eq!(multi.len(), 1);
        let series = multi[0].daily_metric_time_series.as_ref().unwrap();
        assert_eq!(series[0].daily_metric.as_deref(), Some("WEBSITE_CLICKS"));
        let values = series[0].time_series.as_ref().unwrap().daily_values.as_ref().unwrap();
        assert_eq!(values[0].value, Some(100));
    }

    #[test]
    fn test_deserialize_empty_response() {
        let json = r#"{}"#;
        let resp: GetDailyMetricsTimeSeriesResponse = serde_json::from_str(json).unwrap();
        assert!(resp.time_series.is_none());
    }
}
