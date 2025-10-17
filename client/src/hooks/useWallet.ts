import { useState } from 'react';

export const useWallet = () => {
  const [isConnected, setIsConnected] = useState(false);
  const [publicKey, setPublicKey] = useState<string | null>(null);

  const connect = async () => {
    try {
      // Stub for Phantom/Solflare wallet connection
      console.log('Connecting to wallet...');
      
      // Simulate connection delay
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // Mock wallet connection
      setIsConnected(true);
      setPublicKey('mock-public-key-123');
      
      console.log('Wallet connected successfully');
    } catch (error) {
      console.error('Failed to connect wallet:', error);
    }
  };

  const disconnect = () => {
    setIsConnected(false);
    setPublicKey(null);
    console.log('Wallet disconnected');
  };

  return {
    isConnected,
    publicKey,
    connect,
    disconnect,
  };
};