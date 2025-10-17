import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

interface GameBoardProps {
  board: (string | null)[];
  onCellClick: (index: number) => void;
  currentPlayer: string;
  winner: string | null;
  isDraw: boolean;
}

const GameBoard = ({ board, onCellClick, currentPlayer, winner, isDraw }: GameBoardProps) => {
  const getCellContent = (value: string | null) => {
    if (value === 'X') return 'X';
    if (value === 'O') return 'O';
    return '';
  };

  const getCellStyles = (value: string | null, index: number) => {
    const baseStyles = "w-20 h-20 sm:w-24 sm:h-24 md:w-28 md:h-28 rounded-lg transition-all font-montserrat duration-200 text-2xl sm:text-3xl font-bold shadow-cell";
    
    if (value === 'X') {
      return cn(baseStyles, "bg-game-cell text-game-x shadow-glow border-2 border-game-x/30");
    }
    
    if (value === 'O') {
      return cn(baseStyles, "bg-game-cell text-game-o shadow-glow border-2 border-game-o/30");
    }
    
    // Empty cell
    const isDisabled = winner || isDraw;
    return cn(
      baseStyles,
      "bg-game-cell hover:bg-game-cell-hover border-2 border-transparent",
      isDisabled ? "cursor-not-allowed opacity-50" : "cursor-pointer hover:shadow-glow hover:border-primary/20"
    );
  };

  return (
    <div className="grid grid-cols-3 gap-3 sm:gap-4 p-6 bg-card/30 rounded-xl backdrop-blur-sm shadow-elevated">
      {board.map((cell, index) => (
        <Button
          key={index}
          variant="ghost"
          className={getCellStyles(cell, index)}
          onClick={() => onCellClick(index)}
          disabled={cell !== null || winner !== null || isDraw}
          aria-label={`Cell ${index + 1}, ${cell || 'empty'}`}
        >
          {getCellContent(cell)}
        </Button>
      ))}
    </div>
  );
};

export default GameBoard;