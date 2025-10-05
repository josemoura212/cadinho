import { useDependencies } from '@/core/hooks/use-dependencies';
import CheckCircleIcon from '@mui/icons-material/CheckCircle';
import ErrorIcon from '@mui/icons-material/Error';
import Button from '@mui/material/Button';
import CircularProgress from '@mui/material/CircularProgress';

export function Dependencies() {
  const {
    status,
    loading,
    error,
    installing,
    installResults,
    refetch,
    installFFmpeg,
    installPandoc,
    installAll,
  } = useDependencies();

  const isInstallingFFmpeg = installing.includes('ffmpeg');
  const isInstallingPandoc = installing.includes('pandoc');

  return (
    <div className="flex flex-col items-center space-y-4 mb-8">
      <h2 className="text-xl font-semibold">Status das Dependências</h2>

      {loading && (
        <div className="flex items-center space-x-2">
          <CircularProgress size={20} />
          <span>Verificando dependências...</span>
        </div>
      )}

      {error && (
        <div className="flex items-center space-x-2 text-red-600">
          <ErrorIcon />
          <span>{error}</span>
          <Button onClick={refetch} size="small" variant="outlined">
            Tentar novamente
          </Button>
        </div>
      )}

      {status && !loading && !error && (
        <div className="flex flex-col space-y-4">
          <div className="flex flex-col items-center space-y-2 p-4 border rounded-lg">
            <div className="flex items-center space-x-2">
              {status.ffmpeg ? (
                <CheckCircleIcon className="text-green-600" />
              ) : (
                <ErrorIcon className="text-red-600" />
              )}
              <span className="font-medium">
                FFmpeg: {status.ffmpeg ? 'Instalado' : 'Não encontrado'}
              </span>
            </div>
            {!status.ffmpeg && (
              <Button
                onClick={installFFmpeg}
                disabled={isInstallingFFmpeg}
                variant="contained"
                color="primary"
                size="small"
                startIcon={
                  isInstallingFFmpeg ? <CircularProgress size={16} /> : null
                }
              >
                {isInstallingFFmpeg ? 'Instalando...' : 'Instalar FFmpeg'}
              </Button>
            )}
          </div>

          <div className="flex flex-col items-center space-y-2 p-4 border rounded-lg">
            <div className="flex items-center space-x-2">
              {status.pandoc ? (
                <CheckCircleIcon className="text-green-600" />
              ) : (
                <ErrorIcon className="text-red-600" />
              )}
              <span className="font-medium">
                Pandoc: {status.pandoc ? 'Instalado' : 'Não encontrado'}
              </span>
            </div>
            {!status.pandoc && (
              <Button
                onClick={installPandoc}
                disabled={isInstallingPandoc}
                variant="contained"
                color="primary"
                size="small"
                startIcon={
                  isInstallingPandoc ? <CircularProgress size={16} /> : null
                }
              >
                {isInstallingPandoc ? 'Instalando...' : 'Instalar Pandoc'}
              </Button>
            )}
          </div>

          {Object.keys(installResults).length > 0 && (
            <div className="flex flex-col space-y-2 p-4 border rounded-lg bg-gray-50">
              <h3 className="font-medium text-black">
                Resultados da Instalação:
              </h3>
              {Object.entries(installResults).map(([dep, result]) => (
                <div key={dep} className="text-sm">
                  <span
                    className={
                      result.startsWith('Erro:')
                        ? 'text-red-600'
                        : 'text-green-600'
                    }
                  >
                    {result}
                  </span>
                </div>
              ))}
            </div>
          )}

          <div className="flex justify-between gap-4">
            <Button onClick={refetch} size="small" variant="outlined">
              Verificar novamente
            </Button>
            {(!status.ffmpeg || !status.pandoc) && (
              <Button
                onClick={installAll}
                disabled={installing.length > 0}
                variant="contained"
                color="secondary"
                size="small"
                startIcon={
                  installing.length > 0 ? <CircularProgress size={16} /> : null
                }
              >
                {installing.length > 0 ? 'Instalando...' : 'Instalar Todas'}
              </Button>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
