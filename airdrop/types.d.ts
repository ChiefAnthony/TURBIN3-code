declare module 'prompt-sync' {
  const prompt: (options?: { sigint?: boolean }) => (query: string) => string;
  export = prompt;
}

