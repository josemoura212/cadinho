# Cadinho

Conversor universal de arquivos **desktop** (Tauri 2 + Next.js + Rust). Leve, offline e direto.

## Requisitos

- Rust (stable) + cargo
- Node.js 18+ (ou 20) e gerenciador (pnpm, npm ou yarn)
- Opcional: `ffmpeg` e `pandoc` no PATH (para mídia/documentos)

## Estrutura

    cadinho/
      Cargo.toml
      core/
        Cargo.toml
        src/lib.rs
      src-tauri/
        Cargo.toml
        tauri.conf.json
        capabilities/default.json
        src/lib.rs
        icons/
          icon.png
      app/                  # Next (App Router)
        page.tsx
      package.json
      next.config.mjs
      tsconfig.json

## Configuração mínima

### Next (export estático) – `next.config.mjs`

    /** @type {import('next').NextConfig} */
    export default {
      output: "export",
      images: { unoptimized: true }
    };

### Tauri – `src-tauri/tauri.conf.json`

    {
      "productName": "Cadinho",
      "identifier": "dev.seuuser.cadinho",
      "version": "0.1.0",
      "build": {
        "beforeDevCommand": "next dev -p 3000",
        "beforeBuildCommand": "next build",
        "devUrl": "http://localhost:3000",
        "frontendDist": "../out"
      },
      "app": {
        "windows": [
          { "label": "main", "title": "Cadinho", "visible": false, "width": 980, "height": 680 }
        ]
      },
      "bundle": { "active": true, "targets": "all", "icon": ["icons/icon.png"] }
    }

### Capabilities – `src-tauri/capabilities/default.json`

    {
      "$schema": "../gen/schemas/desktop-schema.json",
      "identifier": "main",
      "windows": ["main"],
      "permissions": [
        "core:window:default",
        "dialog:allow-open",
        "autostart:allow-enable",
        "autostart:allow-disable",
        "autostart:allow-is-enabled",
        "log:default"
      ]
    }

## Scripts – `package.json`

    {
      "scripts": {
        "dev": "tauri dev",
        "build": "tauri build"
      }
    }

## Como rodar

    pnpm i
    pnpm dev      # desenvolvimento (Next + Tauri)
    pnpm build    # empacotar app desktop
