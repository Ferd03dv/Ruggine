import React, { useState, useEffect, useRef, useCallback } from 'react';
import { messageService } from '../services/api';
import './Chat.css';

function Chat({ user, onLogout }) {
  const [messages, setMessages] = useState([]);
  const [newMessage, setNewMessage] = useState('');
  const [currentGroup, setCurrentGroup] = useState(1);
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  const loadMessages = useCallback(async () => {
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
    loadMessages();
    // Aggiorna messaggi ogni 3 secondi
    const interval = setInterval(loadMessages, 3000);
    return () => clearInterval(interval);
  }, [loadMessages]);

  const sendMessage = async (e) => {
    e.preventDefault();
    if (!newMessage.trim()) return;

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

  const formatDate = (dateString) => {
    const date = new Date(dateString);
    return date.toLocaleString('it-IT', {
      hour: '2-digit',
      minute: '2-digit',
      day: '2-digit',
      month: '2-digit'
    });
  };

  return (
    <div className="chat-container">
      <div className="chat-header">
        <h3>Chat - Gruppo {currentGroup}</h3>
        <div className="user-info">
          <span>Utente: {user.username}</span>
          <button onClick={onLogout} className="logout-btn">
            Logout
          </button>
        </div>
      </div>

      <div className="group-selector">
        <label htmlFor="group-select">Seleziona Gruppo:</label>
        <select 
          id="group-select"
          value={currentGroup}
          onChange={(e) => setCurrentGroup(parseInt(e.target.value))}
        >
          <option value={1}>Gruppo 1</option>
          <option value={2}>Gruppo 2</option>
          <option value={3}>Gruppo 3</option>
        </select>
      </div>

      <div className="messages-container">
        <div className="messages-list">
          {messages.length === 0 ? (
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
                    {message.sender_id === user.user_id ? 'Tu' : `Utente ${message.sender_id}`}
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
          placeholder="Scrivi un messaggio..."
          disabled={isLoading}
        />
        <button type="submit" disabled={isLoading || !newMessage.trim()}>
          {isLoading ? 'Invio...' : 'Invia'}
        </button>
      </form>
    </div>
  );
}

export default Chat;
