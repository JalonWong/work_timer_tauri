import { cmdGetSettings, cmdSaveSettings } from "./gen";

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
}

export async function loadSettings() {
  let settings = await cmdGetSettings();
  gUserState.tag = settings.tag;
  gUserState.tags = settings.tags;
  gUserState.theme = settings.theme;
  applyTheme(settings.theme);
}

export async function saveSettings() {
  cmdSaveSettings({
    settings: {
      theme: gUserState.theme,
      tag: gUserState.tag,
      tags: gUserState.tags,
    }
  });
}

function applyTheme(theme: string) {
  theme == ""
    ? document.documentElement.removeAttribute('data-theme')
    : document.documentElement.dataset.theme = theme;
}
