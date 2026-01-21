# Blog Deployment

Deploy the blog to the Hetzner server (77.42.27.244).

## Quick Deploy

After committing changes to `master`:

```bash
cd ~/noosphere-nix
nix flake update the-blog
sudo nixos-rebuild switch --flake .#hetzner
```

## What This Does

1. Updates `flake.lock` to point to latest blog commit
2. Rebuilds blog-server package (compiles Rust + Tailwind CSS)
3. Restarts `rust-blog.service` with new binary

## Verify Deployment

```bash
# Check service status
systemctl status rust-blog

# Test locally
curl -I http://localhost:3311/

# Test public
curl -I http://77.42.27.244/
```

## Content-Only Updates

For markdown changes without code changes (faster, no rebuild):

```bash
# Sync content to server
rsync -avz content/ /var/lib/rust-blog/content/

# Reload cache (zero-downtime)
sudo systemctl reload rust-blog
```

## Architecture

```
nginx (:80) → rust-blog (:3311) → Nix package
```

## Troubleshooting

```bash
# View logs
journalctl -u rust-blog -f

# Restart service
sudo systemctl restart rust-blog

# Rollback to previous generation
sudo nixos-rebuild switch --rollback
```
