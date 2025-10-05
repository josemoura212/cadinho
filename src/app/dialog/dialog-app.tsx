'use client';
import { createContext, memo, useContext } from 'react';
import MatDialog, { DialogProps } from '@mui/material/Dialog';
import { closeDialog, dialogs$ } from '@/app/dialog/dialog.facade';
import useBehavior from '@/core/hooks/use-behavior';

const DialogContext = createContext<Partial<DialogProps> | null>(null);

export function useCloseDialog() {
  const dialog = useContext(DialogContext);

  if (!dialog) {
    throw new Error('useCloseDialog must be into a DialogContext.Provider!');
  }

  return () => closeDialog(dialog);
}

function DialogApp() {
  const dialogs = useBehavior(dialogs$);

  return (
    <>
      {dialogs.map(dialog => {
        const { key, ...dialogWithoutKey } = dialog;
        return (
          <DialogContext.Provider key={key} value={dialog}>
            <MatDialog
              open={true}
              onClose={() => closeDialog(dialog)}
              classes={{
                paper: 'rounded-8',
              }}
              {...dialogWithoutKey}
            />
          </DialogContext.Provider>
        );
      })}
    </>
  );
}

export default memo(DialogApp);
