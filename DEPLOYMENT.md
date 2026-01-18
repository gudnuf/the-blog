# Deployment Guide - Blog Caching System

This guide covers deployment and maintenance of the blog's post caching system with SIGHUP reload and automated GitHub deployments.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Prerequisites](#prerequisites)
- [Initial Setup](#initial-setup)
  - [1. GitHub Secrets Configuration](#1-github-secrets-configuration)
  - [2. Server SSH Setup](#2-server-ssh-setup)
  - [3. NixOS Configuration Integration](#3-nixos-configuration-integration)
  - [4. Deploy Initial Code](#4-deploy-initial-code)
- [Deployment Workflows](#deployment-workflows)
  - [Content Deployments](#content-deployments)
  - [Code Deployments](#code-deployments)
- [Maintenance](#maintenance)
- [Troubleshooting](#troubleshooting)
- [Performance Monitoring](#performance-monitoring)

---

## Architecture Overview

**Caching System:**
- Posts loaded into memory (`Arc<RwLock<Vec<Post>>>`) at startup
- SIGHUP signal reloads cache without restarting service
- Eliminates disk I/O on every request (100x faster)

**Deployment Paths:**

```
Content Changes (content/**/*.md)
  ├─> GitHub Actions triggers
  ├─> rsync to server
  └─> SIGHUP reload (seconds, zero-downtime)

Code Changes (crates/**/*.rs)
  ├─> Push to blog repo
  ├─> Update nix-config flake
  └─> nixos-rebuild (minutes, service restart)
```

---

## Prerequisites

- NixOS server (noosphere) with SSH access
- GitHub repository for blog
- `~/.config/nix-config` with NixOS configuration
- SSH key pair for GitHub Actions

---

## Initial Setup

### 1. GitHub Secrets Configuration

Add secrets in GitHub repository settings (`Settings > Secrets and variables > Actions`):

| Secret Name | Description | Example Value |
|-------------|-------------|---------------|
| `DEPLOY_SSH_KEY` | Private SSH key (Ed25519) | `-----BEGIN OPENSSH PRIVATE KEY-----\n...` |
| `DEPLOY_HOST` | Server hostname or IP | `noosphere` or `192.168.1.100` |
| `DEPLOY_USER` | SSH username | `rust-blog` |
| `DEPLOY_CONTENT_PATH` | Remote content directory | `/var/lib/rust-blog/content` |

**Generate SSH key for GitHub Actions:**

```bash
# On your local machine
ssh-keygen -t ed25519 -C "github-actions-blog-deploy" -f ~/.ssh/blog_deploy_key

# Copy private key for DEPLOY_SSH_KEY secret
cat ~/.ssh/blog_deploy_key

# Save public key for server setup (next step)
cat ~/.ssh/blog_deploy_key.pub
```

### 2. Server SSH Setup

Configure SSH access for the `rust-blog` user on your server:

```bash
# SSH into your server
ssh noosphere

# Create SSH directory for rust-blog user
sudo mkdir -p /var/lib/rust-blog/.ssh
sudo chown rust-blog:rust-blog /var/lib/rust-blog/.ssh
sudo chmod 700 /var/lib/rust-blog/.ssh

# Add GitHub Actions public key to authorized_keys
# Replace the key below with your actual public key from step 1
sudo -u rust-blog sh -c 'cat >> /var/lib/rust-blog/.ssh/authorized_keys' <<EOF
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAA... github-actions-blog-deploy
EOF

sudo chmod 600 /var/lib/rust-blog/.ssh/authorized_keys
sudo chown rust-blog:rust-blog /var/lib/rust-blog/.ssh/authorized_keys

# Test SSH access (from local machine)
ssh -i ~/.ssh/blog_deploy_key rust-blog@noosphere 'echo "SSH access confirmed"'
```

**Add sudo permissions for reload:**

On the server, add sudo rule for systemctl reload:

```bash
# Add to /etc/nixos/configuration.nix or nix-config module
security.sudo.extraRules = [{
  users = [ "rust-blog" ];
  commands = [{
    command = "/run/current-system/sw/bin/systemctl reload rust-blog.service";
    options = [ "NOPASSWD" ];
  }];
}];

# Apply configuration
sudo nixos-rebuild switch
```

### 3. NixOS Configuration Integration

Integrate the blog into your `~/.config/nix-config`:

**Step A: Add blog input to flake**

Edit `~/.config/nix-config/flake.nix`:

```nix
{
  inputs = {
    # ... existing inputs (nixpkgs, home-manager, etc.) ...

    blog = {
      url = "github:yourusername/blog";  # Replace with your repo
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  # In outputs, ensure blog is passed through inputs
  # No changes needed if using specialArgs = { inherit inputs; }
}
```

**Step B: Create blog module**

Create `~/.config/nix-config/modules/nixos/blog.nix`:

```nix
{ inputs, config, pkgs, ... }:

{
  imports = [ inputs.blog.nixosModules.default ];

  services.rust-blog = {
    enable = true;
    package = inputs.blog.packages.x86_64-linux.blog-server;
    host = "127.0.0.1";
    port = 3000;
    contentPath = "/var/lib/rust-blog/content";
    enableDrafts = false;
    postsPerPage = 10;
    logLevel = "info";
  };

  # Sudo permission for cache reload
  security.sudo.extraRules = [{
    users = [ "rust-blog" ];
    commands = [{
      command = "/run/current-system/sw/bin/systemctl reload rust-blog.service";
      options = [ "NOPASSWD" ];
    }];
  }];

  # Optional: Reverse proxy with Caddy
  services.caddy = {
    enable = true;
    virtualHosts."blog.yourdomain.com".extraConfig = ''
      reverse_proxy localhost:3000
    '';
  };

  # Open firewall
  networking.firewall.allowedTCPPorts = [ 80 443 ];
}
```

**Step C: Import module**

Edit `~/.config/nix-config/modules/nixos/default.nix`:

```nix
{
  imports = [
    ./ssh.nix
    ./mosh.nix
    ./firewall.nix
    ./vm-guest.nix
    ./blog.nix  # Add this line
  ];
}
```

### 4. Deploy Initial Code

Deploy the blog service to your server:

```bash
# From ~/.config/nix-config
cd ~/.config/nix-config

# Update flake inputs
nix flake update blog

# Commit the flake.lock
git add flake.nix flake.lock modules/nixos/blog.nix modules/nixos/default.nix
git commit -m "Add blog service with caching"

# Deploy to server
nixos-rebuild switch --flake .#noosphere --target-host noosphere --use-remote-sudo

# Verify service is running
ssh noosphere systemctl status rust-blog.service
```

**Verify deployment:**

```bash
# Check service status
ssh noosphere systemctl status rust-blog.service

# Check logs
ssh noosphere journalctl -u rust-blog.service -f

# Test HTTP endpoint
curl http://noosphere:3000/health
# Should return: {"status":"healthy"}

# Test post cache loaded
ssh noosphere journalctl -u rust-blog.service | grep "Loaded.*posts into cache"
```

---

## Deployment Workflows

### Content Deployments

**Automatic (via GitHub Actions):**

1. Edit content files: `content/posts/YYYY-MM-DD-slug.md`
2. Commit and push to GitHub:
   ```bash
   git add content/
   git commit -m "Add new blog post"
   git push origin main
   ```
3. GitHub Actions automatically:
   - Syncs content to server via rsync
   - Sends SIGHUP to reload cache
   - Completes in seconds

**Manual deployment:**

```bash
# Set environment variables
export DEPLOY_HOST="noosphere"
export DEPLOY_USER="rust-blog"
export DEPLOY_CONTENT_PATH="/var/lib/rust-blog/content"

# Run deployment script
./scripts/deploy-content.sh
```

**Verify content deployment:**

```bash
# Check logs for reload
ssh noosphere journalctl -u rust-blog.service -n 20

# Should see:
# "SIGHUP received, reloading post cache"
# "Loaded N posts into cache"
# "Post cache reloaded successfully"

# Visit new post
curl http://noosphere:3000/posts/your-new-post-slug
```

### Code Deployments

When you modify Rust code in `crates/`:

**Step 1: Push changes to blog repo**

```bash
# In blog repository
git add crates/
git commit -m "Implement new feature"
git push origin main
```

**Step 2: Update blog in nix-config**

```bash
# In ~/.config/nix-config
cd ~/.config/nix-config

# Update blog input to latest commit
nix flake lock --update-input blog

# Commit the lock file
git add flake.lock
git commit -m "Update blog to $(cd ~/blog && git rev-parse --short HEAD)"

# Deploy to server
nixos-rebuild switch --flake .#noosphere --target-host noosphere --use-remote-sudo
```

**Step 3: Verify deployment**

```bash
# Check service restarted successfully
ssh noosphere systemctl status rust-blog.service

# Check logs for startup
ssh noosphere journalctl -u rust-blog.service -n 50

# Verify new functionality works
curl http://noosphere:3000/your-new-endpoint
```

---

## Maintenance

### Reloading Post Cache Manually

```bash
# Via systemctl (recommended)
ssh noosphere sudo systemctl reload rust-blog.service

# Or send SIGHUP directly
ssh noosphere sudo pkill -HUP blog-server
```

### Checking Cache Status

```bash
# View logs for cache reloads
ssh noosphere journalctl -u rust-blog.service | grep -E "Loaded|reload"

# Check current service status
ssh noosphere systemctl status rust-blog.service

# View recent logs
ssh noosphere journalctl -u rust-blog.service -n 100
```

### Restarting Service

```bash
# Full restart (clears cache and reloads from disk)
ssh noosphere sudo systemctl restart rust-blog.service

# Check status
ssh noosphere systemctl status rust-blog.service
```

### Viewing Logs

```bash
# Follow logs in real-time
ssh noosphere journalctl -u rust-blog.service -f

# Last 100 lines
ssh noosphere journalctl -u rust-blog.service -n 100

# Logs from last hour
ssh noosphere journalctl -u rust-blog.service --since "1 hour ago"

# Logs with specific pattern
ssh noosphere journalctl -u rust-blog.service | grep -i error
```

### Backing Up Content

```bash
# Sync content from server to local backup
rsync -avz rust-blog@noosphere:/var/lib/rust-blog/content/ ./backup/content-$(date +%Y%m%d)/

# Or create archive on server
ssh noosphere "cd /var/lib/rust-blog && tar czf content-backup-$(date +%Y%m%d).tar.gz content/"
```

### Updating Dependencies

```bash
# In blog repository
cargo update

# Test locally
cargo test
cargo run

# Commit and deploy (see Code Deployments above)
git add Cargo.lock
git commit -m "Update dependencies"
git push origin main
```

---

## Troubleshooting

### GitHub Actions Deployment Fails

**SSH Connection Failure:**

```
ERROR: Cannot connect to noosphere
```

**Solution:**
```bash
# Verify secrets are set correctly
# Check DEPLOY_HOST, DEPLOY_USER, DEPLOY_SSH_KEY in GitHub secrets

# Test SSH key locally
ssh -i ~/.ssh/blog_deploy_key rust-blog@noosphere

# Verify authorized_keys on server
ssh noosphere cat /var/lib/rust-blog/.ssh/authorized_keys
```

**Permission Denied on systemctl reload:**

```
sudo: systemctl reload rust-blog.service: command not found
```

**Solution:**
```bash
# Check sudo rules on server
ssh noosphere sudo -l -U rust-blog

# Should show:
# (root) NOPASSWD: /run/current-system/sw/bin/systemctl reload rust-blog.service

# If missing, add sudo rule (see Initial Setup step 2)
```

### Cache Not Reloading

**SIGHUP sent but cache not updating:**

```bash
# Check if SIGHUP handler is running (Unix only)
ssh noosphere journalctl -u rust-blog.service | grep -i sighup

# Should NOT see: "SIGHUP handler not available on non-Unix systems"

# Check for errors during reload
ssh noosphere journalctl -u rust-blog.service | grep -i "failed to reload"

# If you see errors, check content directory permissions
ssh noosphere ls -la /var/lib/rust-blog/content/
```

**Solution:**
```bash
# Restart service to reinitialize SIGHUP handler
ssh noosphere sudo systemctl restart rust-blog.service

# Verify handler started
ssh noosphere journalctl -u rust-blog.service | tail -20
```

### Service Won't Start

**Check service status:**

```bash
ssh noosphere systemctl status rust-blog.service

# If failed, check logs
ssh noosphere journalctl -u rust-blog.service -n 50
```

**Common issues:**

1. **Port already in use:**
   ```
   Error: Address already in use (os error 48)
   ```
   Solution: Change port in nix-config or stop conflicting service

2. **Content directory missing:**
   ```
   Failed to load posts: No such file or directory
   ```
   Solution:
   ```bash
   ssh noosphere sudo mkdir -p /var/lib/rust-blog/content/{posts,pages,images}
   ssh noosphere sudo chown -R rust-blog:rust-blog /var/lib/rust-blog/content
   ```

3. **Template or static path issues:**
   ```
   Failed to load templates
   ```
   Solution: Verify package paths in module configuration

### Posts Not Appearing

**New post not visible after deployment:**

```bash
# Check if file synced to server
ssh noosphere ls -la /var/lib/rust-blog/content/posts/

# Check if cache reloaded
ssh noosphere journalctl -u rust-blog.service | grep "Post cache reloaded successfully"

# Manually reload cache
ssh noosphere sudo systemctl reload rust-blog.service

# Check post count in logs
ssh noosphere journalctl -u rust-blog.service | grep "Loaded.*posts into cache"
```

**Post has invalid frontmatter:**

```bash
# Check for parsing errors in logs
ssh noosphere journalctl -u rust-blog.service | grep -i error

# Validate frontmatter locally
cargo run --bin blog-server
# Check logs for parsing errors
```

### Performance Issues

**High memory usage:**

```bash
# Check memory usage
ssh noosphere systemctl status rust-blog.service | grep Memory

# Expected: ~5KB per post
# For 100 posts: ~500KB - 1MB
# If much higher, investigate memory leak
```

**Slow response times:**

```bash
# Test response time
time curl -s http://noosphere:3000/ > /dev/null

# Should be <100ms for cached content
# If slower, check:
# 1. Network latency
# 2. Template rendering (not cached)
# 3. Reverse proxy overhead
```

---

## Performance Monitoring

### Key Metrics

**Cache statistics:**
```bash
# Post count
ssh noosphere journalctl -u rust-blog.service | grep "Loaded.*posts into cache" | tail -1

# Reload time
ssh noosphere journalctl -u rust-blog.service | grep -E "SIGHUP received|Post cache reloaded"

# Expected: <100ms for reload
```

**Request performance:**
```bash
# Test endpoint response time
time curl -s http://noosphere:3000/posts/test-slug > /dev/null

# Expected with cache:
# - First request (template render): 5-20ms
# - Subsequent requests: <10ms
```

**Memory usage:**
```bash
# Service memory
ssh noosphere systemctl status rust-blog.service | grep Memory

# Process details
ssh noosphere ps aux | grep blog-server
```

### Expected Performance

| Metric | Before Cache | With Cache | Improvement |
|--------|--------------|------------|-------------|
| Request latency | ~5ms | ~50μs | 100x faster |
| Memory usage | Minimal | ~5KB/post | Negligible |
| Cache reload | N/A | ~100ms | Non-blocking |
| Content deploy | Full rebuild | Seconds | 100x faster |

---

## Additional Resources

**Related Files:**
- `CLAUDE.md` - Project architecture and development guide
- `scripts/deploy-content.sh` - Content deployment script
- `.github/workflows/deploy-content.yml` - Automated content deployment
- `.github/workflows/deploy-code.yml` - Code deployment notifications
- `nixos/module.nix` - NixOS service module

**NixOS Commands:**
```bash
# Show service options
nixos-option services.rust-blog

# View current configuration
nixos-rebuild --flake .#noosphere build

# Dry run deployment
nixos-rebuild --flake .#noosphere dry-activate

# Rollback to previous generation
ssh noosphere nixos-rebuild --rollback switch
```

**Useful Monitoring:**
```bash
# Watch logs in real-time
ssh noosphere journalctl -u rust-blog.service -f

# Monitor system resources
ssh noosphere htop -p $(ssh noosphere pgrep blog-server)

# Check network connections
ssh noosphere ss -tulpn | grep 3000
```

---

## Quick Reference

### Common Commands

```bash
# Deploy content changes
git add content/ && git commit -m "Update content" && git push

# Deploy code changes
cd ~/.config/nix-config && nix flake lock --update-input blog && \
  nixos-rebuild switch --flake .#noosphere --target-host noosphere --use-remote-sudo

# Manual content deployment
DEPLOY_HOST=noosphere ./scripts/deploy-content.sh

# Reload cache
ssh noosphere sudo systemctl reload rust-blog.service

# Restart service
ssh noosphere sudo systemctl restart rust-blog.service

# View logs
ssh noosphere journalctl -u rust-blog.service -f

# Check status
ssh noosphere systemctl status rust-blog.service
```

### Emergency Procedures

**Service is down:**
```bash
# Check status
ssh noosphere systemctl status rust-blog.service

# Restart
ssh noosphere sudo systemctl restart rust-blog.service

# If still failing, rollback NixOS generation
ssh noosphere nixos-rebuild --rollback switch
```

**Corrupted cache:**
```bash
# Restart service (reloads from disk)
ssh noosphere sudo systemctl restart rust-blog.service

# If content is corrupted, restore from backup
rsync -avz ./backup/content-YYYYMMDD/ rust-blog@noosphere:/var/lib/rust-blog/content/
ssh noosphere sudo systemctl reload rust-blog.service
```

**GitHub Actions stuck:**
```bash
# Cancel running workflow in GitHub UI
# Or force push to trigger new deployment
git commit --allow-empty -m "Trigger deployment"
git push origin main
```

---

## Changelog

- **2026-01-17**: Initial deployment guide created with caching system
