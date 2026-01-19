use serde::Serialize;

#[derive(Serialize)]
pub struct WeatherData {
    pub location: String,
    pub temperature: f32,
    pub condition: String,
}

#[lithe::rpc]
pub async fn get_forecast(city: String) -> WeatherData {
    // In a real app, this would call an external API
    WeatherData {
        location: city,
        temperature: 22.5,
        condition: "Sunny".to_string(),
    }
}
