import { useEffect, useState } from 'react';
import axios from 'axios';
import Cookies from 'js-cookie';
import { useRouter } from 'next/router';
import { Pie } from 'react-chartjs-2';
import 'chart.js/auto';

export default function Courses() {
  const [courses, setCourses] = useState([]);
  const [totalSpent, setTotalSpent] = useState(0);
  const router = useRouter();

  useEffect(() => {
    const fetchCourses = async () => {
      const token = Cookies.get('token');
      if (!token) {
        router.push('/');
        return;
      }

      try {
        const response = await axios.get('http://localhost:8080/api/courses', {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        setCourses(response.data);

        // Calculer le total dépensé
        const total = response.data.reduce((acc, course) => acc + course.amount, 0);
        setTotalSpent(total);
      } catch (error) {
        alert('Failed to fetch courses');
        router.push('/');
      }
    };

    fetchCourses();
  }, []);

  // Données pour le graphique en camembert
  const data = {
    labels: ['Spent', 'Remaining'],
    datasets: [
      {
        data: [totalSpent, 10000 - totalSpent],
        backgroundColor: ['#FF6384', '#36A2EB'],
        hoverBackgroundColor: ['#FF6384', '#36A2EB'],
      },
    ],
  };

  return (
    <div>
      <h1>Courses</h1>
      <table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Catégorie</th>
            <th>Prix</th>
            <th>Date</th>
          </tr>
        </thead>
        <tbody>
          {courses.map((course) => (
            <tr key={course.id}>
              <td>{course.name}</td>
              <td>{course.category}</td>
              <td>{course.amount}</td>
              <td>{course.date}</td>
            </tr>
          ))}
        </tbody>
      </table>

      <h2>Spending Overview</h2>
      <div style={{ width: '50%', margin: '0 auto' }}> 
      <Pie data={data} />
      </div>
      </div>
  );
}
