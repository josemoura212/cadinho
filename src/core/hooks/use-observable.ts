import { useEffect, useState } from 'react';
import { Observable } from 'rxjs/internal/Observable';
import type { DependencyList } from 'react';

export function useObservable$<T>(
  observableFn: () => Observable<T[]>,
  deps: DependencyList
): [T[], boolean] {
  const [state, setState] = useState<T[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const subscription = observableFn().subscribe(list => {
      setState(list);
      setLoading(false);
    });

    return () => subscription.unsubscribe();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [observableFn, ...deps]);

  return [state, loading];
}
