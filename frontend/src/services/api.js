import axios from 'axios';

const API_BASE_URL = 'http://127.0.0.1:8000';

// Configurazione axios
const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Auth Services
export const authService = {
  async login(username, password) {
    const response = await api.post('/auth/login', {
      username,
      password,
    });
    return response.data;
  },

  async register(username, email, password) {
    const response = await api.post('/auth/register', {
      username,
      email,
      password,
    });
    return response.data;
  },
};

// Message Services
export const messageService = {
  async getMessages(groupId) {
    const response = await api.get(`/groups/${groupId}/messages`);
    return response.data;
  },

  async sendMessage(groupId, senderId, content) {
    const response = await api.post(`/groups/${groupId}/message`, {
      content,
      sender_id: senderId,
    });
    return response.data;
  },
};

export default api;
