# Plan V2 - Next Steps

**Status**: 📋 Planning
**Target Completion**: TBD
**Version**: 2.0.0

## Overview

This document outlines the next iteration of features and improvements for the Rust SSR Blog. V2 focuses on enhancing discoverability, improving SEO, and adding features that make the blog more useful for readers and search engines.

## Prerequisites

Before starting V2 implementation:
- [ ] Fix critical bugs from V1 (Darwin SDK, footer date)
- [ ] Verify all V1 tests pass
- [ ] Confirm deployment works on NixOS

## V2 Goals

1. **Improve Discoverability**: RSS feeds, sitemaps, better navigation
2. **Enhance SEO**: Meta tags, structured data, canonical URLs
3. **Better Content Organization**: Tag/category archives, search
4. **Reader Experience**: Related posts, reading time, improved navigation
5. **Maintainability**: Better testing, CI/CD setup

## Priority Levels

- **P0**: Critical (blocks other work, affects core functionality)
- **P1**: High (important for production readiness)
- **P2**: Medium (nice to have, improves experience)
- **P3**: Low (future enhancement)

## Phase 1: Bug Fixes and Stability (P0)

### 1.1 Fix Critical Bugs
**Priority**: P0
**Effort**: 1-2 hours

- [ ] Fix Darwin SDK reference in flake.nix
  - Change `apple-sdk` to proper framework paths
  - Test on macOS to verify build works
- [ ] Fix hardcoded copyright year in footer.html
  - Revert to `{{ "now" | date(format="%Y") }}`
- [ ] Add regression tests for both fixes

**Success Criteria**:
- `nix build` works on macOS
- Footer shows current year dynamically
- All existing tests pass

## Phase 2: RSS/Atom Feeds (P1)

### 2.1 RSS Feed Generation
**Priority**: P1
**Effort**: 4-6 hours

**Features**:
- Generate RSS 2.0 feed at `/rss.xml`
- Include last 20 posts
- Support full content or summary (configurable)
- Proper XML escaping
- Validate against RSS 2.0 spec

**Implementation**:
- Create `crates/blog-content/src/rss.rs` module
- Add RSS generation functions
- Add route handler in `blog-server`
- Add template for RSS XML
- Add configuration options (feed_items_count, feed_full_content)

