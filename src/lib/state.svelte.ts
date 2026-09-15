import { cmdSaveTheme, cmdGetSettings } from "./gen";

export const gUserState = $state({
  theme: "",
  tag: "",
  tags: [""],
});

export function getTheme(): string {
  return gUserState.theme;
}

export async function setTheme(theme: string) {
  gUserState.theme = theme;
  applyTheme(theme);
  await cmdSaveTheme({ theme });
}

export async function loadState() {
  let settings = await cmdGetSettings();
  gUserState.tag = settings.tag;
  gUserState.tags = settings.tags;
  gUserState.theme = settings.theme;
  applyTheme(settings.theme);
}

const THEMES: Record<string, string> = {
  "Light": "light",
  "Dark": "dark",
} as const;

function applyTheme(theme: string) {
  theme in THEMES
    ? document.documentElement.dataset.theme = THEMES[theme]!
    : document.documentElement.removeAttribute('data-theme');
}
