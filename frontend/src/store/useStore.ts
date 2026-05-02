import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/tauri';
import type { Photo } from '../types';

interface AppState {
  photos: Photo[];
  searchQuery: string;
  loading: boolean;
  hasMore: boolean;
  offset: number;
  limit: number;
  axumPort: number | null;
  selectedPhoto: Photo | null;
  setSearchQuery: (query: string) => void;
  setSelectedPhoto: (photo: Photo | null) => void;
  fetchPhotos: (reset?: boolean) => Promise<void>;
  importFolder: () => Promise<void>;
  initApp: () => Promise<void>;
}

export const useStore = create<AppState>((set, get) => ({
  photos: [],
  searchQuery: '',
  loading: false,
  hasMore: true,
  offset: 0,
  limit: 50,
  axumPort: null,
  selectedPhoto: null,

  initApp: async () => {
    try {
      const port = await invoke<number>('get_axum_port');
      set({ axumPort: port });
      get().fetchPhotos(true);
    } catch (e) {
      console.error('Failed to init app', e);
    }
  },

  setSearchQuery: (query) => {
    set({ searchQuery: query });
    get().fetchPhotos(true);
  },

  setSelectedPhoto: (photo) => set({ selectedPhoto: photo }),

  fetchPhotos: async (reset = false) => {
    const { searchQuery, limit, offset, loading, hasMore } = get();
    
    if (loading || (!reset && !hasMore)) return;

    set({ loading: true });
    
    try {
      const currentOffset = reset ? 0 : offset;
      let newPhotos: Photo[] = [];

      if (searchQuery.trim()) {
        // Search ignores pagination for now based on backend search_photos signature
        newPhotos = await invoke<Photo[]>('search_photos', { query: searchQuery });
        set({ 
          photos: newPhotos, 
          hasMore: false, 
          offset: 0 
        });
      } else {
        newPhotos = await invoke<Photo[]>('get_photos', { 
          limit, 
          offset: currentOffset 
        });
        
        set((state) => ({
          photos: reset ? newPhotos : [...state.photos, ...newPhotos],
          hasMore: newPhotos.length === limit,
          offset: currentOffset + limit,
        }));
      }
    } catch (error) {
      console.error('Failed to fetch photos:', error);
    } finally {
      set({ loading: false });
    }
  },

  importFolder: async () => {
    try {
      set({ loading: true });
      await invoke('import_folder');
      await get().fetchPhotos(true);
    } catch (error) {
      console.error('Failed to import folder:', error);
    } finally {
      set({ loading: false });
    }
  }
}));
