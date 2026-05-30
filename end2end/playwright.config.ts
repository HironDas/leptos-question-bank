import type { PlaywrightTestConfig } from "@playwright/test";
import { devices, defineConfig } from "@playwright/test";
import path from "path";

const AUTH_FILE = path.join(__dirname, ".auth/user.json");

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  testDir: "./tests",
  /* Maximum time one test can run for. */
  timeout: 60 * 1000,
  expect: {
    /**
     * Maximum time expect() should wait for the condition to be met.
     */
    timeout: 10_000,
  },
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: process.env.CI ? 1 : undefined,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: "html",
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    /* Maximum time each action such as `click()` can take. Defaults to 0 (no limit). */
    actionTimeout: 10_000,
    /* Base URL to use in actions like `await page.goto('/')`. */
    baseURL: "http://127.0.0.1:3000",

    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: "on-first-retry",

    /* Take screenshot on failure */
    screenshot: "only-on-failure",
  },

  /* Configure projects for major browsers */
  projects: [
    /* ── Authentication setup — runs once before all browsers ─────── */
    {
      name: "setup",
      testMatch: /auth\.setup\.ts/,
    },

    /* ── Diagnostic project — standalone, no auth dependency ──────── */
    {
      name: "dom-dump",
      testMatch: /dom-dump\.spec\.ts/,
    },

    /* ── Browser projects — depend on setup for auth state ───────── */
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        /* Load cookies from the setup run */
        storageState: AUTH_FILE,
      },
      dependencies: ["setup"],
    },

    {
      name: "firefox",
      use: {
        ...devices["Desktop Firefox"],
        storageState: AUTH_FILE,
      },
      dependencies: ["setup"],
    },

    {
      name: "webkit",
      use: {
        ...devices["Desktop Safari"],
        storageState: AUTH_FILE,
      },
      dependencies: ["setup"],
    },
  ],

  /* Run your local dev server before starting the tests */
  webServer: {
    command: "cargo leptos watch --split",
    url: "http://127.0.0.1:3000",
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000,
    cwd: "..",
    stdout: "pipe",
    stderr: "pipe",
  },

  /* Folder for test artifacts such as screenshots, videos, traces, etc. */
  outputDir: "./test-results",
});
