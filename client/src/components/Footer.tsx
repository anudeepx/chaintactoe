import { Zap } from "lucide-react";

const Footer = () => {
  return (
    <footer className="w-full px-4 py-6 text-center border-none">
      <div className="max-w-5xl mx-auto flex flex-col md:flex-row items-center justify-between gap-4">
        <p className="text-sm text-muted-foreground flex items-center gap-1">
          Built on Solana
          <Zap className="w-4 h-4 text-yellow-400" />
        </p>

        <div className="flex items-center gap-6">
          <a
            href="https://x.com/0x4nud33p"
            className="text-white/70 hover:text-white transition-colors text-sm"
          >
            Twitter
          </a>
          <a
            href="https://www.linkedin.com/in/0x4nud33p/"
            className="text-white/70 hover:text-white transition-colors text-sm"
          >
            LinkedIn
          </a>
          <a
            href="https://github.com/0x4nud33p/chaintactoe"
            className="text-white/70 hover:text-white transition-colors text-sm"
          >
            GitHub
          </a>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
