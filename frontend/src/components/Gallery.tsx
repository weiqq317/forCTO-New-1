import { useEffect, useRef, useCallback } from 'react';
import { useStore } from '../store/useStore';
import type { Photo } from '../types';

export const Gallery = () => {
  const { photos, loading, hasMore, fetchPhotos, axumPort, setSelectedPhoto } = useStore();
  const observerTarget = useRef<HTMLDivElement>(null);

  const getMediaUrl = useCallback((path: string) => {
    if (!axumPort) return '';
    return `http://127.0.0.1:${axumPort}/media?path=${encodeURIComponent(path)}`;
  }, [axumPort]);

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && hasMore && !loading) {
          fetchPhotos();
        }
      },
      { threshold: 1.0 }
    );

    if (observerTarget.current) {
      observer.observe(observerTarget.current);
    }

    return () => observer.disconnect();
  }, [hasMore, loading, fetchPhotos]);

  if (!axumPort) return null;

  return (
    <div className="flex-1 overflow-y-auto p-4 bg-gray-950">
      <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8 gap-4">
        {photos.map((photo: Photo) => (
          <div 
            key={photo.id} 
            className="aspect-square bg-gray-800 rounded-lg overflow-hidden cursor-pointer hover:ring-2 hover:ring-blue-500 transition-all"
            onClick={() => setSelectedPhoto(photo)}
          >
            <img
              src={getMediaUrl(photo.path)}
              alt="thumbnail"
              loading="lazy"
              className="w-full h-full object-cover"
            />
          </div>
        ))}
      </div>
      
      <div ref={observerTarget} className="h-20 flex items-center justify-center mt-4">
        {loading && <div className="text-gray-400">Loading more...</div>}
        {!hasMore && photos.length > 0 && <div className="text-gray-600">No more photos</div>}
        {!loading && photos.length === 0 && <div className="text-gray-400">No photos found</div>}
      </div>
    </div>
  );
};
