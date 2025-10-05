import SettingsDialog from '@/app/settings/settings-dialog';
import { openDialog } from '../dialog/dialog.facade';

export async function openSettingsDialog() {
  openDialog({
    maxWidth: 'xs',
    children: <SettingsDialog />,
  });
}
