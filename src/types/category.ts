export interface Category {
  id: number;
  name: string | null;
  icon: string | null;
  color: string | null;
}

export interface CategoryFormData {
  name: string;
  icon: string;
  color: string;
}