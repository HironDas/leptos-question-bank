const { chromium } = require("@playwright/test");

const BASE = "http://89.167.123.136:8888";

(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();

  const requests = [];
  page.on("requestfinished", async (req) => {
    const resp = await req.response();
    if (resp && resp.status() >= 400) {
      requests.push(`${resp.status()} ${req.method()} ${req.url()}`);
    }
  });
  page.on("requestfailed", (req) => {
    requests.push(`FAILED ${req.method()} ${req.url()} - ${req.failure()?.errorText}`);
  });
  page.on("console", (msg) => {
    console.log(`[console:${msg.type()}]`, msg.text());
  });
  page.on("pageerror", (err) => {
    console.log("[pageerror]", err.message);
  });

  console.log("Navigating to", BASE + "/");
  await page.goto(BASE + "/", { waitUntil: "networkidle" });

  await page.waitForSelector('input[name="login[id]"]', { timeout: 15000 });
  await page.fill('input[name="login[id]"]', "hiron");
  await page.fill('input[name="login[password]"]', "Hiron@12345");

  console.log("Submitting login form...");
  await Promise.all([
    page.waitForURL("**/home", { timeout: 15000 }).catch((e) => console.log("waitForURL /home failed:", e.message)),
    page.click('button[type="submit"]'),
  ]);

  await page.waitForTimeout(3000);

  console.log("Final URL:", page.url());
  console.log("Page title:", await page.title());
  const bodyText = await page.evaluate(() => document.body.innerText.slice(0, 500));
  console.log("Body text preview:", bodyText);

  console.log("\n=== HTTP errors / failed requests ===");
  console.log(requests.length ? requests.join("\n") : "(none)");

  await page.screenshot({ path: "repro-home-404.png", fullPage: true });
  await browser.close();
})();
