import { test, expect } from "@playwright/test";

const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:4173";

test("non-home search uses native navigation and initializes q", async ({ page }) => {
  const localApiRequests: string[] = [];
  page.on("request", (request) => {
    const url = new URL(request.url());
    if (url.origin === baseURL && url.pathname.startsWith("/api/")) {
      localApiRequests.push(url.pathname);
    }
  });

  await page.goto(`${baseURL}/about`);
  const search = page.locator(".search-input");
  await search.fill("Tor Browser");
  await expect(page).toHaveURL(`${baseURL}/about`);

  const navigation = page.waitForNavigation();
  await search.press("Enter");
  const response = await navigation;

  expect(response?.request().resourceType()).toBe("document");
  expect(new URL(page.url()).searchParams.get("q")).toBe("Tor Browser");
  await expect(page.locator(".search-input")).toHaveValue("Tor Browser");
  await expect(page.locator(".post-card")).toHaveCount(1);
  await expect(page.locator(".post-card")).toContainText("Tor Browser");

  const searchURL = page.url();
  await page.locator(".search-input").fill("QEMU");
  await expect(page).toHaveURL(searchURL);
  await expect(page.locator(".post-card")).toHaveCount(2);
  await expect(page.locator(".post-card h2")).toHaveText(["VM? QEMU!", "VM? QEMU!"]);
  expect(localApiRequests).toEqual([]);
});
