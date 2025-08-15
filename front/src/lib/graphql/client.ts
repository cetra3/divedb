import { GraphQLClient } from 'graphql-request';
import { getSdk } from './generated'; // THIS FILE IS THE GENERATED FILE
import { browser } from '$app/environment';

export const graphqlClient = new GraphQLClient(
	browser ? `${window.location.origin}/api/graphql` : `${process.env.BACKEND_URL}/api/graphql`
);

export const client = getSdk(graphqlClient, (action) => {
	const token = browser ? localStorage.getItem('token') : undefined;

	if (token == undefined) {
		return action();
	} else {
		return action({ 'DiveDB-Token': token });
	}
});

/// For when you want to try and cache results
export const graphqlGetClient = new GraphQLClient(
	browser ? `${window.location.origin}/api/graphql` : `${process.env.BACKEND_URL}/api/graphql`,
	{
		method: 'GET',
		jsonSerializer: JSON
	}
);

export const getClient = getSdk(graphqlGetClient, (action) => {
	const token = browser ? localStorage.getItem('token') : undefined;

	if (token == undefined) {
		return action();
	} else {
		return action({ 'DiveDB-Token': token });
	}
});
