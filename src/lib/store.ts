import { writable, get } from "svelte/store";
import { cmdSaveTheme, cmdGetTheme } from "./gen";

const THEMES: Record<string, string> = {
  "Light": "light",
  "Dark": "dark",
} as const;

const gTheme = writable("System");

export function getTheme(): string {
  return get(gTheme);
}

export async function setTheme(theme: string) {
  gTheme.set(theme);
  applyTheme(theme);
  await cmdSaveTheme({ theme });
}

export async function loadTheme() {
  gTheme.set(await cmdGetTheme());
  applyTheme(get(gTheme));
}

function applyTheme(theme: string) {
  theme in THEMES
    ? document.documentElement.dataset.theme = THEMES[theme]!
    : document.documentElement.removeAttribute('data-theme');
}
