import { useState, useEffect, useCallback } from 'react';
import { userService } from '../services/api';

// Cache globale per i nomi utente
const usernameCache = new Map();

export const useUsernames = () => {
  const [usernames, setUsernames] = useState(new Map(usernameCache));

  const getUsernameById = useCallback(async (userId) => {
    if (usernames.has(userId)) {
      return usernames.get(userId);
    }

    try {
      const response = await userService.getUserById(userId);
      if (response.success && response.user) {
        const username = response.user.username;
        // Aggiorna cache locale e globale
        setUsernames(prev => {
          const newMap = new Map(prev);
          newMap.set(userId, username);
          usernameCache.set(userId, username);
          return newMap;
        });
        return username;
      }
    } catch (error) {
      console.error(`Errore nel recupero utente ${userId}:`, error);
    }
    
    return `Utente ${userId}`; // Fallback
  }, [usernames]);

  const getUsernameSync = useCallback((userId) => {
    return usernames.get(userId) || `Utente ${userId}`;
  }, [usernames]);

  return { getUsernameById, getUsernameSync };
};

// Hook per precaricare usernames da una lista di ID
export const usePreloadUsernames = (userIds) => {
  const { getUsernameById } = useUsernames();

  useEffect(() => {
    const preloadUsernames = async () => {
      const uniqueIds = [...new Set(userIds.filter(id => id && !usernameCache.has(id)))];
      await Promise.all(uniqueIds.map(id => getUsernameById(id)));
    };

    if (userIds.length > 0) {
      preloadUsernames();
    }
  }, [userIds, getUsernameById]);
};
