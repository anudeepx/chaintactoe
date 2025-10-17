import { useToast } from "@/hooks/use-toast";

export const useGameActions = () => {
  const { toast } = useToast();

  const createGame = async () => {
    try {
      console.log('Creating new game...');
      
      // Stub for Anchor program integration
      // This would call the Solana program to create a new game
      
      toast({
        title: "Game Created",
        description: "New TicTacToe game created on-chain",
      });
      
      return 'mock-game-id-123';
    } catch (error) {
      console.error('Failed to create game:', error);
      toast({
        title: "Error",
        description: "Failed to create game",
        variant: "destructive",
      });
    }
  };

  const joinGame = async (gameId: string) => {
    try {
      console.log('Joining game:', gameId);
      
      // Stub for Anchor program integration
      // This would call the Solana program to join an existing game
      
      toast({
        title: "Game Joined",
        description: `Joined game ${gameId}`,
      });
    } catch (error) {
      console.error('Failed to join game:', error);
      toast({
        title: "Error",
        description: "Failed to join game",
        variant: "destructive",
      });
    }
  };

  const makeMove = async (gameId: string, position: number) => {
    try {
      console.log('Making move:', { gameId, position });
      
      // Stub for Anchor program integration
      // This would call the Solana program to make a move
      
      return true;
    } catch (error) {
      console.error('Failed to make move:', error);
      toast({
        title: "Error",
        description: "Failed to make move",
        variant: "destructive",
      });
      return false;
    }
  };

  return {
    createGame,
    joinGame,
    makeMove,
  };
};