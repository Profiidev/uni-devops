import { goto } from '$app/navigation';
import type { Command } from './command.svelte';

let socket: WebSocket | null = null;

export function connectWebSocket(
  id: string,
  onMessage: (data: any) => void
): void {
  if (socket) {
    console.warn('WebSocket already connected.');
    return;
  }

  console.log(`Connecting to WebSocket for game ID: ${id}`);
  socket = new WebSocket(`/backend/game/ws/${id}`);

  socket.onopen = () => {
    console.log('WebSocket connection opened');
  };

  socket.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data);
      onMessage(data);
    } catch (err) {
      console.error('Failed to parse WebSocket message', err);
    }
  };

  socket.onclose = () => {
    console.log('WebSocket connection closed');
    socket = null;
  };

  socket.onerror = (error) => {
    console.error('WebSocket error:', error);
    goto('/');
  };
}

export function sendWebSocketMessage(data: Command): void {
  if (socket && socket.readyState === WebSocket.OPEN) {
    socket.send(JSON.stringify(data));
  } else {
    console.warn('WebSocket is not open');
  }
}

export function closeWebSocket(): void {
  if (socket) {
    socket.close();
    socket = null;
  }
}
