export interface UserSettings {
  id: number;
  defaultCurrency: string;
  dateFormat: string;
  theme: "light" | "dark" | "system";
  updatedAt: string;
}

export interface UserSettingsFormData {
  defaultCurrency: string;
  dateFormat: string;
  theme: "light" | "dark" | "system";
}