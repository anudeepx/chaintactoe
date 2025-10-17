import { Button } from "@/components/ui/button";
import { RotateCcw, Users } from "lucide-react";

interface GameStatusProps {
  currentPlayer: string;
  winner: string | null;
  isDraw: boolean;
  onNewGame: () => void;
  onJoinGame: () => void;
}

const GameStatus = ({ currentPlayer, winner, isDraw, onNewGame, onJoinGame }: GameStatusProps) => {
  const getStatusText = () => {
    if (winner) {
      return (
        <span className="text-2xl font-bold">
          Winner: {" "}
          <span className={winner === 'X' ? 'text-game-x' : 'text-game-o'}>
            {winner}
          </span>
        </span>
      );
    }
    
    if (isDraw) {
      return <span className="text-2xl font-bold text-muted-foreground">It's a Draw!</span>;
    }
    
    return (
      <span className="text-3xl">
        Player{" "}
        <span className={currentPlayer === 'X' ? 'text-game-x font-bold' : 'text-game-o font-bold'}>
          {currentPlayer}
        </span>
        's turn
      </span>
    );
  };

  return (
    <div className="flex flex-col items-center gap-6">
      <div className="text-center">
        {getStatusText()}
      </div>
      
      <div className="flex flex-col sm:flex-row gap-3 w-full max-w-sm">
        <Button 
          onClick={onNewGame}
          variant="default"
          className="flex items-center gap-2 shadow-glow font-montserrat"
        >
          <RotateCcw className="w-4 h-4" />
          New Game
        </Button>
        
        <Button 
          onClick={onJoinGame}
          variant="secondary"
          className="flex items-center gap-2 font-montserrat"
        >
          <Users className="w-4 h-4" />
          Join Game
        </Button>
      </div>
    </div>
  );
};

export default GameStatus;