import { cmdGetSettings, cmdSaveSettings, WindowInfo } from "./gen";
import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window';

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
  const win = getCurrentWindow();

  const position = await win.innerPosition();
  let size = await win.innerSize();
  const osize = await win.outerSize();

  // for wayland bug
  if (size.width == osize.width) {
    size.width -= 90;
  }
  if (size.height == osize.height) {
    size.height -= 138;
  }

  cmdSaveSettings({
    settings: {
      win_info: {
        maximized: await win.isMaximized(),
        x: position.x,
        y: position.y,
        width: size.width,
        height: size.height,
      },
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
