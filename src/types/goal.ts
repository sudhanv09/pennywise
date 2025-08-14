export interface Goal {
  id: string;
  name: string;
  targetAmount: number;
  currentAmount: number;
  targetDate?: string | null;
  description?: string | null;
  createdAt: string;
  isCompleted: boolean;
}

export interface GoalFormData {
  name: string;
  targetAmount: number;
  targetDate?: string;
  description?: string;
}