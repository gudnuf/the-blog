{ config, lib, pkgs, ... }:

let
  cfg = config.services.rust-blog;
in
{
  options.services.rust-blog = {
    enable = lib.mkEnableOption "Rust SSR Blog server";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.blog-server or (throw "blog-server package not found");
      description = "The blog-server package to use";
    };

    host = lib.mkOption {
      type = lib.types.str;
      default = "127.0.0.1";
      description = "Host address to bind to";
    };

    port = lib.mkOption {
      type = lib.types.port;
      default = 3000;
      description = "Port to listen on";
    };

    contentPath = lib.mkOption {
      type = lib.types.path;
      default = "/var/lib/rust-blog/content";
      description = "Path to blog content directory";
    };

    templatesPath = lib.mkOption {
      type = lib.types.nullOr lib.types.path;
      default = null;
      description = "Path to templates directory (defaults to package templates)";
    };

    staticPath = lib.mkOption {
      type = lib.types.nullOr lib.types.path;
      default = null;
      description = "Path to static assets directory (defaults to package static)";
    };

    postsPerPage = lib.mkOption {
      type = lib.types.int;
      default = 10;
      description = "Number of posts to show per page";
    };

    enableDrafts = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = "Whether to show draft posts";
    };

    user = lib.mkOption {
      type = lib.types.str;
      default = "rust-blog";
      description = "User to run the service as";
    };

    group = lib.mkOption {
      type = lib.types.str;
      default = "rust-blog";
      description = "Group to run the service as";
    };

    logLevel = lib.mkOption {
      type = lib.types.str;
      default = "info";
      description = "Log level (trace, debug, info, warn, error)";
    };
  };

  config = lib.mkIf cfg.enable {
    users.users.${cfg.user} = {
      isSystemUser = true;
      group = cfg.group;
      home = "/var/lib/rust-blog";
      createHome = true;
    };

    users.groups.${cfg.group} = {};

    systemd.services.rust-blog = {
      description = "Rust SSR Blog Server";
      after = [ "network.target" ];
      wantedBy = [ "multi-user.target" ];

      environment = {
        BLOG_HOST = cfg.host;
        BLOG_PORT = toString cfg.port;
        BLOG_CONTENT_PATH = cfg.contentPath;
        BLOG_TEMPLATES_PATH = if cfg.templatesPath != null
          then cfg.templatesPath
          else "${cfg.package}/share/blog-server/templates";
        BLOG_STATIC_PATH = if cfg.staticPath != null
          then cfg.staticPath
          else "${cfg.package}/share/blog-server/static";
        BLOG_POSTS_PER_PAGE = toString cfg.postsPerPage;
        BLOG_ENABLE_DRAFTS = lib.boolToString cfg.enableDrafts;
        RUST_LOG = "${cfg.logLevel},blog_server=${cfg.logLevel}";
      };

      serviceConfig = {
        Type = "simple";
        User = cfg.user;
        Group = cfg.group;
        ExecStart = "${cfg.package}/bin/blog-server";
        ExecReload = "${pkgs.coreutils}/bin/kill -HUP $MAINPID";
        Restart = "on-failure";
        RestartSec = 5;

        # Security hardening
        NoNewPrivileges = true;
        PrivateTmp = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        ReadWritePaths = [ cfg.contentPath "/var/lib/rust-blog" ];
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectControlGroups = true;
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        MemoryDenyWriteExecute = true;
        LockPersonality = true;
      };
    };

    # Ensure content directory exists
    systemd.tmpfiles.rules = [
      "d ${cfg.contentPath} 0755 ${cfg.user} ${cfg.group} -"
      "d ${cfg.contentPath}/posts 0755 ${cfg.user} ${cfg.group} -"
      "d ${cfg.contentPath}/pages 0755 ${cfg.user} ${cfg.group} -"
      "d ${cfg.contentPath}/images 0755 ${cfg.user} ${cfg.group} -"
    ];
  };
}
