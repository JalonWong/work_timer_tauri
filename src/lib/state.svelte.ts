import { cmdGetSettings, cmdSaveSettings, cmdStopTimer } from "./gen";
import type { TimerSetting } from "./gen"
import { saveWindowState, restoreStateCurrent, StateFlags } from '@tauri-apps/plugin-window-state';

if (!import.meta.env.DEV) {
  document.addEventListener('contextmenu', (e) => e.preventDefault(), { capture: true });
}

export async function stopTimer() {
  await cmdStopTimer();
}

export const gUserState: {
  theme: string;
  timers: TimerSetting[];
  intervalId: number | null;
  countString: string;
  isTimeout: boolean;
} = $state({
  theme: "",
  timers: [],
  intervalId: null,
  countString: "",
  isTimeout: false,
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
    save_history: false,
    count_up: false,
    play_a_sound: false,
    notification: false,
    show_window: false,
  })
}

export function deleteTimer(index: number) {
  gUserState.timers.splice(index, 1);
}

export function moveTimer(index: number, up: boolean) {
  moveItemInPlace(gUserState.timers, index, up);
}

export async function saveSettings() {
  cmdSaveSettings({
    settings: {
      theme: gUserState.theme,
      timers: gUserState.timers,
    }
  });
}

export async function onStart() {
  let settings = await cmdGetSettings();
  gUserState.theme = settings.theme;
  gUserState.timers = settings.timers;

  await applyTheme(settings.theme);
  restoreStateCurrent(StateFlags.ALL);
}

export async function onExit() {
  saveWindowState(StateFlags.ALL);
  saveSettings();
}

function two_digits(v: number): string {
  return Math.trunc(v).toString().padStart(2, "0");
}

export function secsToString(secs: number): string {
  let rst = "";
  const minutes = secs / 60;
  if (minutes >= 60) {
    const hours = minutes / 60;
    rst = two_digits(hours) + ":" + two_digits(minutes % 60);
  } else {
    rst = two_digits(minutes);
  }
  rst = rst + ":" + two_digits(secs % 60);
  return rst;
}
