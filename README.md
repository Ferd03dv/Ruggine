# Ruggine

**Ruggine** is a lightweight **client/server chat application** for text messaging and group communication.  
Users register on first launch and join groups by invitation.  
The system focuses on **performance**, **low CPU usage**, and **small executable size** across multiple platforms.

---

Features

- **User Registration**  
  New users register automatically by sending a subscription request to the server at first startup.

- **Group Chats**  
  Users can join chat groups **only through invitations**. Messages are shared exclusively within each group.

- **Cross-Platform Support**  
  Runs on at least two of the following platforms:  
  *Windows, Linux, macOS, Android, ChromeOS, iOS.*

- **Performance and Efficiency**  
  Designed for minimal CPU time consumption and compact executable size.

- **Server Logging**  
  The server automatically generates a log file **every 2 minutes**, recording details on CPU time usage.

---

Architecture

- **Client:**  
  Handles user registration, group participation, and message exchange through a simple, responsive interface.

- **Server:**  
  Manages user sessions, group membership, and message routing.  
  Periodically logs CPU performance statistics.

