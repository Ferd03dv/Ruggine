import React, { useState } from 'react';
import './GroupManagement.css';

function GroupManagement({ user, userGroups, onGroupUpdate }) {
  const [showGroupList, setShowGroupList] = useState(false);

  const getGroupsByRole = () => {
    const myGroups = userGroups.filter(group => group.created_by === user.user_id);
    const memberGroups = userGroups.filter(group => group.created_by !== user.user_id);
    
    return { myGroups, memberGroups };
  };

  const { myGroups, memberGroups } = getGroupsByRole();

  return (
    <div className="group-management">
      <button 
        className="toggle-groups-btn"
        onClick={() => setShowGroupList(!showGroupList)}
      >
        {showGroupList ? 'Nascondi Gruppi' : 'Mostra I Miei Gruppi'} ({userGroups.length})
      </button>

      {showGroupList && (
        <div className="groups-list">
          {myGroups.length > 0 && (
            <div className="group-section">
              <h4>Gruppi che ho creato ({myGroups.length})</h4>
              <div className="groups-grid">
                {myGroups.map(group => (
                  <div key={group.g_id} className="group-card admin">
                    <div className="group-header">
                      <span className="group-name">{group.name}</span>
                      <span className="group-role">Amministratore</span>
                    </div>
                    <div className="group-info">
                      <span>ID: {group.g_id}</span>
                      <span>Creato: {new Date(group.created_at).toLocaleDateString('it-IT')}</span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {memberGroups.length > 0 && (
            <div className="group-section">
              <h4>Gruppi di cui sono membro ({memberGroups.length})</h4>
              <div className="groups-grid">
                {memberGroups.map(group => (
                  <div key={group.g_id} className="group-card member">
                    <div className="group-header">
                      <span className="group-name">{group.name}</span>
                      <span className="group-role">Membro</span>
                    </div>
                    <div className="group-info">
                      <span>ID: {group.g_id}</span>
                      <span>Creato da: Utente {group.created_by}</span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {userGroups.length === 0 && (
            <div className="no-groups">
              <p>Non fai ancora parte di nessun gruppo.</p>
              <p>Crea un nuovo gruppo o chiedi a qualcuno di invitarti!</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

export default GroupManagement;
