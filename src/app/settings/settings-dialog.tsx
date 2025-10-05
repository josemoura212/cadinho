'use client';

import { useEffect, useState } from 'react';
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';
import Checkbox from '@mui/material/Checkbox';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import { useCloseDialog } from '@/app/dialog/dialog-app';
import { info, error } from '@tauri-apps/plugin-log';
import { useDependencies } from '@/core/hooks/use-dependencies';

export default function SettingsDialog() {
  const [auto, setAuto] = useState(false);
  const onClose = useCloseDialog();
  const { status } = useDependencies();

  async function loadData() {
    try {
      const isAutoEnabled = await isEnabled();
      info(`Estado de autostart: ${isAutoEnabled}`);
      setAuto(isAutoEnabled);
    } catch (err) {
      error(`Erro ao carregar estado inicial: ${err}`);
      setAuto(false);
    }
  }

  useEffect(() => {
    loadData();
  }, []);

  async function toggle() {
    try {
      if (auto) {
        await disable();
        setAuto(false);
      } else {
        await enable();
        setAuto(true);
      }
    } catch (error) {
      console.error('Erro ao alterar autostart:', error);
    }
  }
  return (
    <Paper className="p-4 flex flex-col h-80 w-80 justify-between rounded-50">
      <div className="flex flex-col gap-4">
        <Typography className="text-center" sx={{ fontSize: 24 }}>
          Configurações
        </Typography>
        <Typography sx={{ fontSize: 20 }}>Dependências</Typography>
        <Typography sx={{ fontSize: 14 }}>
          FFMPEG: {status?.ffmpeg ? 'Instalado ✅' : 'Não instalado ❌'}
        </Typography>
        <Typography sx={{ fontSize: 14 }}>
          PANDOC: {status?.pandoc ? 'Instalado ✅' : 'Não instalado ❌'}
        </Typography>
        <Box className="flex items-center gap-2">
          <Typography>Iniciar com o sistema</Typography>
          <Checkbox checked={auto} onChange={toggle} />
        </Box>
      </div>

      <Button variant="contained" color="primary" onClick={onClose}>
        Fechar
      </Button>
    </Paper>
  );
}
