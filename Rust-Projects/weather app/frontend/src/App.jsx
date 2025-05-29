import { useState } from 'react';
import SearchIcon from '@mui/icons-material/Search';
import axios from 'axios';

function App() {
  const [location, setLocation] = useState('');
  const [weatherData, setWeatherData] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const fetchWeather = async () => {
    if (!location) return;
    
    setLoading(true);
    setError('');
    
    try {
      const response = await axios.get(`/api/current/${location}`);
      if (response.data.Ok) {
        setWeatherData(response.data.Ok);
      } else {
        setError('Failed to fetch weather data. Please try again.');
      }
    } catch (err) {
      setError(err.response?.data?.message || 'Failed to fetch weather data. Please try again.');
      console.error('Error fetching weather:', err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container mx-auto px-4 py-8 max-w-4xl">
      <div className="text-center mb-8">
        <h1 className="text-4xl font-bold mb-6">Weather App</h1>
        <div className="flex justify-center gap-2">
          <input
            type="text"
            placeholder="Enter location"
            value={location}
            onChange={(e) => setLocation(e.target.value)}
            onKeyPress={(e) => e.key === 'Enter' && fetchWeather()}
            className="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 w-64"
          />
          <button
            onClick={fetchWeather}
            disabled={loading}
            className="bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 flex items-center gap-2"
          >
            <SearchIcon />
            Search
          </button>
        </div>
      </div>

      {loading && (
        <div className="flex justify-center my-8">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
        </div>
      )}

      {error && (
        <div className="text-red-500 text-center mb-4">
          {error}
        </div>
      )}

      {weatherData && !loading && (
        <div className="space-y-6">
          {/* Current Weather */}
          <div className="bg-white rounded-lg shadow-lg p-6">
            <h2 className="text-2xl font-semibold mb-4">Current Weather</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <p className="text-xl font-medium">
                  Temperature: {weatherData.current.current.temp_c}°C
                </p>
                <p className="text-gray-600">
                  Wind: {weatherData.current.current.wind_kph} km/h ({weatherData.current.current.wind_dir})
                </p>
                <p className="text-gray-600">
                  Precipitation: {weatherData.current.current.precip_mm} mm
                </p>
              </div>
            </div>
          </div>

          {/* Hourly Forecast */}
          <div className="bg-white rounded-lg shadow-lg p-6">
            <h2 className="text-2xl font-semibold mb-4">Hourly Forecast</h2>
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-4">
              {weatherData.hourly.forecast.forecastday[0].hour.map((hour, index) => (
                <div key={index} className="bg-gray-50 rounded-lg p-4">
                  <p className="text-sm font-medium">
                    {new Date(hour.time).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                  </p>
                  <p className="text-lg font-semibold">
                    {hour.temp_c}°C
                  </p>
                  <p className="text-sm text-gray-600">
                    {hour.condition.text}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App; 