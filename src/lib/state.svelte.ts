import { cmdGetSettings, cmdSaveSettings, cmdStopTimer } from "./gen";
import type { TimerSetting } from "./gen"
import { saveWindowState, restoreStateCurrent, StateFlags } from '@tauri-apps/plugin-window-state';

export async function stopTimer() {
  await cmdStopTimer();
}

export const gUserState: {
  theme: string;
  timers: TimerSetting[];
} = $state({
  theme: "",
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

export async function loadSettings() {
  let settings = await cmdGetSettings();
  gUserState.theme = settings.theme;
  gUserState.timers = settings.timers;

  await applyTheme(settings.theme);
  restoreStateCurrent(StateFlags.ALL);
}

export async function saveSettings() {
  saveWindowState(StateFlags.ALL);
  cmdSaveSettings({
    settings: {
      theme: gUserState.theme,
      timers: gUserState.timers,
    }
  });
}
