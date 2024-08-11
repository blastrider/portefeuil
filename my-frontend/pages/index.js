import { useState, useEffect } from 'react';
import axios from 'axios';
import Cookies from 'js-cookie';
import { useRouter } from 'next/router';

export default function Home() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const router = useRouter();

  // Vérifier si un token est déjà présent au chargement de la page
  useEffect(() => {
    const checkToken = () => {
      if (typeof window !== 'undefined') {  // Vérification si le code est exécuté côté client
        const token = Cookies.get('token');

        if (token) {
          router.push('/courses');  // Rediriger vers les courses si déjà connecté
        }
      }
    };

    checkToken();
  }, []);

  const handleLogin = async (e) => {
    e.preventDefault();
    try {
      const response = await axios.post('http://localhost:8080/login', {
        email,
        password,
      });

      if (response.data) {
        Cookies.set('token', response.data, { expires: 1, path: '/' });

        // Ajouter un délai pour laisser le cookie se définir correctement avant de rediriger
        setTimeout(() => {
          router.push('/courses');
        }, 500);
      } else {
        alert('Login failed');
      }
    } catch (error) {
      alert('Login failed');
    }
  };

  return (
    <div>
      <h1>Login</h1>
      <form onSubmit={handleLogin}>
        <div>
          <label>Email</label>
          <input
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
          />
        </div>
        <div>
          <label>Password</label>
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
          />
        </div>
        <button type="submit">Login</button>
      </form>
    </div>
  );
}
