# Integração com o MeowSystem

O que o MeowSystem precisa saber para compilar, instalar, configurar e pôr as cores do pack no
**Discos**.

| | |
| --- | --- |
| repositório | `https://github.com/vitoriamariadb/meowsystem-drives` (ramo `main`) |
| upstream | `https://github.com/cosmic-utils/cosmic-ext-applet-drives` (base: `4a410b3`, main, 0.1.2) |
| app id | `io.github.meowsystem.Drives` |
| binário | `meowsystem-drives` |
| nome no painel | «Discos» |
| licença | GPL-3.0-only, a do upstream |

## Compilar

Rust com edição 2024 (testado com 1.99.1) e as bibliotecas de desenvolvimento do COSMIC (no
Pop!_OS: `build-essential`, `pkgconf`, `libxkbcommon-dev`, `libwayland-dev`, `libudev-dev`). O
libcosmic vem pinado no `Cargo.lock` (git `dc1cf9f0`); com `--locked` a compilação de
amanhã é a de hoje. O MeowSystem reaponta o libcosmic para o do painel instalado quando ele
compila contra o dele, e cai neste pino se não compilar.

```sh
cargo build --release --locked
cargo test --release --locked
```

## Instalar (em `~/.local`, sem root)

`just install-user` faz isto:

| arquivo | destino |
| --- | --- |
| `target/release/meowsystem-drives` | `~/.local/bin/meowsystem-drives` |
| `resources/io.github.meowsystem.Drives.desktop`, com `Exec=` trocado pelo caminho absoluto do binário | `~/.local/share/applications/io.github.meowsystem.Drives.desktop` |
| `resources/io.github.meowsystem.Drives-symbolic.svg` | `~/.local/share/icons/hicolor/scalable/apps/` |
| `resources/io.github.meowsystem.Drives.metainfo.xml` | `~/.local/share/metainfo/` |

O painel acha o applet pelo nome do `.desktop` (`X-CosmicApplet=true`) e só relê a lista ao subir.
Um applet já posto no painel só troca de binário quando o `cosmic-panel` reinicia.

Desfazer: `just uninstall-user`. A configuração fica em `~/.config/cosmic/io.github.meowsystem.Drives/v1/` (o applet ainda não guarda nada ali).

## Extensão de tema

Sem extensão, tudo segue o tema do COSMIC. Com ela, o pack põe as cores por cima.

- **Arquivo:** `$XDG_CONFIG_HOME/meowsystem/extensoes/io.github.meowsystem.Drives/tema.conf`
  (`~/.config/…` sem `XDG_CONFIG_HOME`). A variável `MEOWSYSTEM_TEMA` aponta outro arquivo.
- **Quando é lido:** ao subir e a cada vez que o popup abre. Trocar de pack vale no próximo clique
  no botão; o painel não precisa reiniciar.
- **Formato:** uma `chave = valor` por linha, `#` começa comentário, espaço em volta é ignorado.
  Cor é `#rrggbb` ou `#rrggbbaa`; nome de paleta (`mauve`) quem traduz é o MeowSystem.
- **Falhas:** arquivo ausente é o tema do COSMIC; arquivo ilegível ou linha ruim (cor inválida,
  chave desconhecida) vira aviso no log e as outras linhas valem. Nada disso derruba o applet.

| chave | valor | onde aparece |
| --- | --- | --- |
| `acento` | cor | botões principais, escolhas marcadas e interruptores ligados |
| `destaque` | cor | texto e ícones sobre o acento |
| `fundo` | cor | fundo do popup |
| `borda` | cor | contorno e divisórias do popup |
| `texto` | cor | texto e ícones do popup |
| `bloco` | cor | fundo dos botões, dos campos e das linhas do popup |
| `tinta` | cor | o ícone do applet na barra do painel |

Exemplo (Catppuccin Mocha, acento mauve):

```ini
# gerado pelo MeowSystem a partir do pack
acento = #cba6f7
destaque = #1e1e2e
fundo = #1e1e2ef0
borda = #45475a
texto = #cdd6f4
bloco = #313244
tinta = #cdd6f4
```

A mesma convenção (pasta, formato, chaves de cor) vale para a Bandeja
(`io.github.meowsystem.Bandeja`) e para o Ajustes rápidos
(`io.github.meowsystem.AjustesRapidos`); o MeowSystem gera todos com o mesmo código.

## O que o MeowSystem faz pela seção «Extensões»

1. **Pino.** `assets/extensoes/drives.pino`: `URL=https://github.com/vitoriamariadb/meowsystem-drives.git`,
   `COMMIT=<hash do main>`, `LIBCOSMIC=dc1cf9f0…`, `PADRAO=sim`.
2. **Build.** `scripts/extensoes_build.sh` clona em `~/.local/state/meowsystem/extensoes/drives/`,
   põe o commit do pino, roda `cargo build --release --locked`, carimba o commit para a segunda
   passada não recompilar e copia a tabela de «Instalar».
3. **Tema.** Na troca de pack escreve o `tema.conf` acima a partir da paleta.
4. **Painel.** Troca a entrada `dev.cappsy.CosmicExtAppletDrives` por `io.github.meowsystem.Drives` quando a pessoa aceita
   (a ordem dos applets é dela) e reinicia o `cosmic-panel`.
5. **Desinstalar.** `just uninstall-user` mais a pasta do clone; a configuração fica.
