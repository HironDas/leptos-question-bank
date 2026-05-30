import { test, expect } from "@playwright/test";

/**
 * Sidebar Collapsible Button E2E Tests
 *
 * Uses singlestage CSS classes directly (no data-testid):
 *   aside.singlestage-sidebar              — sidebar container
 *   button.singlestage-btn-ghost           — the single toggle button
 *
 * State is tracked via aria-hidden on <aside class="singlestage-sidebar">:
 *   - Visible:  aria-hidden="false"
 *   - Hidden:   aria-hidden="true"
 */

const SIDEBAR = "aside.singlestage-sidebar";
const TOGGLE = "button.singlestage-btn-ghost";

test.describe("Sidebar Collapsible Button", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/home");
    await page.waitForSelector(SIDEBAR, { state: "attached", timeout: 15_000 });
    await expect(page.locator(TOGGLE)).toBeAttached({ timeout: 10_000 });
  });

  test("sidebar is visible by default", async ({ page }) => {
    await expect(page.locator(SIDEBAR)).toHaveAttribute("aria-hidden", "false");
  });

  test("clicking the toggle collapses the sidebar", async ({ page }) => {
    const sidebar = page.locator(SIDEBAR);
    await expect(sidebar).toHaveAttribute("aria-hidden", "false");

    // Click the SidebarTrigger — it's the div wrapping the ghost button
    await page.locator(TOGGLE).click();
    await expect(sidebar).toHaveAttribute("aria-hidden", "true");
  });

  test("clicking the toggle again expands the sidebar", async ({ page }) => {
    const sidebar = page.locator(SIDEBAR);
    const toggle = page.locator(TOGGLE);

    await expect(sidebar).toHaveAttribute("aria-hidden", "false");
    await toggle.click();
    await expect(sidebar).toHaveAttribute("aria-hidden", "true");
    await toggle.click();
    await expect(sidebar).toHaveAttribute("aria-hidden", "false");
  });

  test("toggle button contains an SVG icon", async ({ page }) => {
    await expect(page.locator(TOGGLE).locator("svg")).toBeAttached();
  });

  test("toggles correctly through multiple cycles", async ({ page }) => {
    const sidebar = page.locator(SIDEBAR);
    const toggle = page.locator(TOGGLE);

    await toggle.click();
    await expect(sidebar).toHaveAttribute("aria-hidden", "true");
    await toggle.click();
    await expect(sidebar).toHaveAttribute("aria-hidden", "false");
    await toggle.click();
    await expect(sidebar).toHaveAttribute("aria-hidden", "true");
    await toggle.click();
    await expect(sidebar).toHaveAttribute("aria-hidden", "false");
  });

  test("toggle button is present on mobile viewport", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto("/home");
    await page.waitForSelector(SIDEBAR, { state: "attached", timeout: 15_000 });
    await expect(page.locator(TOGGLE)).toBeAttached({ timeout: 10_000 });
  });

  test("tooltip appears on hover", async ({ page }) => {
    test.skip();
  });

  test("keyboard Enter toggles sidebar", async ({ page }) => {
    test.skip();
  });
});
