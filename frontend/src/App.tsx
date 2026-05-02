import { useEffect } from 'react';
import { SearchBar } from './components/SearchBar';
import { Gallery } from './components/Gallery';
import { PreviewModal } from './components/PreviewModal';
import { useStore } from './store/useStore';

function App() {
  const { initApp } = useStore();

  useEffect(() => {
    initApp();
  }, [initApp]);

  return (
    <div className="h-screen flex flex-col bg-gray-950 text-white overflow-hidden">
      <SearchBar />
      <Gallery />
      <PreviewModal />
    </div>
  );
}

export default App;
