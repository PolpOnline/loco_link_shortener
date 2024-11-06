import { persisted } from 'svelte-persisted-store';

export const jwt = persisted('jwt', '');
