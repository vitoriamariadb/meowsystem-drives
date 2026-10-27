# Discos (Drives)

O applet de discos removíveis para o [COSMIC](https://system76.com/cosmic), com as cores do theme pack do [MeowSystem](https://github.com/vitoriamariadb/MeowSystem).

## De onde vem

Os Discos são um fork do **[cosmic-ext-applet-drives](https://github.com/cosmic-utils/cosmic-ext-applet-drives)**, de **Jonathan Capps** e da comunidade cosmic-utils. A lista de discos, o abrir no COSMIC Files e o desmontar são trabalho deles. Este fork muda o nome (`meowsystem-drives`, app id `io.github.meowsystem.Drives`) e acrescenta a extensão de tema do MeowSystem, descrita em [INTEGRACAO.md](INTEGRACAO.md). A licença continua a do original, GPL-3.0-only ([LICENSE](LICENSE)). Autoria do fork: Vitória Maria e André Farias.

---

# Removable Drives Applet (for the COSMIC™ Desktop)

An applet for the COSMIC™ desktop to manage your drives. This is in very early stages but is functional as it stands.

### What we have

- List of all mounted drives (removable USB and internal drives mounted by the user)
- Click the drive to open the mount in COSMIC Files
- Click the unmount button to unmount the drive before removable

### Possible future additions

- Network mounts
- Possibly notifications for unmounting, errors, and a bossy warning about removing a drive before unmounting (with the option to disable, of course)
- More robust unmounting process (currently it's just running the umount command and hoping the user has permission to do so)
- Use your default file browser when opening the drive, rather than forcing COSMIC Files

## Manual installation

The MeowSystem builds and installs it from the commit pinned in `assets/extensoes/drives.pino`. By hand, `just install-user` puts it in `~/.local`, no root. System-wide:

You're going to need to make sure you have the ability to compile Rust binaries, along with `git` and `just`

```sh
git clone https://github.com/vitoriamariadb/meowsystem-drives && cd meowsystem-drives
just build-release
sudo just install
```

## Credit & thanks
* [System76 and their COSMIC desktop environment](https://system76.com/cosmic/)
* [COSMIC Utilities](https://github.com/cosmic-utils/) - Organization containing third party utilities for COSMIC™
