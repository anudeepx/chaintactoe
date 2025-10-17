import { useWallet } from "@/hooks/useWallet";
import { useGameState } from "@/hooks/useGameState";
import { useGameActions } from "@/hooks/useGameActions";
import Navbar from "@/components/Navbar";
import GameBoard from "@/components/GameBoard";
import GameStatus from "@/components/GameStatus";
import Footer from "@/components/Footer";

const Index = () => {
  const { isConnected, connect } = useWallet();
  const { gameState, makeMove, resetGame } = useGameState();
  const { createGame, joinGame, makeMove: makeOnChainMove } = useGameActions();

  const handleCellClick = async (index: number) => {
    const success = makeMove(index);
    if (success && gameState.gameId) {
      // Make the move on-chain if game is connected
      await makeOnChainMove(gameState.gameId, index);
    }
  };

  const handleNewGame = async () => {
    resetGame();
    if (isConnected) {
      const gameId = await createGame();
      // setGameId would be called here when implemented
    }
  };

  const handleJoinGame = async () => {
    if (isConnected) {
      // This would open a dialog to enter game ID
      await joinGame('mock-game-id');
    } else {
      // Prompt to connect wallet first
      await connect();
    }
  };

  return (
    <div className="min-h-screen bg-gradient-bg flex flex-col">
      <Navbar isConnected={isConnected} onConnectWallet={connect} />
      
      <main className="flex-1 flex flex-col items-center justify-center px-4 py-8 gap-8">
        <GameBoard
          board={gameState.board}
          onCellClick={handleCellClick}
          currentPlayer={gameState.currentPlayer}
          winner={gameState.winner}
          isDraw={gameState.isDraw}
        />
        
        <GameStatus
          currentPlayer={gameState.currentPlayer}
          winner={gameState.winner}
          isDraw={gameState.isDraw}
          onNewGame={handleNewGame}
          onJoinGame={handleJoinGame}
        />
      </main>
      
      <Footer />
    </div>
  );
};

export default Index;
