import { useState } from 'react';

type GameState = {
  board: (string | null)[];
  currentPlayer: 'X' | 'O';
  winner: string | null;
  isDraw: boolean;
  gameId: string | null;
};

const initialGameState: GameState = {
  board: Array(9).fill(null),
  currentPlayer: 'X',
  winner: null,
  isDraw: false,
  gameId: null,
};

export const useGameState = () => {
  const [gameState, setGameState] = useState<GameState>(initialGameState);

  const checkWinner = (board: (string | null)[]): string | null => {
    const winPatterns = [
      [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
      [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
      [0, 4, 8], [2, 4, 6], // Diagonals
    ];

    for (const pattern of winPatterns) {
      const [a, b, c] = pattern;
      if (board[a] && board[a] === board[b] && board[a] === board[c]) {
        return board[a];
      }
    }

    return null;
  };

  const checkDraw = (board: (string | null)[]): boolean => {
    return board.every(cell => cell !== null);
  };

  const makeMove = (index: number) => {
    if (gameState.board[index] || gameState.winner || gameState.isDraw) {
      return false;
    }

    const newBoard = [...gameState.board];
    newBoard[index] = gameState.currentPlayer;

    const winner = checkWinner(newBoard);
    const isDraw = !winner && checkDraw(newBoard);

    setGameState(prev => ({
      ...prev,
      board: newBoard,
      currentPlayer: prev.currentPlayer === 'X' ? 'O' : 'X',
      winner,
      isDraw,
    }));

    return true;
  };

  const resetGame = () => {
    setGameState(initialGameState);
  };

  const setGameId = (gameId: string) => {
    setGameState(prev => ({ ...prev, gameId }));
  };

  return {
    gameState,
    makeMove,
    resetGame,
    setGameId,
  };
};