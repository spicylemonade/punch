PREFIX=/home/${USER}
export NIX_STORE_DIR=${PREFIX}/nix/store
export NIX_DATA_DIR=${PREFIX}/nix/share
export NIX_LOG_DIR=${PREFIX}/nix/var/log/nix
export NIX_STATE_DIR=${PREFIX}/nix/var/nix
export NIX_CONF_DIR=${PREFIX}/nix/etc/nix
NIX_PROFILES="${NIX_STATE_DIR}/profiles/default ${PREFIX}/.nix-profile"
