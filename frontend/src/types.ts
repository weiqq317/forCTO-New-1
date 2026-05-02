export interface Photo {
  id: number;
  path: string;
  hash: string | null;
  created_at: string;
  width: number | null;
  height: number | null;
  type: string | null;
}