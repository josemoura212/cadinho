import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export interface DependenciesStatus {
  ffmpeg: boolean;
  pandoc: boolean;
}

export interface InstallResult {
  [key: string]: string;
}

export function useDependencies() {
  const [status, setStatus] = useState<DependenciesStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [installing, setInstalling] = useState<string[]>([]);
  const [installResults, setInstallResults] = useState<InstallResult>({});

  const checkDependencies = async () => {
    try {
      setLoading(true);
      setError(null);
      const result =
        await invoke<Record<string, boolean>>('check_dependencies');
      setStatus({
        ffmpeg: result.ffmpeg || false,
        pandoc: result.pandoc || false,
      });
    } catch (err) {
      setError(
        err instanceof Error ? err.message : 'Erro ao verificar dependências'
      );
    } finally {
      setLoading(false);
    }
  };

  const installDependencies = async (deps: string[]) => {
    try {
      setInstalling(deps);
      setInstallResults({});
      const result = await invoke<InstallResult>('install_dependencies', {
        deps,
      });
      setInstallResults(result);
      // Re-verificar após instalação
      await checkDependencies();
    } catch (err) {
      setInstallResults({
        error:
          err instanceof Error ? err.message : 'Erro ao instalar dependências',
      });
    } finally {
      setInstalling([]);
    }
  };

  const installFFmpeg = async () => {
    await installDependencies(['ffmpeg']);
  };

  const installPandoc = async () => {
    await installDependencies(['pandoc']);
  };

  const installAll = async () => {
    await installDependencies(['ffmpeg', 'pandoc']);
  };

  useEffect(() => {
    checkDependencies();
  }, []);

  return {
    status,
    loading,
    error,
    installing,
    installResults,
    refetch: checkDependencies,
    installFFmpeg,
    installPandoc,
    installAll,
  };
}
