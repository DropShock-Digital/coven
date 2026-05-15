---
title: "Gateway API"
description: "Core Gateway REST and WebSocket API"
---

# Gateway API

The Gateway API is the main interface to Coven's central service.

## Base URL

```
http://localhost:8080
ws://localhost:8080/ws
```

## Authentication

All requests require authentication via:
- Bearer token
- API key header
- Session cookie

## Core Endpoints

### Sessions

- `GET /api/sessions` - List sessions
- `POST /api/sessions` - Create session
- `GET /api/sessions/:id` - Get session
- `PUT /api/sessions/:id` - Update session
- `DELETE /api/sessions/:id` - Delete session

### Messages

- `POST /api/sessions/:id/messages` - Send message
- `GET /api/sessions/:id/messages` - Get message history
- `GET /api/sessions/:id/messages/:msgId` - Get message

### Agents

- `GET /api/agents` - List agents
- `GET /api/agents/:name` - Get agent
- `POST /api/agents` - Create agent

## WebSocket

Real-time communication via WebSocket at `/ws`:

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Agent response:', message.content);
};

ws.send(JSON.stringify({
  type: 'message',
  session_id: 'sess_123',
  content: 'Hello'
}));
```

[See more →](/core/api/sessions)
