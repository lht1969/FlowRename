import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { skeleton } from "@skeletonlabs/tw-plugin";

const skeletonContentPath = join(
  dirname(fileURLToPath(import.meta.resolve("@skeletonlabs/skeleton"))),
  "..",
  "**",
  "*.{html,js,svelte,ts}",
);

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}", skeletonContentPath],
  theme: {
    extend: {},
  },
  plugins: [
    skeleton({
      themes: {
        preset: ["modern", "wintry"],
        custom: [
          // ═══════════════════════════════════════════════
          // Theme: Obsidian (黑曜石)
          // High-contrast dark theme with cyan accents.
          // Deep blue-black backgrounds for maximum readability.
          // ═══════════════════════════════════════════════
          {
            name: "obsidian",
            properties: {
              "--theme-font-family-base":
                '"Inter Variable", "Segoe UI", system-ui, sans-serif',
              "--theme-font-family-heading":
                '"Inter Variable", "Segoe UI", system-ui, sans-serif',
              "--theme-font-color-base": "185 195 215",
              "--theme-font-color-dark": "200 210 230",
              "--theme-rounded-base": "6px",
              "--theme-rounded-container": "8px",
              "--theme-border-base": "1px",

              "--on-primary": "255 255 255",
              "--on-secondary": "255 255 255",
              "--on-tertiary": "255 255 255",
              "--on-success": "255 255 255",
              "--on-warning": "0 0 0",
              "--on-error": "255 255 255",
              "--on-surface": "200 210 230",

              // Primary: Cyan
              "--color-primary-50": "236 254 255",
              "--color-primary-100": "207 250 254",
              "--color-primary-200": "165 243 252",
              "--color-primary-300": "103 232 249",
              "--color-primary-400": "34 211 238",
              "--color-primary-500": "6 182 212",
              "--color-primary-600": "8 145 178",
              "--color-primary-700": "14 116 144",
              "--color-primary-800": "21 94 117",
              "--color-primary-900": "22 78 99",

              // Secondary: Indigo
              "--color-secondary-50": "238 242 255",
              "--color-secondary-100": "224 231 255",
              "--color-secondary-200": "199 210 254",
              "--color-secondary-300": "165 180 252",
              "--color-secondary-400": "129 140 248",
              "--color-secondary-500": "99 102 241",
              "--color-secondary-600": "79 70 229",
              "--color-secondary-700": "67 56 202",
              "--color-secondary-800": "55 48 163",
              "--color-secondary-900": "49 46 129",

              // Tertiary: Teal
              "--color-tertiary-50": "240 253 250",
              "--color-tertiary-100": "204 251 241",
              "--color-tertiary-200": "153 246 228",
              "--color-tertiary-300": "94 234 212",
              "--color-tertiary-400": "45 212 191",
              "--color-tertiary-500": "20 184 166",
              "--color-tertiary-600": "13 148 136",
              "--color-tertiary-700": "15 118 110",
              "--color-tertiary-800": "17 94 89",
              "--color-tertiary-900": "19 78 74",

              // Success: Green
              "--color-success-50": "240 253 244",
              "--color-success-100": "220 252 231",
              "--color-success-200": "187 247 208",
              "--color-success-300": "134 239 172",
              "--color-success-400": "74 222 128",
              "--color-success-500": "34 197 94",
              "--color-success-600": "22 163 74",
              "--color-success-700": "21 128 61",
              "--color-success-800": "22 101 52",
              "--color-success-900": "20 83 45",

              // Warning: Amber
              "--color-warning-50": "255 251 235",
              "--color-warning-100": "254 243 199",
              "--color-warning-200": "253 230 138",
              "--color-warning-300": "252 211 77",
              "--color-warning-400": "251 191 36",
              "--color-warning-500": "245 158 11",
              "--color-warning-600": "217 119 6",
              "--color-warning-700": "180 83 9",
              "--color-warning-800": "146 64 14",
              "--color-warning-900": "120 53 15",

              // Error: Red
              "--color-error-50": "254 242 242",
              "--color-error-100": "254 226 226",
              "--color-error-200": "254 202 202",
              "--color-error-300": "252 165 165",
              "--color-error-400": "248 113 113",
              "--color-error-500": "239 68 68",
              "--color-error-600": "220 38 38",
              "--color-error-700": "185 28 28",
              "--color-error-800": "153 27 27",
              "--color-error-900": "127 29 29",

              // Surface: Deep blue-black (inverted for dark theme)
              "--color-surface-50": "14 16 22",
              "--color-surface-100": "20 23 31",
              "--color-surface-200": "27 31 42",
              "--color-surface-300": "36 41 54",
              "--color-surface-400": "47 53 69",
              "--color-surface-500": "62 69 88",
              "--color-surface-600": "82 91 112",
              "--color-surface-700": "140 150 175",
              "--color-surface-800": "185 195 215",
              "--color-surface-900": "220 228 242",
            },
            properties_dark: {},
            enhancements: {
              "[data-theme='obsidian']": {
                backgroundImage:
                  "radial-gradient(circle at 50% 0%, rgba(6,182,212,0.06) 0%, transparent 60%)",
                backgroundAttachment: "fixed",
                backgroundPosition: "center",
                backgroundRepeat: "no-repeat",
                backgroundSize: "cover",
              },
            },
          },
          {
            name: "daylight",
            properties: {
              "--theme-font-family-base":
                '"Inter Variable", "Segoe UI", system-ui, sans-serif',
              "--theme-font-family-heading":
                '"Inter Variable", "Segoe UI", system-ui, sans-serif',
              "--theme-font-color-base": "50 56 68",
              "--theme-font-color-dark": "32 36 44",
              "--theme-rounded-base": "6px",
              "--theme-rounded-container": "8px",
              "--theme-border-base": "1px",

              "--on-primary": "255 255 255",
              "--on-secondary": "255 255 255",
              "--on-tertiary": "255 255 255",
              "--on-success": "255 255 255",
              "--on-warning": "0 0 0",
              "--on-error": "255 255 255",
              "--on-surface": "50 56 68",

              "--color-primary-50": "239 246 255",
              "--color-primary-100": "219 234 254",
              "--color-primary-200": "191 219 254",
              "--color-primary-300": "147 197 253",
              "--color-primary-400": "96 165 250",
              "--color-primary-500": "37 99 235",
              "--color-primary-600": "29 78 216",
              "--color-primary-700": "22 63 186",
              "--color-primary-800": "17 51 153",
              "--color-primary-900": "13 41 120",

              "--color-secondary-50": "238 242 255",
              "--color-secondary-100": "224 231 255",
              "--color-secondary-200": "199 210 254",
              "--color-secondary-300": "165 180 252",
              "--color-secondary-400": "129 140 248",
              "--color-secondary-500": "99 102 241",
              "--color-secondary-600": "79 70 229",
              "--color-secondary-700": "67 56 202",
              "--color-secondary-800": "55 48 163",
              "--color-secondary-900": "49 46 129",

              "--color-tertiary-50": "240 253 250",
              "--color-tertiary-100": "204 251 241",
              "--color-tertiary-200": "153 246 228",
              "--color-tertiary-300": "94 234 212",
              "--color-tertiary-400": "45 212 191",
              "--color-tertiary-500": "20 184 166",
              "--color-tertiary-600": "13 148 136",
              "--color-tertiary-700": "15 118 110",
              "--color-tertiary-800": "17 94 89",
              "--color-tertiary-900": "19 78 74",

              "--color-success-50": "240 253 244",
              "--color-success-100": "220 252 231",
              "--color-success-200": "187 247 208",
              "--color-success-300": "134 239 172",
              "--color-success-400": "74 222 128",
              "--color-success-500": "22 163 74",
              "--color-success-600": "21 128 61",
              "--color-success-700": "22 101 52",
              "--color-success-800": "20 83 45",
              "--color-success-900": "15 66 35",

              "--color-warning-50": "255 251 235",
              "--color-warning-100": "254 243 199",
              "--color-warning-200": "253 230 138",
              "--color-warning-300": "252 211 77",
              "--color-warning-400": "251 191 36",
              "--color-warning-500": "245 158 11",
              "--color-warning-600": "217 119 6",
              "--color-warning-700": "180 83 9",
              "--color-warning-800": "146 64 14",
              "--color-warning-900": "120 53 15",

              "--color-error-50": "254 242 242",
              "--color-error-100": "254 226 226",
              "--color-error-200": "254 202 202",
              "--color-error-300": "252 165 165",
              "--color-error-400": "248 113 113",
              "--color-error-500": "239 68 68",
              "--color-error-600": "220 38 38",
              "--color-error-700": "185 28 28",
              "--color-error-800": "153 27 27",
              "--color-error-900": "127 29 29",

              "--color-surface-50": "255 255 255",
              "--color-surface-100": "248 249 251",
              "--color-surface-200": "241 243 246",
              "--color-surface-300": "233 236 241",
              "--color-surface-400": "222 226 232",
              "--color-surface-500": "173 181 194",
              "--color-surface-600": "108 117 134",
              "--color-surface-700": "73 80 95",
              "--color-surface-800": "50 56 68",
              "--color-surface-900": "32 36 44",
            },
            properties_dark: {},
            enhancements: {},
          },
        ],
      },
    }),
  ],
};
