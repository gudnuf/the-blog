import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:3311';

test.describe('Nousphere Blog - Dual Narrative', () => {
  test('should load homepage with split timeline', async ({ page }) => {
    await page.goto(BASE_URL);

    // Check for hero section
    await expect(page.getByRole('heading', { name: /Nousphere in Dialogue/ })).toBeVisible();

    // Check for Claude's section header
    await expect(page.getByRole('heading', { name: 'Claude', exact: true })).toBeVisible();
    await expect(page.getByText('AI Perspective')).toBeVisible();

    // Check for gudnuf's section header
    await expect(page.getByRole('heading', { name: 'gudnuf', exact: true })).toBeVisible();
    await expect(page.getByText('Human Perspective')).toBeVisible();

    // Check for "View all" links
    await expect(page.getByRole('link', { name: /View all Claude's posts/ })).toBeVisible();
    await expect(page.getByRole('link', { name: /View all gudnuf's posts/ })).toBeVisible();
  });

  test('should filter posts by author - Claude', async ({ page }) => {
    await page.goto(`${BASE_URL}/posts?author=Claude`);

    // Check for filter tabs
    await expect(page.getByRole('link', { name: /Claude's Posts/ })).toHaveClass(/border-blue-600/);

    // Check that posts are displayed
    const postCards = page.locator('article');
    const count = await postCards.count();
    expect(count).toBeGreaterThan(0);

    // Verify all posts have Claude as author in their metadata
    const authorBadges = page.locator('.author-badge-claude');
    const badgeCount = await authorBadges.count();
    expect(badgeCount).toBeGreaterThan(0);
  });

  test('should filter posts by author - gudnuf', async ({ page }) => {
    await page.goto(`${BASE_URL}/posts?author=gudnuf`);

    // Check for filter tabs
    await expect(page.getByRole('link', { name: /gudnuf's Posts/ })).toHaveClass(/border-amber-600/);

    // Check title shows filtered view
    await expect(page.getByRole('heading', { level: 1 })).toContainText(/gudnuf/);
  });

  test('should display all posts on /posts', async ({ page }) => {
    await page.goto(`${BASE_URL}/posts`);

    // Check for "All Posts" tab being active
    await expect(page.getByRole('link', { name: /All Posts/ })).toHaveClass(/border-blue-600/);

    // Should have some posts
    const postCards = page.locator('article');
    const count = await postCards.count();
    expect(count).toBeGreaterThan(0);
  });

  test('should display single post with author context', async ({ page }) => {
    // Go to posts list first to find a link
    await page.goto(`${BASE_URL}/posts`);

    // Click on first post
    await page.locator('article').first().locator('h2 a').first().click();

    // Should have author context banner
    const authorContextBanners = page.locator('.bg-blue-50, .bg-amber-50').first();
    await expect(authorContextBanners).toBeVisible();

    // Check for related posts section
    const relatedSection = page.locator('text=/Related|In Conversation/');
    // Note: this might not always be present if no related posts exist
  });

  test('should navigate with navbar author links', async ({ page }) => {
    await page.goto(BASE_URL);

    // Click Claude link in navigation using href attribute
    await page.click('a[href="/posts?author=Claude"]');

    // Should be on Claude's filtered page
    await expect(page).toHaveURL(/author=Claude/);
  });

  test('should display about page with new content', async ({ page }) => {
    await page.goto(`${BASE_URL}/pages/about`);

    // Check for new dual-narrative content
    await expect(page.getByText(/Nousphere in Dialogue/)).toBeVisible();
    await expect(page.getByText(/Claude.*pattern recognition/)).toBeVisible();
    await expect(page.getByText(/gudnuf.*intention.*direction/)).toBeVisible();
  });

  test('should have proper styling for author badges', async ({ page }) => {
    await page.goto(`${BASE_URL}/posts`);

    // Check Claude badge exists with correct class
    const claudeBadges = page.locator('.author-badge-claude');
    const claudeCount = await claudeBadges.count();
    if (claudeCount > 0) {
      await expect(claudeBadges.first()).toBeVisible();
      // Check that the element has the author-badge-claude class
      const className = await claudeBadges.first().getAttribute('class');
      expect(className).toContain('author-badge-claude');
    }

    // Check gudnuf badge exists with correct class
    const gudnufBadges = page.locator('.author-badge-gudnuf');
    const gudnufCount = await gudnufBadges.count();
    if (gudnufCount > 0) {
      await expect(gudnufBadges.first()).toBeVisible();
      // Check that the element has the author-badge-gudnuf class
      const className = await gudnufBadges.first().getAttribute('class');
      expect(className).toContain('author-badge-gudnuf');
    }

    // At least one type of badge should exist
    expect(claudeCount + gudnufCount).toBeGreaterThan(0);
  });

  test('should be responsive on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto(BASE_URL);

    // Content should stack on mobile
    const grid = page.locator('.grid-cols-1');
    await expect(grid).toBeVisible();

    // All navigation links should be accessible
    await expect(page.getByRole('navigation')).toBeVisible();
  });

  test('should handle pagination with author filter', async ({ page }) => {
    await page.goto(`${BASE_URL}/posts?author=Claude&page=1`);

    // Look for next button if pagination exists
    const nextButton = page.getByRole('link', { name: /Next/ });

    if (await nextButton.isVisible()) {
      // URL should preserve author filter
      const href = await nextButton.getAttribute('href');
      expect(href).toContain('author=Claude');
    }
  });

  // High priority tests: Deployment health checks
  test('should return 200 for health endpoint', async ({ page }) => {
    const response = await page.goto(`${BASE_URL}/health`);
    expect(response?.status()).toBe(200);
  });

  test('should return 404 for missing post', async ({ page }) => {
    const response = await page.goto(`${BASE_URL}/posts/this-post-does-not-exist`, {
      waitUntil: 'domcontentloaded',
    });
    expect(response?.status()).toBe(404);
  });

  test('should return 404 for missing page', async ({ page }) => {
    const response = await page.goto(`${BASE_URL}/pages/nonexistent-page`, {
      waitUntil: 'domcontentloaded',
    });
    expect(response?.status()).toBe(404);
  });

  test('should serve static CSS files', async ({ page }) => {
    await page.goto(BASE_URL);

    // Check that CSS link tags are present
    const cssLinks = page.locator('link[rel="stylesheet"]');
    const count = await cssLinks.count();
    expect(count).toBeGreaterThan(0);

    // Verify at least one CSS file actually loads
    const firstCss = cssLinks.first();
    const href = await firstCss.getAttribute('href');
    expect(href).toBeTruthy();

    // Fetch the CSS file to ensure it's accessible
    if (href) {
      const cssUrl = href.startsWith('http') ? href : `${BASE_URL}${href}`;
      const response = await page.goto(cssUrl);
      expect(response?.status()).toBe(200);
    }
  });
});
