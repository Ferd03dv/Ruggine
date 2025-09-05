import React, { useState, useEffect } from 'react';
import { invitationService, groupService } from '../services/api';
import './Invitations.css';

function Invitations({ user, onInvitationUpdate }) {
  const [invitations, setInvitations] = useState([]);
  const [isLoading, setIsLoading] = useState(false);
  const [groups, setGroups] = useState({});

  const loadInvitations = async () => {
    try {
      setIsLoading(true);
      const response = await invitationService.getUserInvitations(user.user_id);
      const userInvitations = response.invitations || response; // Supporta entrambi i formati
      setInvitations(userInvitations);
      
      // Carica i dettagli dei gruppi per ogni invito
      const groupDetails = {};
      for (const invitation of userInvitations) {
        if (!groupDetails[invitation.group_id]) {
          try {
            // Proviamo a ottenere il nome del gruppo (questo potrebbe non funzionare se non abbiamo l'endpoint)
            groupDetails[invitation.group_id] = `Gruppo ${invitation.group_id}`;
          } catch (error) {
            groupDetails[invitation.group_id] = `Gruppo ${invitation.group_id}`;
          }
        }
      }
      setGroups(groupDetails);
    } catch (error) {
      console.error('Errore nel caricamento degli inviti:', error);
      setInvitations([]);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadInvitations();
  }, [user.user_id]);

  const handleAcceptInvitation = async (invitationId) => {
    try {
      await invitationService.acceptInvitation(invitationId);
      loadInvitations(); // Ricarica la lista
      onInvitationUpdate(); // Notifica il componente padre per ricaricare i gruppi
      alert('Invito accettato con successo! I tuoi gruppi sono stati aggiornati.');
    } catch (error) {
      console.error('Errore nell\'accettazione dell\'invito:', error);
      alert('Errore nell\'accettazione dell\'invito');
    }
  };

  const handleRejectInvitation = async (invitationId) => {
    try {
      await invitationService.rejectInvitation(invitationId);
      loadInvitations(); // Ricarica la lista
      alert('Invito rifiutato');
    } catch (error) {
      console.error('Errore nel rifiuto dell\'invito:', error);
      alert('Errore nel rifiuto dell\'invito');
    }
  };

  const formatDate = (dateString) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('it-IT', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  if (isLoading) {
    return (
      <div className="invitations-container">
        <h3>Inviti Ricevuti</h3>
        <div className="invitations-loading">Caricamento inviti...</div>
      </div>
    );
  }

  return (
    <div className="invitations-container">
      <h3>Inviti Ricevuti ({invitations.length})</h3>
      
      {invitations.length === 0 ? (
        <div className="no-invitations">
          <p>Nessun invito in sospeso</p>
          <p>Quando qualcuno ti inviterà a un gruppo, gli inviti appariranno qui automaticamente.</p>
        </div>
      ) : (
        <div className="invitations-list">
          {invitations.map((invitation) => (
            <div key={invitation.i_id} className="invitation-item">
              <div className="invitation-info">
                <span className="invitation-group">
                  {groups[invitation.group_id] || `Gruppo ${invitation.group_id}`}
                </span>
                <span className="invitation-from">
                  Invitato da: Utente ID {invitation.invited_by}
                </span>
                <span className="invitation-date">
                  Ricevuto: {formatDate(invitation.sent_at)}
                </span>
                <span className="invitation-id">
                  ID Invito: {invitation.i_id}
                </span>
              </div>
              <div className="invitation-actions">
                <button 
                  onClick={() => handleAcceptInvitation(invitation.i_id)}
                  className="accept-btn"
                >
                  Accetta
                </button>
                <button 
                  onClick={() => handleRejectInvitation(invitation.i_id)}
                  className="reject-btn"
                >
                  Rifiuta
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export default Invitations;
