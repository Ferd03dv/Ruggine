import React, { useState } from 'react';
import { authService } from '../services/api';
import './Login.css';

function Login({ onLogin }) {
  const [isLoginMode, setIsLoginMode] = useState(true);
  const [formData, setFormData] = useState({
    username: '',
    email: '',
    password: ''
  });
  const [message, setMessage] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const handleInputChange = (e) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value
    });
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsLoading(true);
    setMessage('');

    try {
      if (isLoginMode) {
        // Login
        const response = await authService.login(formData.username, formData.password);

        if (response.success) {
          setMessage('Login effettuato con successo!');
          onLogin({
            user_id: response.user_id,
            username: formData.username
          });
        }
      } else {
        // Register
        const response = await authService.register(formData.username, formData.email, formData.password);

        if (response.success) {
          setMessage('Registrazione completata! Effettua il login.');
          setIsLoginMode(true);
          setFormData({ username: '', email: '', password: '' });
        }
      }
    } catch (error) {
      if (error.response && error.response.data) {
        setMessage(error.response.data.message || 'Errore durante l\'operazione');
      } else {
        setMessage('Errore di connessione al server');
      }
    }

    setIsLoading(false);
  };

  return (
    <div className="login-container">
      <div className="login-form">
        <h2>Ruggine Chat</h2>
        
        <div className="tab-buttons">
          <button 
            className={`tab-button ${isLoginMode ? 'active' : ''}`}
            onClick={() => {
              setIsLoginMode(true);
              setMessage('');
              setFormData({ username: '', email: '', password: '' });
            }}
          >
            Login
          </button>
          <button 
            className={`tab-button ${!isLoginMode ? 'active' : ''}`}
            onClick={() => {
              setIsLoginMode(false);
              setMessage('');
              setFormData({ username: '', email: '', password: '' });
            }}
          >
            Registrati
          </button>
        </div>

        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label htmlFor="username">Username:</label>
            <input
              type="text"
              id="username"
              name="username"
              value={formData.username}
              onChange={handleInputChange}
              required
            />
          </div>

          {!isLoginMode && (
            <div className="form-group">
              <label htmlFor="email">Email:</label>
              <input
                type="email"
                id="email"
                name="email"
                value={formData.email}
                onChange={handleInputChange}
                required
              />
            </div>
          )}

          <div className="form-group">
            <label htmlFor="password">Password:</label>
            <input
              type="password"
              id="password"
              name="password"
              value={formData.password}
              onChange={handleInputChange}
              required
            />
          </div>

          <button type="submit" disabled={isLoading}>
            {isLoading ? 'Caricamento...' : (isLoginMode ? 'Accedi' : 'Registrati')}
          </button>
        </form>

        {message && (
          <div className={`message ${message.includes('successo') || message.includes('completata') ? 'success' : 'error'}`}>
            {message}
          </div>
        )}
      </div>
    </div>
  );
}

export default Login;
