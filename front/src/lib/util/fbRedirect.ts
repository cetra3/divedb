import { browser } from '$app/environment';

export const fbLoginRedirect = browser
	? location.protocol +
	  '//' +
	  location.hostname +
	  (location.port ? ':' + location.port : '') +
	  '/facebook/login'
	: '';

export const fbRegisterRedirect = browser
	? location.protocol +
	  '//' +
	  location.hostname +
	  (location.port ? ':' + location.port : '') +
	  '/facebook/register'
	: '';
