import { useState, useEffect } from "react";
import AppLayout from "./components/AppLayout";

function App() {
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    // Show the app immediately, don't wait for backend
    setIsReady(true);
  }, []);

  if (!isReady) {
    return (
      <div className="h-screen w-screen flex items-center justify-center bg-dark-bg text-dark-text">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto mb-4"></div>
          <div className="text-sm text-dark-textMuted">
            Loading RedSys Desktop Agent...
          </div>
        </div>
      </div>
    );
  }

  return <AppLayout />;
}

export default App;
