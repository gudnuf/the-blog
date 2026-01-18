# Vision: Rust SSR Blog

**Last Updated**: 2025-01-17
**Status**: Active Development

## Purpose

This document articulates the big picture vision for the Rust SSR Blog project - what we're building, why it matters, and where we're headed.

## The Problem

Modern blogging platforms face several challenges:

1. **Complexity Overhead**: Most platforms (WordPress, Ghost, Medium) require databases, complex infrastructure, and ongoing maintenance
2. **Performance**: JavaScript-heavy SPAs slow down initial page loads and hurt SEO
3. **Vendor Lock-in**: Proprietary platforms control your content and can change terms at any time
4. **Privacy Concerns**: Many platforms track readers extensively
5. **Cost**: Hosting, databases, and managed services add up
6. **Security**: Attack surface increases with complexity (databases, admin panels, plugins)

## The Vision

**Build the simplest, fastest, most maintainable blog platform that respects both writers and readers.**

A blog should be:
- **Fast**: Render pages in milliseconds, not seconds
- **Simple**: Write in markdown, deploy with one command
- **Private**: No tracking, no ads, no compromises
- **Yours**: Own your content, host anywhere
- **Secure**: Minimal attack surface, no database to compromise
- **Beautiful**: Clean design that focuses on content
- **Permanent**: Built to last decades, not quarters

## Core Philosophy

### 1. Content is Files

Your blog content should be:
- Plain text markdown files
- Version controlled with git
- Portable across any platform
- Editable with any text editor
- Backed up with your code

**No database means**: No backups to restore, no migrations to run, no data to export. Your content is already in its most portable format.

### 2. Server-Side Rendering

HTML should be rendered on the server for:
- **Speed**: No JavaScript bundle to download, parse, and execute
- **SEO**: Search engines get complete HTML immediately
- **Accessibility**: Works without JavaScript
- **Reliability**: No hydration failures or flash of unstyled content

JavaScript is for enhancement, not requirements.

### 3. Simplicity as a Feature

Every feature has a cost:
- Code to write and maintain
- Bugs to fix
- Documentation to update
- Cognitive load for users

We only add features that earn their cost. Simplicity is not the absence of features - it's the absence of unnecessary features.

### 4. Rust for Reliability

Rust provides:
- **Memory safety**: No segfaults, no buffer overflows
- **Performance**: Native speed, small binary size
- **Type safety**: Catch bugs at compile time
- **Concurrency**: Safe parallelism without data races
- **Ecosystem**: Excellent libraries for web development

The blog should be rock-solid, not require restarts, and handle traffic spikes gracefully.

### 5. Nix for Reproducibility

Development and deployment should be:
- **Reproducible**: Same result every time
- **Declarative**: Describe what you want, not how to build it
- **Isolated**: No "works on my machine" problems
- **Composable**: Build on solid foundations

## What We're Building

### Short Term (V1 - Completed)

A production-ready blog platform with:
- Markdown content with frontmatter
- Syntax-highlighted code blocks
- Responsive design
- Tag and category support
- Table of contents generation
- Draft mode
- NixOS deployment module

**Status**: ✅ Complete and deployable

### Medium Term (V2 - Next 6 Months)

Enhance discoverability and SEO:
- RSS/Atom feeds for subscribers
- XML sitemaps for search engines
- Tag and category archive pages
- Basic search functionality
- Better meta tags and OpenGraph support
- Related posts suggestions
- CI/CD for quality assurance

**Goal**: Make the blog discoverable and useful for readers.

### Long Term (V3+ - Beyond 6 Months)

Expand capabilities while maintaining simplicity:
- Advanced search with relevance ranking
- Series/collection support for multi-part posts
- Image optimization and responsive images
- Internationalization (i18n) support
- Email newsletter integration (optional)
- WebSub for real-time feed updates
- AMP or similar mobile optimizations
- Reader analytics (privacy-preserving)

**Goal**: Best-in-class blogging experience while staying true to core principles.

## Who This Is For

### Primary Audience: Technical Writers

Developers, engineers, and technical writers who:
- Want to own their platform
- Value performance and simplicity
- Write in markdown naturally
- Use git for version control
- Deploy to their own infrastructure

### Secondary Audience: Bloggers Seeking Simplicity

Non-technical bloggers who:
- Want fast, reliable hosting
- Don't need dynamic features
- Value privacy and ownership
- Work with developers for setup
- Focus on writing, not maintenance

## What This Is NOT

This project explicitly rejects:

1. **A WordPress Replacement**: WordPress is great for certain use cases. This is not trying to be WordPress.

2. **A CMS**: No admin dashboard, no visual editor, no content management UI. Content management happens in your text editor and git.

3. **A Social Platform**: No user accounts, no comments system (use external solutions), no social features.

4. **An Everything Platform**: Not for e-commerce, not for forums, not for wikis. Focused on blogging only.

5. **JavaScript-First**: This is a server-rendered platform. JavaScript is for enhancement only.

## Success Criteria

We'll know we've succeeded when:

1. **Performance**:
   - Lighthouse score > 95
   - First Contentful Paint < 500ms
   - Time to Interactive < 1s

2. **Developer Experience**:
   - New post in under 5 minutes
   - Deploy with single command
   - Zero-config development environment

3. **Reliability**:
   - Uptime > 99.9%
   - No memory leaks
   - Handles 10K req/s on modest hardware

4. **Simplicity**:
   - Core codebase under 5K LOC
   - New developer productive in 1 day
   - Documentation that's actually read

5. **Adoption**:
   - 100+ blogs running in production
   - Active community contributions
   - Positive feedback from users

## Design Principles

### 1. Boring Technology

Use proven, stable technologies. Prefer 10-year-old solutions to cutting-edge frameworks. Boring is reliable.

