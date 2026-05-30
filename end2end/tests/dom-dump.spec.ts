import { test, expect } from "@playwright/test";

/**
 * Quick diagnostic: is the sidebar toggle button visible?
 */
test("check toggle visibility", async ({ page }) => {
  // Login first (reuse existing setup if available, or create new)
  await page.goto("/home");
  await page.waitForSelector("aside.singlestage-sidebar", { state: "attached", timeout: 15_000 });
  await page.waitForTimeout(3000);

  // Check ALL buttons for visibility
  const result = await page.evaluate(() => {
    const buttons = document.querySelectorAll("button.singlestage-btn-ghost");
    return Array.from(buttons).map((btn, i) => {
      const style = getComputedStyle(btn);
      const parentStyle = getComputedStyle(btn.parentElement!);
      const gpStyle = getComputedStyle(btn.parentElement!.parentElement!);
      const ggpStyle = getComputedStyle(btn.parentElement!.parentElement!.parentElement!);
      return {
        index: i,
        classes: btn.className,
        display: style.display,
        parentDisplay: parentStyle.display,
        parentClasses: btn.parentElement!.className,
        gpDisplay: gpStyle.display,
        gpClasses: btn.parentElement!.parentElement!.className,
        ggpDisplay: ggpStyle.display,
        ggpClasses: btn.parentElement!.parentElement!.parentElement!.className,
        visible: btn.checkVisibility(),
        boundingBox: btn.getBoundingClientRect(),
      };
    });
  });
  console.log(JSON.stringify(result, null, 2));

  // Also check the header structure
  const headerInfo = await page.evaluate(() => {
    const header = document.querySelector("main > main > header");
    if (!header) return "NO CONTENT HEADER";
    return {
      innerHTML: header.innerHTML.substring(0, 500),
      childCount: header.children.length,
    };
  });
  console.log("=== HEADER INFO ===");
  console.log(JSON.stringify(headerInfo, null, 2));

  expect(true).toBe(true);
});
