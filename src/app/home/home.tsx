'use client';

import { message, open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import Button from '@mui/material/Button';
import Paper from '@mui/material/Paper';
import IconButton from '@mui/material/IconButton';
import SettingsIcon from '@mui/icons-material/Settings';
import { openSettingsDialog } from '@/app/home/home.facade';

export default function HomePage() {
  async function testJob() {
    const file = await open({ multiple: false, directory: false });
    if (!file) return;
    const input = String(file);
    const output = input + '.copy.txt';
    await invoke('run_job', {
      job: { id: '1', input, output, kind: { Text: null } },
    });

    await message('Success', { title: 'Tauri', kind: 'info' });
  }

  return (
    <Paper
      variant="elevation"
      className="flex flex-col w-full min-h-screen p-4 justify-between"
      elevation={0}
      square
    >
      <div className="flex flex-row justify-end">
        <IconButton onClick={() => openSettingsDialog()}>
          <SettingsIcon />
        </IconButton>
      </div>

      <div className="flex flex-col items-end">
        <Button
          variant="contained"
          color="primary"
          onClick={testJob}
          className="font-medium rounded-xl p-4"
        >
          Rodar job de teste
        </Button>
      </div>
    </Paper>
  );
}
