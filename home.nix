{ config, pkgs, ... }:

{
  programs.home-manager.enable = true;

  home.packages = with pkgs; [
    nix

  ];

  programs.bash = {
    enable = true;
    profileExtra = ''
      . $HOME/.nix-profile/etc/profile.d/nix.sh

      if [ -d $HOME/profile.d ]; then
        for i in $HOME/profile.d/*.sh; do
          if [ -r $i ]; then
            . $i
          fi
        done
        unset i
      fi


    '';
  };
  home.file."profile.d/nix_vars.sh".source = ./nix_vars.sh;

}
