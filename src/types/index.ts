export interface ObfuscateOptions {
  timestamp: boolean;
  silkscreen: boolean;
  geometry: boolean;
  structure: boolean;
  physical: boolean;
}

export interface ProcessRequest {
  input_path: string;
  output_dir: string | null;
  count: number;
  options: ObfuscateOptions;
}

export interface ProcessResult {
  success: boolean;
  output_files: string[];
  message: string;
}

export const defaultOptions: ObfuscateOptions = {
  timestamp: false,
  silkscreen: true,
  geometry: false,
  structure: false,
  physical: true,
};
