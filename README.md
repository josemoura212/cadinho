# Cadinho

Conversor universal de arquivos **desktop** (Tauri 2 + Next.js + Rust). Leve, offline e direto.

## Requisitos

- Rust (stable) + cargo
- Node.js 18+ (ou 20) e gerenciador (pnpm, npm ou yarn)
- Opcional: `ffmpeg` e `pandoc` no PATH (para mídia/documentos)

## Como rodar

    pnpm i
    pnpm dev      # desenvolvimento (Next + Tauri)
    pnpm build    # empacotar app desktop
