import { cmdGetSettings, cmdSaveSettings } from "./gen";
import type { TimerSetting } from "./gen"
import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window';

export const gUserState: {
  theme: string;
  tag: string;
  tags: string[];
  timers: TimerSetting[];
} = $state({
  theme: "",
  tag: "",
  tags: [],
  timers: [],
});

export function getTheme(): string {
  return gUserState.theme;
}

function applyTheme(theme: string) {
  theme == ""
    ? document.documentElement.removeAttribute('data-theme')
    : document.documentElement.dataset.theme = theme;
}

export async function setTheme(theme: string) {
  gUserState.theme = theme;
  applyTheme(theme);
}

function moveItemInPlace<T>(arr: T[], index: number, up: boolean): void {
  let to_index = index;
  if (up) {
    if (index == 0) {
      return;
    }
    to_index -= 1;
  } else {
    to_index += 1;
    if (to_index >= arr.length) {
      return;
    }
  }

  const [item] = arr.splice(index, 1);
  arr.splice(to_index, 0, item);
}

export function newTimer() {
  gUserState.timers.push({
    label: "new",
    limit_time: 5,
    for_work: false,
    count_up: false,
    notify: false,
  })
}

export function deleteTimer(index: number) {
  gUserState.timers.splice(index, 1);
}

export function moveTimer(index: number, up: boolean) {
  moveItemInPlace(gUserState.timers, index, up);
}

export function newTag() {
  gUserState.tags.push("new")
}

export function deleteTag(index: number) {
  gUserState.tags.splice(index, 1);
}

export function moveTag(index: number, up: boolean) {
  moveItemInPlace(gUserState.tags, index, up);
}

export async function loadSettings() {
  let settings = await cmdGetSettings();
  gUserState.tag = settings.tag;
  gUserState.tags = settings.tags;
  gUserState.theme = settings.theme;
  applyTheme(settings.theme);
  gUserState.timers = settings.timers;
}

export async function saveSettings() {
  const win = getCurrentWindow();

  const position = await win.outerPosition();
  let size = await win.innerSize();
  const osize = await win.outerSize();

  // for wayland bug
  // if (size.width == osize.width) {
  //   size.width -= 90;
  // }
  // if (size.height == osize.height) {
  //   size.height -= 138;
  // }

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
      timers: gUserState.timers,
    }
  });
}
