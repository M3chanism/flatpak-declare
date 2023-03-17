Config file is located at:
```
/home/$USER/.config/flatpak-declare/config
```

Example config:
```
# Format:
# <install type>::<remote>::<appID>
system::flathub::com.discordapp.Discord
system::flathub::com.github.tchx84.Flatseal
system::flathub::com.usebottles.bottles
# This is a comment!
system::flathub::org.freedesktop.Platform.VulkanLayer.MangoHud
system::flathub::com.valvesoftware.Steam
system::flathub::org.gnucash.GnuCash
```

Installation:
``` 
mkdir -p ~/.config/flatpak-declare
touch ~/.config/flatpak-declare/config
 # Copy the example config into this file and modify as necessary.

mkdir -p ~/src/flatpak-declare
git clone https://github.com/justinmakes/flatpak-declare.git ~/src/flatpak-declare
```

To run the program, use 'cargo run' from the project root.
```
cd ~/src/flatpak-declare
cargo run
```

Issues:
- If a package does not install/uninstall the first time, run flatpak-declare a second time!

Current limitations:
- Does not manage remotes yet. Must add/remove remotes manually.
- Does not yet track pinned runtimes (listed with 'flatpak pin')

Goals:
- Create release binaries
- Manage remotes/origins
- Track pinned runtimes
- Prompt to remove unused apps/runtimes (flatpak uninstall --unused)
- Prompt to remove orphaned files (flatpak uninstall --delete-data)
- Set app permissions from config file as follows:
  ```
  <install type>::<remote>::<appID> {
    #permissions
  }
  ```

Stretch Goals:
- Rewrite all functions using 'libflatpak' rust crate
