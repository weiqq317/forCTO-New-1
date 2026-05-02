import { useState } from 'react';
import type { FormEvent } from 'react';
import { useStore } from '../store/useStore';

export const SearchBar = () => {
  const [localQuery, setLocalQuery] = useState('');
  const { setSearchQuery, importFolder, loading } = useStore();

  const handleSearch = (e: FormEvent) => {
    e.preventDefault();
    setSearchQuery(localQuery);
  };

  return (
    <div className="flex items-center gap-4 p-4 bg-gray-900 border-b border-gray-800">
      <form onSubmit={handleSearch} className="flex-1 max-w-xl flex gap-2">
        <input
          type="text"
          value={localQuery}
          onChange={(e) => setLocalQuery(e.target.value)}
          placeholder="Search by tag..."
          className="flex-1 px-4 py-2 bg-gray-800 border border-gray-700 rounded-md focus:outline-none focus:border-blue-500 text-white"
        />
        <button
          type="submit"
          className="px-6 py-2 bg-blue-600 hover:bg-blue-700 rounded-md text-white font-medium transition-colors"
        >
          Search
        </button>
      </form>
      <button
        onClick={importFolder}
        disabled={loading}
        className="px-4 py-2 bg-gray-800 hover:bg-gray-700 border border-gray-700 rounded-md text-white font-medium transition-colors disabled:opacity-50"
      >
        {loading ? 'Importing...' : 'Import Folder'}
      </button>
    </div>
  );
};
