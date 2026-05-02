import { useStore } from '../store/useStore';
import { useCallback } from 'react';

export const PreviewModal = () => {
  const { selectedPhoto, setSelectedPhoto, axumPort } = useStore();

  const getMediaUrl = useCallback((path: string) => {
    if (!axumPort) return '';
    return `http://127.0.0.1:${axumPort}/media?path=${encodeURIComponent(path)}`;
  }, [axumPort]);

  if (!selectedPhoto) return null;

  return (
    <div 
      className="fixed inset-0 bg-black/90 z-50 flex items-center justify-center p-4 sm:p-8"
      onClick={() => setSelectedPhoto(null)}
    >
      <div 
        className="relative max-w-full max-h-full flex flex-col items-center justify-center"
        onClick={(e) => e.stopPropagation()}
      >
        <button
          className="absolute -top-10 right-0 text-white hover:text-gray-300 p-2"
          onClick={() => setSelectedPhoto(null)}
        >
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
        <img
          src={getMediaUrl(selectedPhoto.path)}
          alt="preview"
          className="max-w-full max-h-[85vh] object-contain rounded-sm"
        />
        <div className="mt-4 text-gray-300 text-sm bg-gray-900/50 px-4 py-2 rounded-md">
          {selectedPhoto.path.split(/[/\\]/).pop()}
        </div>
      </div>
    </div>
  );
};