### 2. Explicit Over Implicit

Configuration should be obvious. No magic, no conventions that require tribal knowledge. If it's not in the docs, it shouldn't exist.

### 3. Fail Loudly

Errors should be clear and actionable. A cryptic error that fails fast is better than silent corruption.

### 4. Convention with Escape Hatches

Sensible defaults for 90% of use cases. Clear ways to customize for the other 10%. No fighting the framework.

### 5. Documentation is Code

If it's not documented, it's not done. Documentation is not an afterthought - it's part of the feature.

## Technical Vision

### Architecture

```
┌─────────────────────────────────────────┐
│           Rust Binary                    │
│  ┌─────────────────────────────────┐    │
│  │     Axum Web Server             │    │
│  ├─────────────────────────────────┤    │
│  │  Routes  │  Templates  │ Config │    │
│  └─────────────────────────────────┘    │
│  ┌─────────────────────────────────┐    │
│  │   Blog Content Library          │    │
│  ├─────────────────────────────────┤    │
│  │ Parser │ Highlighter │ TOC Gen  │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
                  ↓
         ┌────────────────┐
         │  Content Files │
         │   (Markdown)   │
         └────────────────┘
```

**Key Characteristics**:
- Single statically-linked binary
- No runtime dependencies
- No database
- No Node.js build step (Tailwind via standalone CLI)
- Stateless (can run multiple instances)

### Deployment Models

1. **NixOS Module** (Primary)
   - Declarative configuration
   - Security hardening built-in
   - Automatic updates via Nix
   - Works with standard NixOS tools

2. **Docker** (Secondary)
   - For non-NixOS environments
   - Lightweight container
   - Easy deployment to cloud platforms

3. **Static Binary** (Tertiary)
   - Download and run
   - No dependencies
   - Works on any Linux/macOS system

### Scaling Strategy

**Vertical First**: Optimize for single-server performance
- Rust is fast enough to handle most blogs on modest hardware
- Focus on efficiency over distributed complexity
- Add caching if needed (not required for most blogs)

**Horizontal Later**: Support multi-instance if needed
- Stateless design means easy horizontal scaling
- Load balancer + N instances
- Shared content via NFS or content CDN

## Ecosystem Integration

### Compatible With

- **Static Site Generators**: Can export to static HTML
- **Git Workflows**: Content managed via git
- **CI/CD**: Easy integration with GitHub Actions, GitLab CI
- **Monitoring**: Prometheus metrics endpoint
- **CDNs**: Works behind Cloudflare, Fastly, etc.
- **Comment Systems**: Giscus, Utterances, Commento

### Intentionally Incompatible With

- Database-dependent plugins
- PHP modules
- WordPress themes
- JavaScript-required admin panels

## Community Values

### Open Source

- MIT or Apache-2.0 dual license
- All development in public
- Welcoming to contributors
- Clear contribution guidelines

### Sustainability

- No venture capital
- No growth-at-all-costs
- No pivot to SaaS
- Focus on long-term maintainability

### Privacy

- No tracking by default
- No data collection
- No phone-home features
- Privacy-friendly analytics (optional)

### Accessibility

- WCAG 2.1 AA compliance
- Semantic HTML
- Screen reader friendly
- Keyboard navigation

## Roadmap Themes

### 2025: Foundation

- V1: Core platform (✅ Complete)
- V2: Discoverability and SEO
- Establish community
- Build documentation
- Create showcase of live blogs

### 2026: Polish

- V3: Advanced features
- Performance optimizations
- Comprehensive testing
- Plugin system (maybe)
- Multi-language support

### 2027: Maturity

- Stable 1.0 release
- Proven in production at scale
- Comprehensive ecosystem
- Book/guide publication
- Conference talks

## Risks and Mitigations

### Risk: Rust Learning Curve

**Mitigation**:
- Excellent documentation
- Example customizations
- Active community support
- Content creation doesn't require Rust knowledge

### Risk: Limited Plugins

**Mitigation**:
- Core functionality is comprehensive
- Integration with external services
- Extensibility through configuration
- Future plugin system if needed

### Risk: Nix Complexity

**Mitigation**:
- Docker alternative
- Static binary distribution
- Nix-free development option
- Clear Nix documentation

### Risk: Feature Requests

**Mitigation**:
- Clear vision document (this!)
- Say no to complexity
- Focus on core use case
- Encourage forks for divergent needs

## Measuring Success

### Quantitative

- GitHub stars (community interest)
- Production deployments (real usage)
- Performance benchmarks (speed)
- Test coverage (quality)
- Documentation completeness (usability)

### Qualitative

- User testimonials
- Ease of onboarding
- Code maintainability
- Community health
- Long-term viability

## Call to Action

This vision guides every decision we make. When in doubt:

1. **Does it serve writers?** Making content creation easier is always good.
2. **Does it serve readers?** Faster, more accessible content is always good.
3. **Does it add complexity?** If yes, the benefit must be substantial.
4. **Can it be simpler?** If yes, make it simpler.
5. **Will it matter in 5 years?** Focus on lasting value.

## Conclusion

We're building a blog platform that:
- **Writers love** because it's simple and focuses on content
- **Readers love** because it's fast and respects privacy
- **Developers love** because it's maintainable and reliable
- **Ops teams love** because it's easy to deploy and monitor

Not by adding features, but by ruthlessly focusing on what matters.

**Simple. Fast. Yours.**

---

For implementation details, see:
- [PLAN-V1-COMPLETED.md](./PLAN-V1-COMPLETED.md) - What we've built
- [PLAN-V2-NEXT.md](./PLAN-V2-NEXT.md) - What we're building next
- [CLAUDE.md](../CLAUDE.md) - Technical project context
