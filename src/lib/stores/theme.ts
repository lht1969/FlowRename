// AdRename - Theme store with persistence
// Manages dark/light theme switching with localStorage persistence

import { writable } from "svelte/store";
import { browser } from "$app/environment";

/** Available Skeleton UI themes */
export const AVAILABLE_THEMES = [
  { id: "modern", label: "Modern", isDark: true },
  { id: "wintry", label: "Wintry", isDark: true },
  { id: "obsidian", label: "Obsidian", isDark: true },
  { id: "daylight", label: "Daylight", isDark: false },
] as const;

export type ThemeId = (typeof AVAILABLE_THEMES)[number]["id"];

/** 从 localStorage 恢复主题设置 */
function getStoredTheme(): ThemeId {
  if (!browser) return "wintry";
  const stored = localStorage.getItem("adr-theme");
  if (stored && AVAILABLE_THEMES.some((t) => t.id === stored)) {
    return stored as ThemeId;
  }
  return "wintry";
}

/** 主题 Store - 响应式主题状态管理 */
function createThemeStore() {
  const { subscribe, set, update } = writable<ThemeId>(getStoredTheme());

  return {
    subscribe,
    /** 设置主题并持久化到 localStorage */
    set: (theme: ThemeId) => {
      if (browser) {
        localStorage.setItem("adr-theme", theme);
        document.body.setAttribute("data-theme", theme);
      }
      set(theme);
    },
    /** 切换到下一个主题 */
    cycle: () => {
      update((current) => {
        const idx = AVAILABLE_THEMES.findIndex((t) => t.id === current);
        const next = AVAILABLE_THEMES[(idx + 1) % AVAILABLE_THEMES.length];
        if (browser) {
          localStorage.setItem("adr-theme", next.id);
          document.body.setAttribute("data-theme", next.id);
        }
        return next.id;
      });
    },
    /** 初始化主题（在 layout 加载时调用） */
    init: () => {
      if (browser) {
        const theme = getStoredTheme();
        document.body.setAttribute("data-theme", theme);
        set(theme);
      }
    },
  };
}

export const themeStore = createThemeStore();