**Dependencies**:
- Consider using `rss` crate (https://crates.io/crates/rss)
- Alternative: Build XML manually with `quick-xml`

**Testing**:
- Validate RSS feed with https://validator.w3.org/feed/
- Test with feed readers (NetNewsWire, Feedly)

### 2.2 Atom Feed Generation
**Priority**: P2
**Effort**: 2-3 hours

- Generate Atom 1.0 feed at `/atom.xml`
- Similar features to RSS
- Add `<link rel="alternate">` tags in HTML head

**Success Criteria**:
- RSS feed validates
- Atom feed validates
- Feeds update when new posts are added
- Links in HTML templates point to feeds

## Phase 3: Sitemap Generation (P1)

### 3.1 XML Sitemap
**Priority**: P1
**Effort**: 3-4 hours

**Features**:
- Generate sitemap at `/sitemap.xml`
- Include all posts and pages
- Add last modified dates
- Add change frequency hints
- Respect draft status

**Implementation**:
- Create sitemap generation module
- Add route handler
- Include in robots.txt
- Consider `sitemap` crate or build manually

**Testing**:
- Validate with https://www.xml-sitemaps.com/validate-xml-sitemap.html
- Submit to Google Search Console

**Success Criteria**:
- Sitemap includes all published content
- Updates automatically with content changes
- Validates against sitemap spec

## Phase 4: Tag and Category Archives (P1)

### 4.1 Tag Archive Pages
**Priority**: P1
**Effort**: 6-8 hours

**Features**:
- Route: `/tags/:tag`
- List all posts with specific tag
- Show tag cloud on main blog listing
- Pagination support
- Tag counts

**Implementation**:
- Add tag index building in `blog-content`
- Create tag archive route handler
- Create tag archive template
- Create tag cloud partial
- Add tests for tag filtering

### 4.2 Category Archive Pages
**Priority**: P2
**Effort**: 4-5 hours

- Route: `/categories/:category`
- Similar to tags but single category per post
- Category listing page
- Breadcrumb navigation

**Success Criteria**:
- Tag pages show correct posts
- Tag cloud displays all tags with counts
- Category pages work similarly
- All pages have proper pagination

## Phase 5: SEO Enhancements (P1)

### 5.1 Meta Tags
**Priority**: P1
**Effort**: 3-4 hours

**Features**:
- OpenGraph meta tags for social sharing
- Twitter Card meta tags
- Canonical URLs
- Meta descriptions for all pages
- Structured data (JSON-LD for blog posts)

**Implementation**:
- Add meta tag partial template
- Include in base.html
- Add to all page types
- Test with social media validators

### 5.2 robots.txt
**Priority**: P1
**Effort**: 1 hour

- Create `/robots.txt` route
- Include sitemap reference
- Configurable crawl rules

**Testing**:
- Verify with Google Search Console
- Test with https://en.ryte.com/free-tools/robots-txt/

**Success Criteria**:
- OpenGraph preview works on Twitter, Facebook, LinkedIn
- Google Rich Results Test passes
- robots.txt accessible and valid

## Phase 6: Search Functionality (P2)

### 6.1 Client-Side Search
**Priority**: P2
**Effort**: 8-10 hours

**Approach**: Simple client-side search to maintain SSR architecture

**Features**:
- Search box in navigation
- Real-time search results
- Search across post titles, descriptions, content
- Highlight search terms

**Implementation Options**:
1. **Static JSON index** (Recommended)
   - Generate search index at build time
   - Load JSON on demand
   - Use lightweight JS library (Fuse.js, lunr.js)
   - Keep it under 100KB

2. **Server-side search endpoint**
   - `/api/search?q=term`
   - Search in-memory or use grep
   - Return JSON results
   - HTMX integration

**Success Criteria**:
- Search returns relevant results
- Fast response time (< 100ms)
- Mobile-friendly search UI
- Works without JavaScript (fallback to Google)

## Phase 7: Content Enhancements (P2)

### 7.1 Related Posts
**Priority**: P2
**Effort**: 4-5 hours

**Features**:
- Show 3-5 related posts at bottom of post
- Based on shared tags/categories
- Exclude current post
- Configurable count

**Implementation**:
- Add related posts function in blog-content
- Update post template
- Simple scoring algorithm (count shared tags)

### 7.2 Reading Time Estimation
**Priority**: P3
**Effort**: 2 hours

- Calculate reading time (words / 200 wpm)
- Display in post header
- Add to post frontmatter (optional override)

### 7.3 Series/Collection Support
**Priority**: P3
**Effort**: 6-8 hours

**Features**:
- Group related posts into series
- Series navigation (next/previous in series)
- Series index page
- Frontmatter: `series: "Getting Started with Rust"`

**Success Criteria**:
- Related posts show relevant content
- Reading time is accurate
- Series navigation works correctly

## Phase 8: Image Optimization (P2)

### 8.1 Responsive Images
**Priority**: P2
**Effort**: 6-8 hours

**Features**:
- Generate multiple image sizes
- Use `srcset` for responsive images
- WebP format support (with fallback)
- Lazy loading

**Implementation**:
- Image processing at build time
- Use `image` crate
- Generate thumbnails and optimized versions
- Add to build pipeline

### 8.2 Image CDN Integration (Optional)
**Priority**: P3
**Effort**: 4-5 hours

- Support external image hosting (Cloudflare Images, imgix)
- Configuration for CDN URLs
- Fallback to local images

**Success Criteria**:
- Images load faster
- Proper sizing for different screens
- WebP used where supported
- Core Web Vitals improved

## Phase 9: Analytics and Monitoring (P2)

### 9.1 Privacy-Friendly Analytics
**Priority**: P2
**Effort**: 3-4 hours

**Options**:
- Plausible Analytics (recommended - privacy-friendly)
- GoatCounter (open source)
- Self-hosted Umami
- Server-side request logging

**Implementation**:
- Add analytics script to base template
- Configurable via environment variable
- Respect Do Not Track
- No cookies required

### 9.2 Health Check Enhancements
**Priority**: P2
**Effort**: 2-3 hours

- Detailed health endpoint with metrics
- Content count stats
- Memory usage
- Uptime
- Version info

**Success Criteria**:
- Analytics data is collected
- Privacy compliant (no PII)
- Health checks provide useful info

## Phase 10: Developer Experience (P2)

### 10.1 CI/CD Setup
**Priority**: P2
**Effort**: 4-6 hours

**Features**:
- GitHub Actions workflow
- Run tests on PR
- Build check
- Clippy linting
- Format checking
- Nix build test

### 10.2 Content Preview
**Priority**: P3
**Effort**: 4-5 hours

- Draft preview endpoint
- Hot reload for content changes
- Preview URL for sharing drafts

### 10.3 Admin Scripts
**Priority**: P3
**Effort**: 3-4 hours

- Script to create new post with template
- Slug generation helper
- Image optimization script
- Content validation script

**Success Criteria**:
- CI runs on every PR
- Tests prevent regressions
- Content creation is easier

## Phase 11: Comment System (P3)

### 11.1 Third-Party Integration
**Priority**: P3
**Effort**: 2-3 hours

**Options**:
- Giscus (GitHub Discussions)
- Utterances (GitHub Issues)
- Commento
- No comments (email/social media only)

**Implementation**:
- Configurable per-post (`comments: true`)
- Add script to post template
- Privacy considerations

**Success Criteria**:
- Comments work on posts
- Easy to moderate
- Privacy-friendly

## Non-Goals for V2

The following remain out of scope:

- User authentication/accounts
- Admin dashboard/CMS
- Multi-language support
- Multiple authors with profiles
- Newsletter integration
- Markdown extensions (beyond standard)

## Implementation Order (Recommended)

1. **Phase 1**: Bug fixes (required before anything else)
2. **Phase 2**: RSS feeds (high value, easy to implement)
3. **Phase 3**: Sitemap (SEO critical)
4. **Phase 5**: SEO enhancements (high value)
5. **Phase 4**: Tag/category archives (improve navigation)
6. **Phase 7**: Content enhancements (related posts, reading time)
7. **Phase 6**: Search (nice to have, more complex)
8. **Phase 8**: Image optimization (performance)
9. **Phase 9**: Analytics (monitoring)
10. **Phase 10**: CI/CD (developer productivity)
11. **Phase 11**: Comments (optional, low priority)

## Success Metrics

- **SEO**: Google Search Console shows improvement in impressions/clicks
- **Performance**: Lighthouse score > 90
- **Discoverability**: RSS subscribers, search engine traffic
- **Content**: Tag pages help readers find related content
- **Stability**: CI catches issues before deployment

## Migration Notes

- V2 should be backward compatible with V1 content
- New features should be opt-in via configuration
- No breaking changes to content format
- Migration guide for any new features

## Timeline Estimate

- **Phase 1**: 1-2 hours (critical)
- **Quick Wins** (Phases 2, 3, 5): 10-15 hours
- **Navigation** (Phase 4): 10-13 hours
- **Advanced Features** (Phases 6, 7, 8): 20-28 hours
- **Polish** (Phases 9, 10, 11): 12-18 hours

**Total**: 53-76 hours of development time

Suggested approach: Tackle phases 1-5 first (the "Core V2"), then evaluate which advanced features are worth implementing based on actual usage.

---

**Next Steps**: Review this plan, prioritize phases, and begin implementation with Phase 1 (bug fixes).
