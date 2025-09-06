import React, { useState, useEffect, useRef, useCallback } from 'react';
import { messageService, groupService, invitationService } from '../services/api';
import { useUsernames, usePreloadUsernames } from '../hooks/useUsernames';
import Invitations from './Invitations';
import GroupManagement from './GroupManagement';
import './Chat.css';

function Chat({ user, onLogout }) {
  const [messages, setMessages] = useState([]);
  const [newMessage, setNewMessage] = useState('');
  const [currentGroup, setCurrentGroup] = useState(null);
  const [userGroups, setUserGroups] = useState([]);
  const [isLoading, setIsLoading] = useState(false);
  const [showCreateGroup, setShowCreateGroup] = useState(false);
  const [showInviteUser, setShowInviteUser] = useState(false);
  const [newGroupName, setNewGroupName] = useState('');
  const [inviteUserId, setInviteUserId] = useState('');
  const messagesEndRef = useRef(null);

  // Hook per gestire i nomi utente
  const { getUsernameSync } = useUsernames();
  
  // Precarica i nomi utente dai messaggi e gruppi
  const allUserIds = [
    ...messages.map(msg => msg.sender_id),
    ...userGroups.map(group => group.created_by)
  ].filter(Boolean);
  usePreloadUsernames(allUserIds);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  const loadUserGroups = useCallback(async () => {
    try {
      const response = await groupService.getUserGroups(user.user_id);
      const groups = response.groups || [];
      setUserGroups(groups);
      if (groups.length > 0 && !currentGroup) {
        setCurrentGroup(groups[0].g_id);
      }
    } catch (error) {
      console.error('Errore nel caricamento dei gruppi:', error);
      setUserGroups([]);
    }
  }, [user.user_id, currentGroup]);

  const loadMessages = useCallback(async () => {
    if (!currentGroup) return;
    try {
      const messagesData = await messageService.getMessages(currentGroup);
      setMessages(messagesData);
    } catch (error) {
      console.error('Errore nel caricamento messaggi:', error);
    }
  }, [currentGroup]);

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  useEffect(() => {
    loadUserGroups();
  }, [loadUserGroups]);

  useEffect(() => {
    if (currentGroup) {
      loadMessages();
      // Aggiorna messaggi ogni 3 secondi
      const interval = setInterval(loadMessages, 3000);
      return () => clearInterval(interval);
    }
  }, [loadMessages, currentGroup]);

  const sendMessage = async (e) => {
    e.preventDefault();
    if (!newMessage.trim() || !currentGroup) return;

    setIsLoading(true);
    try {
      await messageService.sendMessage(currentGroup, user.user_id, newMessage);
      setNewMessage('');
      // Ricarica immediatamente i messaggi dopo l'invio
      setTimeout(loadMessages, 100);
    } catch (error) {
      console.error('Errore invio messaggio:', error);
      alert('Errore invio messaggio');
    }
    setIsLoading(false);
  };

  const createGroup = async (e) => {
    e.preventDefault();
    if (!newGroupName.trim()) return;

    try {
      await groupService.createGroup(newGroupName, user.user_id);
      setNewGroupName('');
      setShowCreateGroup(false);
      loadUserGroups();
      alert('Gruppo creato con successo!');
    } catch (error) {
      console.error('Errore creazione gruppo:', error);
      alert('Errore nella creazione del gruppo');
    }
  };

  const inviteUser = async (e) => {
    e.preventDefault();
    if (!inviteUserId.trim() || !currentGroup) return;

    try {
      await invitationService.createInvitation(user.user_id, parseInt(inviteUserId), currentGroup);
      setInviteUserId('');
      setShowInviteUser(false);
      alert(`Invito inviato con successo! L'utente riceverà una notifica dell'invito.`);
    } catch (error) {
      console.error('Errore invito utente:', error);
      console.error('Dettagli errore:', error.response?.data);
      alert(`Errore nell'invio dell'invito: ${error.response?.data || error.message}. Verifica che l'ID utente sia corretto.`);
    }
  };

  const formatDate = (dateString) => {
    const date = new Date(dateString);
    return date.toLocaleString('it-IT', {
      hour: '2-digit',
      minute: '2-digit',
      day: '2-digit',
      month: '2-digit'
    });
  };

  const getCurrentGroupName = () => {
    const group = userGroups.find(g => g.g_id === currentGroup);
    return group ? group.name : 'Nessun gruppo';
  };

  const getCurrentGroupInfo = () => {
    const group = userGroups.find(g => g.g_id === currentGroup);
    return group;
  };

  return (
    <div className="chat-container">
      {/* Header */}
      <div className="chat-header">
        <h3>Ruggine - {getCurrentGroupName()}</h3>
        <div className="user-info">
          <span>Utente: {user.username} (ID: {user.user_id})</span>
          <button onClick={onLogout} className="logout-btn">
            Logout
          </button>
        </div>
      </div>

      {/* Main Content - Two Columns */}
      <div className="main-content">
        {/* Left Column - Groups and Invitations Management */}
        <div className="left-column">
          <div className="group-management-section">
            <h4>Gestione Gruppi</h4>
            <div className="group-controls">
              <label htmlFor="group-select">Seleziona Gruppo:</label>
              <select 
                id="group-select"
                value={currentGroup || ''}
                onChange={(e) => setCurrentGroup(parseInt(e.target.value))}
              >
                <option value="">Seleziona un gruppo</option>
                {userGroups.map((group) => (
                  <option key={group.g_id} value={group.g_id}>
                    {group.name}
                  </option>
                ))}
              </select>
              
              <button 
                onClick={() => setShowCreateGroup(true)}
                className="action-btn"
              >
                Crea Gruppo
              </button>
              
              {currentGroup && (
                <button 
                  onClick={() => setShowInviteUser(true)}
                  className="action-btn"
                >
                  Invita Utente
                </button>
              )}
            </div>
            
            {currentGroup && getCurrentGroupInfo() && (
              <div className="group-info">
                <h5>Gruppo Attivo</h5>
                <div className="group-details">
                  <span><strong>Nome:</strong> {getCurrentGroupInfo().name}</span>
                </div>
              </div>
            )}
          </div>

          <Invitations 
            user={user} 
            onInvitationUpdate={loadUserGroups}
          />

          <GroupManagement 
            user={user}
            userGroups={userGroups}
            onGroupUpdate={loadUserGroups}
          />
        </div>

        {/* Right Column - Chat */}
        <div className="right-column">
          <div className="chat-section">
            <div className="messages-container">
              <div className="messages-list">
                {!currentGroup ? (
                  <div className="no-messages">
                    {userGroups.length === 0 ? 
                      'Nessun gruppo disponibile. Crea il tuo primo gruppo!' :
                      'Seleziona un gruppo per visualizzare i messaggi'
                    }
                  </div>
                ) : messages.length === 0 ? (
                  <div className="no-messages">
                    Nessun messaggio in questo gruppo. Inizia la conversazione!
                  </div>
                ) : (
                  messages.map((message) => (
                    <div 
                      key={message.m_id} 
                      className={`message-item ${message.sender_id === user.user_id ? 'own' : 'other'}`}
                    >
                      <div className="message-header">
                        <span className="sender">
                          {message.sender_id === user.user_id ? 'Tu' : getUsernameSync(message.sender_id)}
                        </span>
                        <span className="timestamp">
                          {formatDate(message.sent_at)}
                        </span>
                      </div>
                      <div className="message-content">
                        {message.content}
                      </div>
                    </div>
                  ))
                )}
                <div ref={messagesEndRef} />
              </div>
            </div>

            <form onSubmit={sendMessage} className="message-form">
              <input
                type="text"
                value={newMessage}
                onChange={(e) => setNewMessage(e.target.value)}
                placeholder={currentGroup ? "Scrivi un messaggio..." : "Seleziona un gruppo per inviare messaggi"}
                disabled={isLoading || !currentGroup}
              />
              <button type="submit" disabled={isLoading || !newMessage.trim() || !currentGroup}>
                {isLoading ? 'Invio...' : 'Invia'}
              </button>
            </form>
          </div>
        </div>
      </div>

      {/* Modal Crea Gruppo */}
      {showCreateGroup && (
        <div className="modal-overlay">
          <div className="modal">
            <h3>Crea Nuovo Gruppo</h3>
            <form onSubmit={createGroup}>
              <input
                type="text"
                value={newGroupName}
                onChange={(e) => setNewGroupName(e.target.value)}
                placeholder="Nome del gruppo"
                required
              />
              <div className="modal-buttons">
                <button type="submit">Crea</button>
                <button type="button" onClick={() => setShowCreateGroup(false)}>
                  Annulla
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Modal Invita Utente */}
      {showInviteUser && (
        <div className="modal-overlay">
          <div className="modal">
            <h3>Invita Utente al Gruppo</h3>
            <div className="invite-info">
              <p>Stai invitando un utente al gruppo: <strong>{getCurrentGroupName()}</strong></p>
              <p>L'utente riceverà un ID invito che potrà utilizzare per accettare o rifiutare l'invito.</p>
            </div>
            <form onSubmit={inviteUser}>
              <div className="form-group">
                <label htmlFor="user-id">ID Utente da invitare:</label>
                <input
                  id="user-id"
                  type="number"
                  value={inviteUserId}
                  onChange={(e) => setInviteUserId(e.target.value)}
                  placeholder="Es: 123"
                  required
                  min="1"
                />
                <small>Inserisci l'ID numerico dell'utente che vuoi invitare</small>
              </div>
              <div className="modal-buttons">
                <button type="submit" className="primary-btn">Invia Invito</button>
                <button type="button" onClick={() => setShowInviteUser(false)} className="secondary-btn">
                  Annulla
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

export default Chat;
