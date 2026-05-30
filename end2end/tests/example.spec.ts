import { test, expect } from "@playwright/test";

/**
 * Basic smoke tests.
 *
 * Uses data-testid attributes from layout.rs where available, with
 * fallback selectors for the current (non-rebuilt) binary.
 */

test("app loads and renders the login page", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle("Question Bank");

  // Login form fields
  await expect(page.locator('input[name="login[id]"]')).toBeVisible({
    timeout: 10_000,
  });
  await expect(page.locator('input[name="login[password]"]')).toBeVisible();
  await expect(page.locator('button[type="submit"]')).toContainText("Log in");
});

test("signup page loads and renders the form", async ({ page }) => {
  await page.goto("/signup");

  await expect(page).toHaveTitle("Question Bank");

  // Signup form fields
  await expect(page.locator('input[name="user[username]"]')).toBeVisible({
    timeout: 10_000,
  });
  await expect(page.locator('input[name="user[email]"]')).toBeVisible();
  await expect(page.locator('input[name="user[password]"]')).toBeVisible();
  await expect(page.locator('input[name="user[confirm_password]"]')).toBeVisible();
});

test("navigation to home page shows sidebar layout (authenticated)", async ({ page }) => {
  // Auth cookies are pre-loaded from the setup project
  await page.goto("/home");

  // Sidebar container
  const sidebar = page.locator("aside.singlestage-sidebar");
  await expect(sidebar).toBeAttached({ timeout: 15_000 });

  // Toggle button
  await expect(page.locator("button.singlestage-btn-ghost")).toBeAttached({ timeout: 10_000 });
});
