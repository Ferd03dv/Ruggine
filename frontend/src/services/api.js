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

// Group Services
export const groupService = {
  async getUserGroups(userId) {
    const response = await api.get(`/groups/user/${userId}`);
    return response.data;
  },

  async createGroup(name, createdBy) {
    const response = await api.post('/groups/create', {
      name,
      created_by: createdBy,
    });
    return response.data;
  },
};

// Invitation Services
export const invitationService = {
  async createInvitation(invitedBy, invitedUser, groupId) {
    const now = new Date();
    const sentAt = now.toISOString().slice(0, 19); // Format: YYYY-MM-DDTHH:MM:SS

    const response = await api.post('/invitations/invitation/create', {
      i_id: null,
      status: 0, // pending
      sent_at: sentAt,
      invited_by: invitedBy,
      invited_user: invitedUser,
      group_id: groupId,
    });
    return response.data;
  },

  async getUserInvitations(userId) {
    const response = await api.get(`/invitations/user/${userId}`);
    return response.data;
  },

  async acceptInvitation(invitationId) {
    const response = await api.post(`/invitations/invitation/${invitationId}/accept`);
    return response.data;
  },

  async rejectInvitation(invitationId) {
    const response = await api.post(`/invitations/invitation/${invitationId}/reject`);
    return response.data;
  },
};

// Message Services
export const messageService = {
  async getMessages(groupId) {
    const response = await api.get(`/message/groups/${groupId}/messages`);
    return response.data;
  },

  async sendMessage(groupId, senderId, content) {
    const response = await api.post(`/message/groups/${groupId}/message`, {
      content,
      sender_id: senderId,
    });
    return response.data;
  },
};

export default api;
