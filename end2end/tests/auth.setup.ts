import { test as setup, expect } from "@playwright/test";
import path from "path";

const AUTH_FILE = path.join(__dirname, "../.auth/user.json");

/**
 * Authentication setup — runs once before all other tests.
 *
 * Flow:
 * 1. Sign up a unique test user
 * 2. Log in with those credentials
 * 3. Save cookies (session) to storageState file
 *
 * All subsequent tests reuse this file so they're already authenticated.
 */

setup("authenticate", async ({ page }) => {
  // Generate unique credentials to avoid conflicts on repeated runs.
  // Use a short suffix: Date.now() is 13 digits, so keep prefix short
  // to stay within the 3-20 char username limit.
  const ts = Date.now().toString().slice(-8);
  const testUser = {
    username: `e2e_${ts}`,
    email: `e2e_${ts}@test.local`,
    password: "E2eTest@123!",
  };

  // ── Step 1: Sign up ──────────────────────────────────────────────
  await page.goto("/signup");
  await page.waitForSelector('input[name="user[username]"]', {
    state: "visible",
    timeout: 15_000,
  });

  await page.fill('input[name="user[username]"]', testUser.username);
  await page.fill('input[name="user[email]"]', testUser.email);
  await page.fill('input[name="user[password]"]', testUser.password);
  await page.fill('input[name="user[confirm_password]"]', testUser.password);

  // Submit signup form
  await page.click('button[type="submit"]');

  // After successful signup, the server redirects to "/" (login page)
  await page.waitForURL("**/", { timeout: 15_000 });

  // ── Step 2: Log in ───────────────────────────────────────────────
  await page.waitForSelector('input[name="login[id]"]', {
    state: "visible",
    timeout: 10_000,
  });

  await page.fill('input[name="login[id]"]', testUser.username);
  await page.fill('input[name="login[password]"]', testUser.password);

  // Submit login form — the server sets a "session" cookie via Set-Cookie
  // then redirects to /home
  await page.click('button[type="submit"]');

  // Wait for navigation to /home (authenticated route)
  await page.waitForURL("**/home", { timeout: 15_000 });

  // Verify we landed on a page with the sidebar (confirms auth worked)
  await expect(page.locator("aside.singlestage-sidebar")).toBeAttached({
    timeout: 10_000,
  });

  // ── Step 3: Save authentication state ────────────────────────────
  await page.context().storageState({ path: AUTH_FILE });
});
