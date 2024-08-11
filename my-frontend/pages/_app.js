import '../styles/globals.css';
import axios from 'axios';
import Cookies from 'js-cookie';

function MyApp({ Component, pageProps }) {
  axios.interceptors.request.use((config) => {
    const token = Cookies.get('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  });

  return <Component {...pageProps} />;
}

export default MyApp;
