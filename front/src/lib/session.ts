import { writable } from 'svelte/store';

export interface UserInfo {
	id: string;
	email: string;
	level: 'ADMIN' | 'EDITOR' | 'USER';
}
// interface Locals {}
// interface Platform {}
export interface Session {
	loggedIn?: boolean;
	user?: UserInfo;
}

export const session = writable<Session>({});
