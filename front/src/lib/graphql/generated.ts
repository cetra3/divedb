import type { GraphQLClient } from 'graphql-request';
import type { GraphQLClientRequestHeaders } from 'graphql-request/build/cjs/types';
import gql from 'graphql-tag';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = {
	[_ in K]?: never;
};
export type Incremental<T> =
	| T
	| { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
	ID: { input: string | number; output: string };
	String: { input: string; output: string };
	Boolean: { input: boolean; output: boolean };
	Int: { input: number; output: number };
	Float: { input: number; output: number };
	/**
	 * Implement the DateTime<Utc> scalar
	 *
	 * The input/output is a string in RFC3339 format.
	 */
	DateTime: { input: any; output: any };
	/** A scalar that can represent any JSON Object value. */
	JSONObject: { input: any; output: any };
	/**
	 * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
	 * Strings within GraphQL. UUIDs are used to assign unique identifiers to
	 * entities without requiring a central allocating authority.
	 *
	 * # References
	 *
	 * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
	 * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
	 */
	UUID: { input: string; output: string };
};

export type Category = {
	__typename?: 'Category';
	id: Scalars['UUID']['output'];
	name: Scalars['String']['output'];
	values: Array<CategoryValue>;
};

export type CategoryValue = {
	__typename?: 'CategoryValue';
	categoryId: Scalars['UUID']['output'];
	id: Scalars['UUID']['output'];
	value: Scalars['String']['output'];
};

export type CreateCategory = {
	id?: InputMaybe<Scalars['UUID']['input']>;
	name: Scalars['String']['input'];
};

export type CreateCategoryValue = {
	categoryId: Scalars['UUID']['input'];
	id?: InputMaybe<Scalars['UUID']['input']>;
	value: Scalars['String']['input'];
};

export type CreateDive = {
	date: Scalars['DateTime']['input'];
	depth: Scalars['Float']['input'];
	description: Scalars['String']['input'];
	diveSiteId?: InputMaybe<Scalars['UUID']['input']>;
	duration: Scalars['Int']['input'];
	id?: InputMaybe<Scalars['UUID']['input']>;
	published: Scalars['Boolean']['input'];
};

export type CreateDiveComment = {
	description: Scalars['String']['input'];
	diveId: Scalars['UUID']['input'];
	id?: InputMaybe<Scalars['UUID']['input']>;
};

export type CreateDiveSite = {
	access: Scalars['String']['input'];
	depth: Scalars['Float']['input'];
	description: Scalars['String']['input'];
	difficulty: Difficulty;
	id?: InputMaybe<Scalars['UUID']['input']>;
	lat: Scalars['Float']['input'];
	lon: Scalars['Float']['input'];
	name: Scalars['String']['input'];
	photoId?: InputMaybe<Scalars['UUID']['input']>;
	published: Scalars['Boolean']['input'];
};

export type CreatePhoto = {
	date: Scalars['DateTime']['input'];
	diveId?: InputMaybe<Scalars['UUID']['input']>;
	diveSiteId?: InputMaybe<Scalars['UUID']['input']>;
	filename: Scalars['String']['input'];
	id?: InputMaybe<Scalars['UUID']['input']>;
	internal?: InputMaybe<Scalars['Boolean']['input']>;
	sealifeId?: InputMaybe<Scalars['UUID']['input']>;
	size: Scalars['Int']['input'];
	userId: Scalars['UUID']['input'];
};

export type CreateRegion = {
	id?: InputMaybe<Scalars['UUID']['input']>;
	latMax: Scalars['Float']['input'];
	latMin: Scalars['Float']['input'];
	lonMax: Scalars['Float']['input'];
	lonMin: Scalars['Float']['input'];
	name: Scalars['String']['input'];
};

export type CreateSealife = {
	categoryMap?: InputMaybe<Scalars['JSONObject']['input']>;
	description: Scalars['String']['input'];
	hideLocation: Scalars['Boolean']['input'];
	id?: InputMaybe<Scalars['UUID']['input']>;
	name: Scalars['String']['input'];
	photoId?: InputMaybe<Scalars['UUID']['input']>;
	scientificName?: InputMaybe<Scalars['String']['input']>;
};

export enum Difficulty {
	Aow = 'AOW',
	Ow = 'OW',
	Tech = 'TECH'
}

export type Dive = {
	__typename?: 'Dive';
	comments: Array<DiveComment>;
	date: Scalars['DateTime']['output'];
	depth: Scalars['Float']['output'];
	description: Scalars['String']['output'];
	diveSite?: Maybe<DiveSite>;
	diveSiteId?: Maybe<Scalars['UUID']['output']>;
	duration: Scalars['Int']['output'];
	hasMetrics: Scalars['Boolean']['output'];
	id: Scalars['UUID']['output'];
	latestPhotos: Array<Photo>;
	liked: Scalars['Boolean']['output'];
	likes: Scalars['Int']['output'];
	numComments: Scalars['Int']['output'];
	number: Scalars['Int']['output'];
	published: Scalars['Boolean']['output'];
	summary: Scalars['String']['output'];
	user: PublicUserInfo;
	userId: Scalars['UUID']['output'];
};

export type DiveComment = {
	__typename?: 'DiveComment';
	date: Scalars['DateTime']['output'];
	description: Scalars['String']['output'];
	dive: Dive;
	diveId: Scalars['UUID']['output'];
	id: Scalars['UUID']['output'];
	user: PublicUserInfo;
	userId: Scalars['UUID']['output'];
};

export type DiveSite = {
	__typename?: 'DiveSite';
	access: Scalars['String']['output'];
	date: Scalars['DateTime']['output'];
	depth: Scalars['Float']['output'];
	description: Scalars['String']['output'];
	difficulty: Difficulty;
	id: Scalars['UUID']['output'];
	lat: Scalars['Float']['output'];
	latestDives: Array<Dive>;
	latestPhotos: Array<Photo>;
	lon: Scalars['Float']['output'];
	name: Scalars['String']['output'];
	photo?: Maybe<Photo>;
	photoId?: Maybe<Scalars['UUID']['output']>;
	published: Scalars['Boolean']['output'];
	references: Array<OgReference>;
	siteMetrics: SiteMetric;
	slug?: Maybe<Scalars['String']['output']>;
	summary: Scalars['String']['output'];
	userId?: Maybe<Scalars['UUID']['output']>;
};

export type Feedback = {
	__typename?: 'Feedback';
	date: Scalars['DateTime']['output'];
	feedback: Scalars['String']['output'];
	id: Scalars['UUID']['output'];
	user: UserInfo;
	userId: Scalars['UUID']['output'];
};

export type LoginResponse = {
	__typename?: 'LoginResponse';
	copyrightLocation?: Maybe<OverlayLocation>;
	description: Scalars['String']['output'];
	displayName?: Maybe<Scalars['String']['output']>;
	email: Scalars['String']['output'];
	emailVerified: Scalars['Boolean']['output'];
	id: Scalars['UUID']['output'];
	level: UserLevel;
	photoId?: Maybe<Scalars['UUID']['output']>;
	token: Scalars['String']['output'];
	username: Scalars['String']['output'];
	watermarkLocation: OverlayLocation;
};

export type Mutation = {
	__typename?: 'Mutation';
	addFeedback: Feedback;
	changePassword: Scalars['Boolean']['output'];
	checkReference: OgReference;
	deleteUser: Scalars['Boolean']['output'];
	fbLogin: LoginResponse;
	fbRegisterUser: LoginResponse;
	likeDive: Scalars['Boolean']['output'];
	likePhoto: Scalars['Boolean']['output'];
	login: LoginResponse;
	mergeDiveSites: Scalars['Boolean']['output'];
	newCategory: Category;
	newCategoryValue: CategoryValue;
	newComment: DiveComment;
	newDive: Dive;
	newDiveSite: DiveSite;
	newReference: OgReference;
	newRegion: Region;
	newSealife: Sealife;
	registerUser: Scalars['Boolean']['output'];
	removeCategory: Scalars['Boolean']['output'];
	removeCategoryValue: Scalars['Boolean']['output'];
	removeComment: Scalars['Boolean']['output'];
	removeDive: Scalars['Boolean']['output'];
	removeDiveSite: Scalars['Boolean']['output'];
	removePhoto: Scalars['Boolean']['output'];
	removeReference: Scalars['Boolean']['output'];
	removeRegion: Scalars['Boolean']['output'];
	removeSealife: Scalars['Boolean']['output'];
	requestResetToken: Scalars['Boolean']['output'];
	resendVerification: Scalars['Boolean']['output'];
	resetPassword: LoginResponse;
	syncSubsurface: Scalars['Boolean']['output'];
	unlikeDive: Scalars['Boolean']['output'];
	unlikePhoto: Scalars['Boolean']['output'];
	updatePhoto: Photo;
	updateSettings?: Maybe<LoginResponse>;
	verifyEmail: LoginResponse;
};

export type MutationAddFeedbackArgs = {
	feedback: Scalars['String']['input'];
};

export type MutationChangePasswordArgs = {
	newPassword: Scalars['String']['input'];
	oldPassword: Scalars['String']['input'];
};

export type MutationCheckReferenceArgs = {
	url: Scalars['String']['input'];
};

export type MutationDeleteUserArgs = {
	password: Scalars['String']['input'];
};

export type MutationFbLoginArgs = {
	code: Scalars['String']['input'];
	redirectUri: Scalars['String']['input'];
};

export type MutationFbRegisterUserArgs = {
	code: Scalars['String']['input'];
	redirectUri: Scalars['String']['input'];
	username: Scalars['String']['input'];
};

export type MutationLikeDiveArgs = {
	diveId: Scalars['UUID']['input'];
};

export type MutationLikePhotoArgs = {
	photoId: Scalars['UUID']['input'];
};

export type MutationLoginArgs = {
	email: Scalars['String']['input'];
	password: Scalars['String']['input'];
};

export type MutationMergeDiveSitesArgs = {
	fromId: Scalars['UUID']['input'];
	toId: Scalars['UUID']['input'];
};

export type MutationNewCategoryArgs = {
	category: CreateCategory;
};

export type MutationNewCategoryValueArgs = {
	categoryValue: CreateCategoryValue;
};

export type MutationNewCommentArgs = {
	comment: CreateDiveComment;
};

export type MutationNewDiveArgs = {
	dive: CreateDive;
};

export type MutationNewDiveSiteArgs = {
	site: CreateDiveSite;
};

export type MutationNewReferenceArgs = {
	diveSiteId?: InputMaybe<Scalars['UUID']['input']>;
	sealifeId?: InputMaybe<Scalars['UUID']['input']>;
	url: Scalars['String']['input'];
};

export type MutationNewRegionArgs = {
	region: CreateRegion;
};

export type MutationNewSealifeArgs = {
	sealife: CreateSealife;
};

export type MutationRegisterUserArgs = {
	email: Scalars['String']['input'];
	password: Scalars['String']['input'];
	username: Scalars['String']['input'];
};

export type MutationRemoveCategoryArgs = {
	categoryId: Scalars['UUID']['input'];
};

export type MutationRemoveCategoryValueArgs = {
	categoryValueId: Scalars['UUID']['input'];
};

export type MutationRemoveCommentArgs = {
	id: Scalars['UUID']['input'];
};

export type MutationRemoveDiveArgs = {
	id: Scalars['UUID']['input'];
};

export type MutationRemoveDiveSiteArgs = {
	id: Scalars['UUID']['input'];
};

export type MutationRemovePhotoArgs = {
	id: Scalars['UUID']['input'];
};

export type MutationRemoveReferenceArgs = {
	id: Scalars['UUID']['input'];
};

export type MutationRemoveRegionArgs = {
	id: Scalars['UUID']['input'];
};

export type MutationRemoveSealifeArgs = {
	id: Scalars['UUID']['input'];
};

export type MutationRequestResetTokenArgs = {
	email: Scalars['String']['input'];
};

export type MutationResendVerificationArgs = {
	email: Scalars['String']['input'];
};

export type MutationResetPasswordArgs = {
	email: Scalars['String']['input'];
	newPassword: Scalars['String']['input'];
	token: Scalars['UUID']['input'];
};

export type MutationSyncSubsurfaceArgs = {
	email: Scalars['String']['input'];
	password: Scalars['String']['input'];
};

export type MutationUnlikeDiveArgs = {
	diveId: Scalars['UUID']['input'];
};

export type MutationUnlikePhotoArgs = {
	photoId: Scalars['UUID']['input'];
};

export type MutationUpdatePhotoArgs = {
	photo: CreatePhoto;
};

export type MutationUpdateSettingsArgs = {
	copyrightLocation?: InputMaybe<OverlayLocation>;
	description: Scalars['String']['input'];
	displayName?: InputMaybe<Scalars['String']['input']>;
	photoId?: InputMaybe<Scalars['UUID']['input']>;
	watermarkLocation: OverlayLocation;
};

export type MutationVerifyEmailArgs = {
	email: Scalars['String']['input'];
	token: Scalars['UUID']['input'];
};

export type OgReference = {
	__typename?: 'OgReference';
	description: Scalars['String']['output'];
	id: Scalars['UUID']['output'];
	imageUrl?: Maybe<Scalars['String']['output']>;
	lastFetched: Scalars['DateTime']['output'];
	title: Scalars['String']['output'];
	url: Scalars['String']['output'];
};

export enum OverlayLocation {
	BottomLeft = 'BOTTOM_LEFT',
	BottomRight = 'BOTTOM_RIGHT',
	TopLeft = 'TOP_LEFT',
	TopRight = 'TOP_RIGHT'
}

export type Photo = {
	__typename?: 'Photo';
	date?: Maybe<Scalars['DateTime']['output']>;
	dive?: Maybe<Dive>;
	diveSite?: Maybe<DiveSite>;
	filename: Scalars['String']['output'];
	height: Scalars['Float']['output'];
	id: Scalars['UUID']['output'];
	liked: Scalars['Boolean']['output'];
	likes: Scalars['Int']['output'];
	sealife?: Maybe<Sealife>;
	size: Scalars['Float']['output'];
	user: PublicUserInfo;
	userId: Scalars['UUID']['output'];
	width: Scalars['Float']['output'];
};

export type PublicUserInfo = {
	__typename?: 'PublicUserInfo';
	description: Scalars['String']['output'];
	displayName?: Maybe<Scalars['String']['output']>;
	diveCount: Scalars['Int']['output'];
	id: Scalars['UUID']['output'];
	latestDives: Array<Dive>;
	latestPhotos: Array<Photo>;
	photo?: Maybe<Photo>;
	photoCount: Scalars['Int']['output'];
	photoId?: Maybe<Scalars['UUID']['output']>;
	username: Scalars['String']['output'];
};

export type Query = {
	__typename?: 'Query';
	categories: Array<Category>;
	categoryValues: Array<CategoryValue>;
	currentUser?: Maybe<LoginResponse>;
	diveSites: Array<DiveSite>;
	dives: Array<Dive>;
	fbAppId: Scalars['String']['output'];
	feedback: Array<Feedback>;
	photos: Array<Photo>;
	popularDiveSites: Array<DiveSite>;
	recentDives: Array<Dive>;
	regions: Array<Region>;
	sealife: Array<Sealife>;
	search: Array<SearchResult>;
	siteUrl: Scalars['String']['output'];
	user: PublicUserInfo;
};

export type QueryDiveSitesArgs = {
	id?: InputMaybe<Scalars['UUID']['input']>;
	maxDepth?: InputMaybe<Scalars['Float']['input']>;
	name?: InputMaybe<Scalars['String']['input']>;
	slug?: InputMaybe<Scalars['String']['input']>;
};

export type QueryDivesArgs = {
	diveSite?: InputMaybe<Scalars['UUID']['input']>;
	id?: InputMaybe<Scalars['UUID']['input']>;
	maxDepth?: InputMaybe<Scalars['Float']['input']>;
	offset?: InputMaybe<Scalars['Int']['input']>;
	userId?: InputMaybe<Scalars['UUID']['input']>;
	username?: InputMaybe<Scalars['String']['input']>;
};

export type QueryFeedbackArgs = {
	id?: InputMaybe<Scalars['UUID']['input']>;
};

export type QueryPhotosArgs = {
	dive?: InputMaybe<Scalars['UUID']['input']>;
	diveSite?: InputMaybe<Scalars['UUID']['input']>;
	id?: InputMaybe<Scalars['UUID']['input']>;
	offset?: InputMaybe<Scalars['Int']['input']>;
	orderByUpload?: InputMaybe<Scalars['Boolean']['input']>;
	sealifeId?: InputMaybe<Scalars['UUID']['input']>;
	userId?: InputMaybe<Scalars['UUID']['input']>;
	username?: InputMaybe<Scalars['String']['input']>;
};

export type QuerySealifeArgs = {
	categoryValues?: InputMaybe<Array<Scalars['UUID']['input']>>;
	id?: InputMaybe<Scalars['UUID']['input']>;
	name?: InputMaybe<Scalars['String']['input']>;
	scientificName?: InputMaybe<Scalars['String']['input']>;
	slug?: InputMaybe<Scalars['String']['input']>;
};

export type QuerySearchArgs = {
	offset?: InputMaybe<Scalars['Int']['input']>;
	query: Scalars['String']['input'];
};

export type QueryUserArgs = {
	username: Scalars['String']['input'];
};

export type Region = {
	__typename?: 'Region';
	id: Scalars['UUID']['output'];
	latMax: Scalars['Float']['output'];
	latMin: Scalars['Float']['output'];
	lonMax: Scalars['Float']['output'];
	lonMin: Scalars['Float']['output'];
	name: Scalars['String']['output'];
	slug: Scalars['String']['output'];
};

export type Sealife = {
	__typename?: 'Sealife';
	categoryMap: Scalars['JSONObject']['output'];
	date: Scalars['DateTime']['output'];
	description: Scalars['String']['output'];
	hideLocation: Scalars['Boolean']['output'];
	id: Scalars['UUID']['output'];
	latestPhotos: Array<Photo>;
	name: Scalars['String']['output'];
	photo?: Maybe<Photo>;
	photoId?: Maybe<Scalars['UUID']['output']>;
	references: Array<OgReference>;
	scientificName?: Maybe<Scalars['String']['output']>;
	slug?: Maybe<Scalars['String']['output']>;
	summary: Scalars['String']['output'];
};

export type SearchResult = {
	__typename?: 'SearchResult';
	id: Scalars['String']['output'];
	kind: SearchResultKind;
	name: Scalars['String']['output'];
	photoId?: Maybe<Scalars['String']['output']>;
	scientificName?: Maybe<Scalars['String']['output']>;
	slug: Scalars['String']['output'];
	summary: Scalars['String']['output'];
};

export enum SearchResultKind {
	DiveSite = 'DIVE_SITE',
	Sealife = 'SEALIFE'
}

export type SiteMetric = {
	__typename?: 'SiteMetric';
	diveCount: Scalars['Int']['output'];
	photoCount: Scalars['Int']['output'];
};

export type UserInfo = {
	__typename?: 'UserInfo';
	description: Scalars['String']['output'];
	displayName?: Maybe<Scalars['String']['output']>;
	email?: Maybe<Scalars['String']['output']>;
	emailVerified: Scalars['Boolean']['output'];
	id: Scalars['UUID']['output'];
	level: UserLevel;
	username: Scalars['String']['output'];
};

export enum UserLevel {
	Admin = 'ADMIN',
	Editor = 'EDITOR',
	User = 'USER'
}

export type CategoryNodeFragment = {
	__typename?: 'Category';
	id: string;
	name: string;
	values: Array<{ __typename?: 'CategoryValue'; id: string; categoryId: string; value: string }>;
};

export type CategoryValueNodeFragment = {
	__typename?: 'CategoryValue';
	id: string;
	categoryId: string;
	value: string;
};

export type CommentFragment = {
	__typename?: 'DiveComment';
	id: string;
	diveId: string;
	date: any;
	description: string;
	user: {
		__typename?: 'PublicUserInfo';
		id: string;
		username: string;
		displayName?: string | null;
	};
};

export type DiveSummaryFragment = {
	__typename?: 'Dive';
	id: string;
	date: any;
	number: number;
	numComments: number;
	user: {
		__typename?: 'PublicUserInfo';
		id: string;
		username: string;
		displayName?: string | null;
	};
	diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
};

export type DiveWithMetricsFragment = {
	__typename?: 'Dive';
	id: string;
	userId: string;
	date: any;
	depth: number;
	duration: number;
	number: number;
	hasMetrics: boolean;
	summary: string;
	likes: number;
	liked: boolean;
	numComments: number;
	diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
	latestPhotos: Array<{
		__typename?: 'Photo';
		id: string;
		userId: string;
		filename: string;
		date?: any | null;
		size: number;
		width: number;
		height: number;
		likes: number;
		liked: boolean;
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			number: number;
			numComments: number;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		} | null;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		sealife?: {
			__typename?: 'Sealife';
			id: string;
			name: string;
			scientificName?: string | null;
			summary: string;
			slug?: string | null;
			photoId?: string | null;
		} | null;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	}>;
	user: {
		__typename?: 'PublicUserInfo';
		id: string;
		username: string;
		displayName?: string | null;
	};
};

export type DiveNodeFragment = {
	__typename?: 'Dive';
	id: string;
	userId: string;
	date: any;
	depth: number;
	duration: number;
	number: number;
	hasMetrics: boolean;
	description: string;
	summary: string;
	published: boolean;
	likes: number;
	liked: boolean;
	numComments: number;
	diveSiteId?: string | null;
	comments: Array<{
		__typename?: 'DiveComment';
		id: string;
		diveId: string;
		date: any;
		description: string;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	}>;
	user: {
		__typename?: 'PublicUserInfo';
		id: string;
		username: string;
		displayName?: string | null;
	};
	latestPhotos: Array<{
		__typename?: 'Photo';
		id: string;
		userId: string;
		filename: string;
		date?: any | null;
		size: number;
		width: number;
		height: number;
		likes: number;
		liked: boolean;
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			number: number;
			numComments: number;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		} | null;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		sealife?: {
			__typename?: 'Sealife';
			id: string;
			name: string;
			scientificName?: string | null;
			summary: string;
			slug?: string | null;
			photoId?: string | null;
		} | null;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	}>;
	diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
};

export type SiteFragment = {
	__typename?: 'DiveSite';
	id: string;
	name: string;
	description: string;
	summary: string;
	access: string;
	difficulty: Difficulty;
	depth: number;
	lat: number;
	lon: number;
	published: boolean;
	userId?: string | null;
	slug?: string | null;
	date: any;
	photoId?: string | null;
	siteMetrics: { __typename?: 'SiteMetric'; photoCount: number; diveCount: number };
	photo?: {
		__typename?: 'Photo';
		id: string;
		userId: string;
		filename: string;
		date?: any | null;
		size: number;
		width: number;
		height: number;
		likes: number;
		liked: boolean;
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			number: number;
			numComments: number;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		} | null;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		sealife?: {
			__typename?: 'Sealife';
			id: string;
			name: string;
			scientificName?: string | null;
			summary: string;
			slug?: string | null;
			photoId?: string | null;
		} | null;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	} | null;
	latestPhotos: Array<{
		__typename?: 'Photo';
		id: string;
		userId: string;
		filename: string;
		date?: any | null;
		size: number;
		width: number;
		height: number;
		likes: number;
		liked: boolean;
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			number: number;
			numComments: number;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		} | null;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		sealife?: {
			__typename?: 'Sealife';
			id: string;
			name: string;
			scientificName?: string | null;
			summary: string;
			slug?: string | null;
			photoId?: string | null;
		} | null;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	}>;
	latestDives: Array<{
		__typename?: 'Dive';
		id: string;
		userId: string;
		date: any;
		depth: number;
		duration: number;
		number: number;
		hasMetrics: boolean;
		summary: string;
		likes: number;
		liked: boolean;
		numComments: number;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		latestPhotos: Array<{
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	}>;
	references: Array<{
		__typename?: 'OgReference';
		id: string;
		url: string;
		title: string;
		description: string;
		imageUrl?: string | null;
		lastFetched: any;
	}>;
};

export type SiteSummaryFragment = {
	__typename?: 'DiveSite';
	name: string;
	id: string;
	slug?: string | null;
};

export type SiteMetricNodeFragment = {
	__typename?: 'SiteMetric';
	photoCount: number;
	diveCount: number;
};

export type SiteSummaryMetricsFragment = {
	__typename?: 'DiveSite';
	id: string;
	name: string;
	summary: string;
	slug?: string | null;
	lat: number;
	lon: number;
	photoId?: string | null;
	siteMetrics: { __typename?: 'SiteMetric'; photoCount: number; diveCount: number };
};

export type FeedbackNodeFragment = {
	__typename?: 'Feedback';
	id: string;
	date: any;
	feedback: string;
	user: {
		__typename?: 'UserInfo';
		id: string;
		email?: string | null;
		level: UserLevel;
		username: string;
	};
};

export type PhotoSummaryFragment = {
	__typename?: 'Photo';
	id: string;
	userId: string;
	filename: string;
	date?: any | null;
	size: number;
	width: number;
	height: number;
	likes: number;
	liked: boolean;
	dive?: {
		__typename?: 'Dive';
		id: string;
		date: any;
		number: number;
		numComments: number;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
	} | null;
	diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
	sealife?: {
		__typename?: 'Sealife';
		id: string;
		name: string;
		scientificName?: string | null;
		summary: string;
		slug?: string | null;
		photoId?: string | null;
	} | null;
	user: {
		__typename?: 'PublicUserInfo';
		id: string;
		username: string;
		displayName?: string | null;
	};
};

export type ReferenceFragment = {
	__typename?: 'OgReference';
	id: string;
	url: string;
	title: string;
	description: string;
	imageUrl?: string | null;
	lastFetched: any;
};

export type RegionNodeFragment = {
	__typename?: 'Region';
	id: string;
	name: string;
	latMin: number;
	lonMin: number;
	latMax: number;
	lonMax: number;
	slug: string;
};

export type SealifeNodeFragment = {
	__typename?: 'Sealife';
	id: string;
	name: string;
	summary: string;
	scientificName?: string | null;
	description: string;
	slug?: string | null;
	date: any;
	categoryMap: any;
	hideLocation: boolean;
	photoId?: string | null;
	photo?: {
		__typename?: 'Photo';
		id: string;
		userId: string;
		filename: string;
		date?: any | null;
		size: number;
		width: number;
		height: number;
		likes: number;
		liked: boolean;
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			number: number;
			numComments: number;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		} | null;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		sealife?: {
			__typename?: 'Sealife';
			id: string;
			name: string;
			scientificName?: string | null;
			summary: string;
			slug?: string | null;
			photoId?: string | null;
		} | null;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	} | null;
	latestPhotos: Array<{
		__typename?: 'Photo';
		id: string;
		userId: string;
		filename: string;
		date?: any | null;
		size: number;
		width: number;
		height: number;
		likes: number;
		liked: boolean;
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			number: number;
			numComments: number;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		} | null;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		sealife?: {
			__typename?: 'Sealife';
			id: string;
			name: string;
			scientificName?: string | null;
			summary: string;
			slug?: string | null;
			photoId?: string | null;
		} | null;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	}>;
	references: Array<{
		__typename?: 'OgReference';
		id: string;
		url: string;
		title: string;
		description: string;
		imageUrl?: string | null;
		lastFetched: any;
	}>;
};

export type SealifeSummaryFragment = {
	__typename?: 'Sealife';
	id: string;
	name: string;
	scientificName?: string | null;
	summary: string;
	slug?: string | null;
	photoId?: string | null;
};

export type SearchResultNodeFragment = {
	__typename?: 'SearchResult';
	id: string;
	kind: SearchResultKind;
	slug: string;
	photoId?: string | null;
	name: string;
	scientificName?: string | null;
	summary: string;
};

export type CurrentUserFragment = {
	__typename?: 'LoginResponse';
	id: string;
	email: string;
	level: UserLevel;
	username: string;
	displayName?: string | null;
	watermarkLocation: OverlayLocation;
	copyrightLocation?: OverlayLocation | null;
	description: string;
	photoId?: string | null;
};

export type CurrentUserTokenFragment = {
	__typename?: 'LoginResponse';
	id: string;
	email: string;
	username: string;
	level: UserLevel;
	token: string;
};

export type UserSummaryFragment = {
	__typename?: 'PublicUserInfo';
	id: string;
	username: string;
	displayName?: string | null;
};

export type UserInfoFragment = {
	__typename?: 'PublicUserInfo';
	id: string;
	username: string;
	displayName?: string | null;
	description: string;
	photoId?: string | null;
	photoCount: number;
	diveCount: number;
	latestPhotos: Array<{
		__typename?: 'Photo';
		id: string;
		userId: string;
		filename: string;
		date?: any | null;
		size: number;
		width: number;
		height: number;
		likes: number;
		liked: boolean;
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			number: number;
			numComments: number;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		} | null;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		sealife?: {
			__typename?: 'Sealife';
			id: string;
			name: string;
			scientificName?: string | null;
			summary: string;
			slug?: string | null;
			photoId?: string | null;
		} | null;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	}>;
	latestDives: Array<{
		__typename?: 'Dive';
		id: string;
		userId: string;
		date: any;
		depth: number;
		duration: number;
		number: number;
		hasMetrics: boolean;
		summary: string;
		likes: number;
		liked: boolean;
		numComments: number;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		latestPhotos: Array<{
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	}>;
};

export type AddDiveMutationVariables = Exact<{
	dive: CreateDive;
}>;

export type AddDiveMutation = {
	__typename?: 'Mutation';
	newDive: {
		__typename?: 'Dive';
		id: string;
		userId: string;
		date: any;
		depth: number;
		duration: number;
		number: number;
		hasMetrics: boolean;
		description: string;
		summary: string;
		published: boolean;
		likes: number;
		liked: boolean;
		numComments: number;
		diveSiteId?: string | null;
		comments: Array<{
			__typename?: 'DiveComment';
			id: string;
			diveId: string;
			date: any;
			description: string;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
		latestPhotos: Array<{
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
	};
};

export type AddDiveSiteMutationVariables = Exact<{
	site: CreateDiveSite;
}>;

export type AddDiveSiteMutation = {
	__typename?: 'Mutation';
	newDiveSite: {
		__typename?: 'DiveSite';
		id: string;
		name: string;
		description: string;
		summary: string;
		access: string;
		difficulty: Difficulty;
		depth: number;
		lat: number;
		lon: number;
		published: boolean;
		userId?: string | null;
		slug?: string | null;
		date: any;
		photoId?: string | null;
		siteMetrics: { __typename?: 'SiteMetric'; photoCount: number; diveCount: number };
		photo?: {
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		} | null;
		latestPhotos: Array<{
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		latestDives: Array<{
			__typename?: 'Dive';
			id: string;
			userId: string;
			date: any;
			depth: number;
			duration: number;
			number: number;
			hasMetrics: boolean;
			summary: string;
			likes: number;
			liked: boolean;
			numComments: number;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			latestPhotos: Array<{
				__typename?: 'Photo';
				id: string;
				userId: string;
				filename: string;
				date?: any | null;
				size: number;
				width: number;
				height: number;
				likes: number;
				liked: boolean;
				dive?: {
					__typename?: 'Dive';
					id: string;
					date: any;
					number: number;
					numComments: number;
					user: {
						__typename?: 'PublicUserInfo';
						id: string;
						username: string;
						displayName?: string | null;
					};
					diveSite?: {
						__typename?: 'DiveSite';
						name: string;
						id: string;
						slug?: string | null;
					} | null;
				} | null;
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
				sealife?: {
					__typename?: 'Sealife';
					id: string;
					name: string;
					scientificName?: string | null;
					summary: string;
					slug?: string | null;
					photoId?: string | null;
				} | null;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
			}>;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		references: Array<{
			__typename?: 'OgReference';
			id: string;
			url: string;
			title: string;
			description: string;
			imageUrl?: string | null;
			lastFetched: any;
		}>;
	};
};

export type AddFeedbackMutationVariables = Exact<{
	feedback: Scalars['String']['input'];
}>;

export type AddFeedbackMutation = {
	__typename?: 'Mutation';
	addFeedback: { __typename?: 'Feedback'; id: string };
};

export type NewReferenceMutationVariables = Exact<{
	url: Scalars['String']['input'];
	sealifeId?: InputMaybe<Scalars['UUID']['input']>;
	diveSiteId?: InputMaybe<Scalars['UUID']['input']>;
}>;

export type NewReferenceMutation = {
	__typename?: 'Mutation';
	newReference: {
		__typename?: 'OgReference';
		id: string;
		url: string;
		title: string;
		description: string;
		imageUrl?: string | null;
		lastFetched: any;
	};
};

export type AddSealifeMutationVariables = Exact<{
	sealife: CreateSealife;
}>;

export type AddSealifeMutation = {
	__typename?: 'Mutation';
	newSealife: {
		__typename?: 'Sealife';
		id: string;
		name: string;
		summary: string;
		scientificName?: string | null;
		description: string;
		slug?: string | null;
		date: any;
		categoryMap: any;
		hideLocation: boolean;
		photoId?: string | null;
		photo?: {
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		} | null;
		latestPhotos: Array<{
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		references: Array<{
			__typename?: 'OgReference';
			id: string;
			url: string;
			title: string;
			description: string;
			imageUrl?: string | null;
			lastFetched: any;
		}>;
	};
};

export type AddCommentMutationVariables = Exact<{
	diveId: Scalars['UUID']['input'];
	description: Scalars['String']['input'];
}>;

export type AddCommentMutation = {
	__typename?: 'Mutation';
	newComment: {
		__typename?: 'DiveComment';
		id: string;
		diveId: string;
		date: any;
		description: string;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	};
};

export type RemoveCommentMutationVariables = Exact<{
	commentId: Scalars['UUID']['input'];
}>;

export type RemoveCommentMutation = { __typename?: 'Mutation'; removeComment: boolean };

export type DeleteUserMutationVariables = Exact<{
	password: Scalars['String']['input'];
}>;

export type DeleteUserMutation = { __typename?: 'Mutation'; deleteUser: boolean };

export type FbLoginUserMutationVariables = Exact<{
	redirectUri: Scalars['String']['input'];
	code: Scalars['String']['input'];
}>;

export type FbLoginUserMutation = {
	__typename?: 'Mutation';
	fbLogin: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		username: string;
		level: UserLevel;
		token: string;
	};
};

export type FbRegisterUserMutationVariables = Exact<{
	username: Scalars['String']['input'];
	redirectUri: Scalars['String']['input'];
	code: Scalars['String']['input'];
}>;

export type FbRegisterUserMutation = {
	__typename?: 'Mutation';
	fbRegisterUser: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		username: string;
		level: UserLevel;
		token: string;
	};
};

export type LikeDiveMutationVariables = Exact<{
	diveId: Scalars['UUID']['input'];
}>;

export type LikeDiveMutation = { __typename?: 'Mutation'; likeDive: boolean };

export type UnlikeDiveMutationVariables = Exact<{
	diveId: Scalars['UUID']['input'];
}>;

export type UnlikeDiveMutation = { __typename?: 'Mutation'; unlikeDive: boolean };

export type LikePhotoMutationVariables = Exact<{
	photoId: Scalars['UUID']['input'];
}>;

export type LikePhotoMutation = { __typename?: 'Mutation'; likePhoto: boolean };

export type UnlikePhotoMutationVariables = Exact<{
	photoId: Scalars['UUID']['input'];
}>;

export type UnlikePhotoMutation = { __typename?: 'Mutation'; unlikePhoto: boolean };

export type LoginUserMutationVariables = Exact<{
	email: Scalars['String']['input'];
	password: Scalars['String']['input'];
}>;

export type LoginUserMutation = {
	__typename?: 'Mutation';
	login: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		username: string;
		level: UserLevel;
		token: string;
	};
};

export type UpdateSettingsMutationVariables = Exact<{
	displayName?: InputMaybe<Scalars['String']['input']>;
	watermarkLocation: OverlayLocation;
	copyrightLocation?: InputMaybe<OverlayLocation>;
	description: Scalars['String']['input'];
	photoId?: InputMaybe<Scalars['UUID']['input']>;
}>;

export type UpdateSettingsMutation = {
	__typename?: 'Mutation';
	updateSettings?: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		level: UserLevel;
		username: string;
		displayName?: string | null;
		watermarkLocation: OverlayLocation;
		copyrightLocation?: OverlayLocation | null;
		description: string;
		photoId?: string | null;
	} | null;
};

export type MergeDiveSitesMutationVariables = Exact<{
	fromId: Scalars['UUID']['input'];
	toId: Scalars['UUID']['input'];
}>;

export type MergeDiveSitesMutation = { __typename?: 'Mutation'; mergeDiveSites: boolean };

export type RequestResetTokenMutationVariables = Exact<{
	email: Scalars['String']['input'];
}>;

export type RequestResetTokenMutation = { __typename?: 'Mutation'; requestResetToken: boolean };

export type ResetPasswordMutationVariables = Exact<{
	email: Scalars['String']['input'];
	newPassword: Scalars['String']['input'];
	token: Scalars['UUID']['input'];
}>;

export type ResetPasswordMutation = {
	__typename?: 'Mutation';
	resetPassword: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		username: string;
		level: UserLevel;
		token: string;
	};
};

export type ChangePasswordMutationVariables = Exact<{
	oldPassword: Scalars['String']['input'];
	newPassword: Scalars['String']['input'];
}>;

export type ChangePasswordMutation = { __typename?: 'Mutation'; changePassword: boolean };

export type NewRegionMutationVariables = Exact<{
	region: CreateRegion;
}>;

export type NewRegionMutation = {
	__typename?: 'Mutation';
	newRegion: {
		__typename?: 'Region';
		id: string;
		name: string;
		latMin: number;
		lonMin: number;
		latMax: number;
		lonMax: number;
		slug: string;
	};
};

export type RemoveRegionMutationVariables = Exact<{
	id: Scalars['UUID']['input'];
}>;

export type RemoveRegionMutation = { __typename?: 'Mutation'; removeRegion: boolean };

export type RegisterUserMutationVariables = Exact<{
	username: Scalars['String']['input'];
	email: Scalars['String']['input'];
	password: Scalars['String']['input'];
}>;

export type RegisterUserMutation = { __typename?: 'Mutation'; registerUser: boolean };

export type RemoveDiveMutationVariables = Exact<{
	id: Scalars['UUID']['input'];
}>;

export type RemoveDiveMutation = { __typename?: 'Mutation'; removeDive: boolean };

export type RemoveDiveSiteMutationVariables = Exact<{
	id: Scalars['UUID']['input'];
}>;

export type RemoveDiveSiteMutation = { __typename?: 'Mutation'; removeDiveSite: boolean };

export type RemovePhotoMutationVariables = Exact<{
	id: Scalars['UUID']['input'];
}>;

export type RemovePhotoMutation = { __typename?: 'Mutation'; removePhoto: boolean };

export type RemoveReferenceMutationVariables = Exact<{
	id: Scalars['UUID']['input'];
}>;

export type RemoveReferenceMutation = { __typename?: 'Mutation'; removeReference: boolean };

export type RemoveSealifeMutationVariables = Exact<{
	id: Scalars['UUID']['input'];
}>;

export type RemoveSealifeMutation = { __typename?: 'Mutation'; removeSealife: boolean };

export type SyncSubsurfaceMutationVariables = Exact<{
	email: Scalars['String']['input'];
	password: Scalars['String']['input'];
}>;

export type SyncSubsurfaceMutation = { __typename?: 'Mutation'; syncSubsurface: boolean };

export type UpdatePhotoMutationVariables = Exact<{
	photo: CreatePhoto;
}>;

export type UpdatePhotoMutation = {
	__typename?: 'Mutation';
	updatePhoto: {
		__typename?: 'Photo';
		id: string;
		userId: string;
		filename: string;
		date?: any | null;
		size: number;
		width: number;
		height: number;
		likes: number;
		liked: boolean;
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			number: number;
			numComments: number;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		} | null;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		sealife?: {
			__typename?: 'Sealife';
			id: string;
			name: string;
			scientificName?: string | null;
			summary: string;
			slug?: string | null;
			photoId?: string | null;
		} | null;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	};
};

export type VerifyEmailMutationVariables = Exact<{
	email: Scalars['String']['input'];
	token: Scalars['UUID']['input'];
}>;

export type VerifyEmailMutation = {
	__typename?: 'Mutation';
	verifyEmail: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		username: string;
		level: UserLevel;
		token: string;
	};
};

export type GetCategoriesQueryVariables = Exact<{ [key: string]: never }>;

export type GetCategoriesQuery = {
	__typename?: 'Query';
	categories: Array<{
		__typename?: 'Category';
		id: string;
		name: string;
		values: Array<{ __typename?: 'CategoryValue'; id: string; categoryId: string; value: string }>;
	}>;
};

export type GetCategoryValuesQueryVariables = Exact<{ [key: string]: never }>;

export type GetCategoryValuesQuery = {
	__typename?: 'Query';
	categoryValues: Array<{
		__typename?: 'CategoryValue';
		id: string;
		categoryId: string;
		value: string;
	}>;
};

export type GetCurrentUserQueryVariables = Exact<{ [key: string]: never }>;

export type GetCurrentUserQuery = {
	__typename?: 'Query';
	currentUser?: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		level: UserLevel;
		username: string;
		displayName?: string | null;
		watermarkLocation: OverlayLocation;
		copyrightLocation?: OverlayLocation | null;
		description: string;
		photoId?: string | null;
	} | null;
};

export type FbAppIdQueryVariables = Exact<{ [key: string]: never }>;

export type FbAppIdQuery = { __typename?: 'Query'; fbAppId: string };

export type GetFeedbackQueryVariables = Exact<{
	id?: InputMaybe<Scalars['UUID']['input']>;
}>;

export type GetFeedbackQuery = {
	__typename?: 'Query';
	feedback: Array<{
		__typename?: 'Feedback';
		id: string;
		date: any;
		feedback: string;
		user: {
			__typename?: 'UserInfo';
			id: string;
			email?: string | null;
			level: UserLevel;
			username: string;
		};
	}>;
};

export type FrontPageQueryVariables = Exact<{ [key: string]: never }>;

export type FrontPageQuery = {
	__typename?: 'Query';
	popularDiveSites: Array<{
		__typename?: 'DiveSite';
		id: string;
		name: string;
		summary: string;
		slug?: string | null;
		lat: number;
		lon: number;
		photoId?: string | null;
		siteMetrics: { __typename?: 'SiteMetric'; photoCount: number; diveCount: number };
	}>;
	recentDives: Array<{
		__typename?: 'Dive';
		id: string;
		userId: string;
		date: any;
		depth: number;
		duration: number;
		number: number;
		hasMetrics: boolean;
		summary: string;
		likes: number;
		liked: boolean;
		numComments: number;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		latestPhotos: Array<{
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	}>;
};

export type GetDiveQueryVariables = Exact<{
	id: Scalars['UUID']['input'];
}>;

export type GetDiveQuery = {
	__typename?: 'Query';
	siteUrl: string;
	dives: Array<{
		__typename?: 'Dive';
		id: string;
		userId: string;
		date: any;
		depth: number;
		duration: number;
		number: number;
		hasMetrics: boolean;
		description: string;
		summary: string;
		published: boolean;
		likes: number;
		liked: boolean;
		numComments: number;
		diveSiteId?: string | null;
		comments: Array<{
			__typename?: 'DiveComment';
			id: string;
			diveId: string;
			date: any;
			description: string;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
		latestPhotos: Array<{
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
	}>;
};

export type GetDiveSitesQueryVariables = Exact<{
	id?: InputMaybe<Scalars['UUID']['input']>;
	name?: InputMaybe<Scalars['String']['input']>;
	maxDepth?: InputMaybe<Scalars['Float']['input']>;
	slug?: InputMaybe<Scalars['String']['input']>;
}>;

export type GetDiveSitesQuery = {
	__typename?: 'Query';
	siteUrl: string;
	diveSites: Array<{
		__typename?: 'DiveSite';
		id: string;
		name: string;
		description: string;
		summary: string;
		access: string;
		difficulty: Difficulty;
		depth: number;
		lat: number;
		lon: number;
		published: boolean;
		userId?: string | null;
		slug?: string | null;
		date: any;
		photoId?: string | null;
		siteMetrics: { __typename?: 'SiteMetric'; photoCount: number; diveCount: number };
		photo?: {
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		} | null;
		latestPhotos: Array<{
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		latestDives: Array<{
			__typename?: 'Dive';
			id: string;
			userId: string;
			date: any;
			depth: number;
			duration: number;
			number: number;
			hasMetrics: boolean;
			summary: string;
			likes: number;
			liked: boolean;
			numComments: number;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			latestPhotos: Array<{
				__typename?: 'Photo';
				id: string;
				userId: string;
				filename: string;
				date?: any | null;
				size: number;
				width: number;
				height: number;
				likes: number;
				liked: boolean;
				dive?: {
					__typename?: 'Dive';
					id: string;
					date: any;
					number: number;
					numComments: number;
					user: {
						__typename?: 'PublicUserInfo';
						id: string;
						username: string;
						displayName?: string | null;
					};
					diveSite?: {
						__typename?: 'DiveSite';
						name: string;
						id: string;
						slug?: string | null;
					} | null;
				} | null;
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
				sealife?: {
					__typename?: 'Sealife';
					id: string;
					name: string;
					scientificName?: string | null;
					summary: string;
					slug?: string | null;
					photoId?: string | null;
				} | null;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
			}>;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		references: Array<{
			__typename?: 'OgReference';
			id: string;
			url: string;
			title: string;
			description: string;
			imageUrl?: string | null;
			lastFetched: any;
		}>;
	}>;
	regions: Array<{
		__typename?: 'Region';
		id: string;
		name: string;
		latMin: number;
		lonMin: number;
		latMax: number;
		lonMax: number;
		slug: string;
	}>;
};

export type GetDiveSitesSummaryMetricsQueryVariables = Exact<{
	id?: InputMaybe<Scalars['UUID']['input']>;
	name?: InputMaybe<Scalars['String']['input']>;
	maxDepth?: InputMaybe<Scalars['Float']['input']>;
	slug?: InputMaybe<Scalars['String']['input']>;
}>;

export type GetDiveSitesSummaryMetricsQuery = {
	__typename?: 'Query';
	diveSites: Array<{
		__typename?: 'DiveSite';
		id: string;
		name: string;
		summary: string;
		slug?: string | null;
		lat: number;
		lon: number;
		photoId?: string | null;
		siteMetrics: { __typename?: 'SiteMetric'; photoCount: number; diveCount: number };
	}>;
};

export type GetDivesQueryVariables = Exact<{
	diveSite?: InputMaybe<Scalars['UUID']['input']>;
	username?: InputMaybe<Scalars['String']['input']>;
	offset?: InputMaybe<Scalars['Int']['input']>;
}>;

export type GetDivesQuery = {
	__typename?: 'Query';
	dives: Array<{
		__typename?: 'Dive';
		id: string;
		userId: string;
		date: any;
		depth: number;
		duration: number;
		number: number;
		hasMetrics: boolean;
		summary: string;
		likes: number;
		liked: boolean;
		numComments: number;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		latestPhotos: Array<{
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	}>;
};

export type GetPhotosQueryVariables = Exact<{
	id?: InputMaybe<Scalars['UUID']['input']>;
	userId?: InputMaybe<Scalars['UUID']['input']>;
	username?: InputMaybe<Scalars['String']['input']>;
	diveSite?: InputMaybe<Scalars['UUID']['input']>;
	dive?: InputMaybe<Scalars['UUID']['input']>;
	sealifeId?: InputMaybe<Scalars['UUID']['input']>;
	offset?: InputMaybe<Scalars['Int']['input']>;
	orderByUpload?: InputMaybe<Scalars['Boolean']['input']>;
}>;

export type GetPhotosQuery = {
	__typename?: 'Query';
	siteUrl: string;
	photos: Array<{
		__typename?: 'Photo';
		id: string;
		userId: string;
		filename: string;
		date?: any | null;
		size: number;
		width: number;
		height: number;
		likes: number;
		liked: boolean;
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			number: number;
			numComments: number;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		} | null;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
		sealife?: {
			__typename?: 'Sealife';
			id: string;
			name: string;
			scientificName?: string | null;
			summary: string;
			slug?: string | null;
			photoId?: string | null;
		} | null;
		user: {
			__typename?: 'PublicUserInfo';
			id: string;
			username: string;
			displayName?: string | null;
		};
	}>;
};

export type GetSealifeSummaryQueryVariables = Exact<{
	id?: InputMaybe<Scalars['UUID']['input']>;
	name?: InputMaybe<Scalars['String']['input']>;
	scientificName?: InputMaybe<Scalars['String']['input']>;
	slug?: InputMaybe<Scalars['String']['input']>;
	categoryValues?: InputMaybe<Array<Scalars['UUID']['input']> | Scalars['UUID']['input']>;
}>;

export type GetSealifeSummaryQuery = {
	__typename?: 'Query';
	sealife: Array<{
		__typename?: 'Sealife';
		id: string;
		name: string;
		scientificName?: string | null;
		summary: string;
		slug?: string | null;
		photoId?: string | null;
	}>;
};

export type GetSealifeQueryVariables = Exact<{
	id?: InputMaybe<Scalars['UUID']['input']>;
	name?: InputMaybe<Scalars['String']['input']>;
	scientificName?: InputMaybe<Scalars['String']['input']>;
	slug?: InputMaybe<Scalars['String']['input']>;
}>;

export type GetSealifeQuery = {
	__typename?: 'Query';
	siteUrl: string;
	sealife: Array<{
		__typename?: 'Sealife';
		id: string;
		name: string;
		summary: string;
		scientificName?: string | null;
		description: string;
		slug?: string | null;
		date: any;
		categoryMap: any;
		hideLocation: boolean;
		photoId?: string | null;
		photo?: {
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		} | null;
		latestPhotos: Array<{
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		references: Array<{
			__typename?: 'OgReference';
			id: string;
			url: string;
			title: string;
			description: string;
			imageUrl?: string | null;
			lastFetched: any;
		}>;
	}>;
};

export type GetUserQueryVariables = Exact<{
	username: Scalars['String']['input'];
}>;

export type GetUserQuery = {
	__typename?: 'Query';
	siteUrl: string;
	user: {
		__typename?: 'PublicUserInfo';
		id: string;
		username: string;
		displayName?: string | null;
		description: string;
		photoId?: string | null;
		photoCount: number;
		diveCount: number;
		latestPhotos: Array<{
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			likes: number;
			liked: boolean;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				number: number;
				numComments: number;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
			} | null;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			sealife?: {
				__typename?: 'Sealife';
				id: string;
				name: string;
				scientificName?: string | null;
				summary: string;
				slug?: string | null;
				photoId?: string | null;
			} | null;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
		latestDives: Array<{
			__typename?: 'Dive';
			id: string;
			userId: string;
			date: any;
			depth: number;
			duration: number;
			number: number;
			hasMetrics: boolean;
			summary: string;
			likes: number;
			liked: boolean;
			numComments: number;
			diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
			latestPhotos: Array<{
				__typename?: 'Photo';
				id: string;
				userId: string;
				filename: string;
				date?: any | null;
				size: number;
				width: number;
				height: number;
				likes: number;
				liked: boolean;
				dive?: {
					__typename?: 'Dive';
					id: string;
					date: any;
					number: number;
					numComments: number;
					user: {
						__typename?: 'PublicUserInfo';
						id: string;
						username: string;
						displayName?: string | null;
					};
					diveSite?: {
						__typename?: 'DiveSite';
						name: string;
						id: string;
						slug?: string | null;
					} | null;
				} | null;
				diveSite?: {
					__typename?: 'DiveSite';
					name: string;
					id: string;
					slug?: string | null;
				} | null;
				sealife?: {
					__typename?: 'Sealife';
					id: string;
					name: string;
					scientificName?: string | null;
					summary: string;
					slug?: string | null;
					photoId?: string | null;
				} | null;
				user: {
					__typename?: 'PublicUserInfo';
					id: string;
					username: string;
					displayName?: string | null;
				};
			}>;
			user: {
				__typename?: 'PublicUserInfo';
				id: string;
				username: string;
				displayName?: string | null;
			};
		}>;
	};
};

export type GetRegionsQueryVariables = Exact<{ [key: string]: never }>;

export type GetRegionsQuery = {
	__typename?: 'Query';
	regions: Array<{
		__typename?: 'Region';
		id: string;
		name: string;
		latMin: number;
		lonMin: number;
		latMax: number;
		lonMax: number;
		slug: string;
	}>;
};

export type SearchQueryVariables = Exact<{
	query: Scalars['String']['input'];
	offset?: InputMaybe<Scalars['Int']['input']>;
}>;

export type SearchQuery = {
	__typename?: 'Query';
	search: Array<{
		__typename?: 'SearchResult';
		id: string;
		kind: SearchResultKind;
		slug: string;
		photoId?: string | null;
		name: string;
		scientificName?: string | null;
		summary: string;
	}>;
};

export const CategoryValueNodeFragmentDoc = gql`
	fragment CategoryValueNode on CategoryValue {
		id
		categoryId
		value
	}
`;
export const CategoryNodeFragmentDoc = gql`
	fragment CategoryNode on Category {
		id
		name
		values {
			...CategoryValueNode
		}
	}
`;
export const UserSummaryFragmentDoc = gql`
	fragment UserSummary on PublicUserInfo {
		id
		username
		displayName
	}
`;
export const CommentFragmentDoc = gql`
	fragment Comment on DiveComment {
		id
		diveId
		user {
			...UserSummary
		}
		date
		description
	}
`;
export const SiteSummaryFragmentDoc = gql`
	fragment SiteSummary on DiveSite {
		name
		id
		slug
	}
`;
export const DiveSummaryFragmentDoc = gql`
	fragment DiveSummary on Dive {
		id
		date
		number
		numComments
		user {
			...UserSummary
		}
		diveSite {
			...SiteSummary
		}
	}
`;
export const SealifeSummaryFragmentDoc = gql`
	fragment SealifeSummary on Sealife {
		id
		name
		scientificName
		summary
		slug
		photoId
	}
`;
export const PhotoSummaryFragmentDoc = gql`
	fragment PhotoSummary on Photo {
		id
		userId
		filename
		date
		size
		width
		height
		likes
		liked
		dive {
			...DiveSummary
		}
		diveSite {
			...SiteSummary
		}
		sealife {
			...SealifeSummary
		}
		user {
			...UserSummary
		}
	}
`;
export const DiveNodeFragmentDoc = gql`
	fragment DiveNode on Dive {
		id
		userId
		date
		depth
		duration
		number
		hasMetrics
		description
		summary
		published
		likes
		liked
		numComments
		comments {
			...Comment
		}
		user {
			...UserSummary
		}
		latestPhotos {
			...PhotoSummary
		}
		diveSiteId
		diveSite {
			...SiteSummary
		}
	}
`;
export const SiteMetricNodeFragmentDoc = gql`
	fragment SiteMetricNode on SiteMetric {
		photoCount
		diveCount
	}
`;
export const DiveWithMetricsFragmentDoc = gql`
	fragment DiveWithMetrics on Dive {
		id
		userId
		date
		depth
		duration
		number
		hasMetrics
		summary
		likes
		liked
		numComments
		diveSite {
			...SiteSummary
		}
		latestPhotos {
			...PhotoSummary
		}
		user {
			...UserSummary
		}
	}
`;
export const ReferenceFragmentDoc = gql`
	fragment Reference on OgReference {
		id
		url
		title
		description
		imageUrl
		lastFetched
	}
`;
export const SiteFragmentDoc = gql`
	fragment Site on DiveSite {
		id
		name
		description
		summary
		access
		difficulty
		depth
		lat
		lon
		published
		userId
		slug
		date
		siteMetrics {
			...SiteMetricNode
		}
		photoId
		photo {
			...PhotoSummary
		}
		latestPhotos {
			...PhotoSummary
		}
		latestDives {
			...DiveWithMetrics
		}
		references {
			...Reference
		}
	}
`;
export const SiteSummaryMetricsFragmentDoc = gql`
	fragment SiteSummaryMetrics on DiveSite {
		id
		name
		summary
		slug
		siteMetrics {
			...SiteMetricNode
		}
		lat
		lon
		photoId
	}
`;
export const FeedbackNodeFragmentDoc = gql`
	fragment FeedbackNode on Feedback {
		id
		user {
			id
			email
			level
			username
		}
		date
		feedback
	}
`;
export const RegionNodeFragmentDoc = gql`
	fragment RegionNode on Region {
		id
		name
		latMin
		lonMin
		latMax
		lonMax
		slug
	}
`;
export const SealifeNodeFragmentDoc = gql`
	fragment SealifeNode on Sealife {
		id
		name
		summary
		scientificName
		description
		slug
		date
		categoryMap
		hideLocation
		photoId
		photo {
			...PhotoSummary
		}
		latestPhotos {
			...PhotoSummary
		}
		references {
			...Reference
		}
	}
`;
export const SearchResultNodeFragmentDoc = gql`
	fragment SearchResultNode on SearchResult {
		id
		kind
		slug
		photoId
		name
		scientificName
		summary
	}
`;
export const CurrentUserFragmentDoc = gql`
	fragment CurrentUser on LoginResponse {
		id
		email
		level
		username
		displayName
		watermarkLocation
		copyrightLocation
		description
		photoId
	}
`;
export const CurrentUserTokenFragmentDoc = gql`
	fragment CurrentUserToken on LoginResponse {
		id
		email
		username
		level
		token
	}
`;
export const UserInfoFragmentDoc = gql`
	fragment UserInfo on PublicUserInfo {
		id
		username
		displayName
		description
		photoId
		photoCount
		diveCount
		latestPhotos {
			...PhotoSummary
		}
		latestDives {
			...DiveWithMetrics
		}
	}
`;
export const AddDiveDocument = gql`
	mutation addDive($dive: CreateDive!) {
		newDive(dive: $dive) {
			...DiveNode
		}
	}
	${DiveNodeFragmentDoc}
	${CommentFragmentDoc}
	${UserSummaryFragmentDoc}
	${PhotoSummaryFragmentDoc}
	${DiveSummaryFragmentDoc}
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
`;
export const AddDiveSiteDocument = gql`
	mutation addDiveSite($site: CreateDiveSite!) {
		newDiveSite(site: $site) {
			...Site
		}
	}
	${SiteFragmentDoc}
	${SiteMetricNodeFragmentDoc}
	${PhotoSummaryFragmentDoc}
	${DiveSummaryFragmentDoc}
	${UserSummaryFragmentDoc}
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
	${DiveWithMetricsFragmentDoc}
	${ReferenceFragmentDoc}
`;
export const AddFeedbackDocument = gql`
	mutation addFeedback($feedback: String!) {
		addFeedback(feedback: $feedback) {
			id
		}
	}
`;
export const NewReferenceDocument = gql`
	mutation newReference($url: String!, $sealifeId: UUID, $diveSiteId: UUID) {
		newReference(url: $url, sealifeId: $sealifeId, diveSiteId: $diveSiteId) {
			...Reference
		}
	}
	${ReferenceFragmentDoc}
`;
export const AddSealifeDocument = gql`
	mutation addSealife($sealife: CreateSealife!) {
		newSealife(sealife: $sealife) {
			...SealifeNode
		}
	}
	${SealifeNodeFragmentDoc}
	${PhotoSummaryFragmentDoc}
	${DiveSummaryFragmentDoc}
	${UserSummaryFragmentDoc}
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
	${ReferenceFragmentDoc}
`;
export const AddCommentDocument = gql`
	mutation addComment($diveId: UUID!, $description: String!) {
		newComment(comment: { description: $description, diveId: $diveId }) {
			...Comment
		}
	}
	${CommentFragmentDoc}
	${UserSummaryFragmentDoc}
`;
export const RemoveCommentDocument = gql`
	mutation removeComment($commentId: UUID!) {
		removeComment(id: $commentId)
	}
`;
export const DeleteUserDocument = gql`
	mutation deleteUser($password: String!) {
		deleteUser(password: $password)
	}
`;
export const FbLoginUserDocument = gql`
	mutation fbLoginUser($redirectUri: String!, $code: String!) {
		fbLogin(redirectUri: $redirectUri, code: $code) {
			...CurrentUserToken
		}
	}
	${CurrentUserTokenFragmentDoc}
`;
export const FbRegisterUserDocument = gql`
	mutation fbRegisterUser($username: String!, $redirectUri: String!, $code: String!) {
		fbRegisterUser(username: $username, redirectUri: $redirectUri, code: $code) {
			...CurrentUserToken
		}
	}
	${CurrentUserTokenFragmentDoc}
`;
export const LikeDiveDocument = gql`
	mutation likeDive($diveId: UUID!) {
		likeDive(diveId: $diveId)
	}
`;
export const UnlikeDiveDocument = gql`
	mutation unlikeDive($diveId: UUID!) {
		unlikeDive(diveId: $diveId)
	}
`;
export const LikePhotoDocument = gql`
	mutation likePhoto($photoId: UUID!) {
		likePhoto(photoId: $photoId)
	}
`;
export const UnlikePhotoDocument = gql`
	mutation unlikePhoto($photoId: UUID!) {
		unlikePhoto(photoId: $photoId)
	}
`;
export const LoginUserDocument = gql`
	mutation loginUser($email: String!, $password: String!) {
		login(email: $email, password: $password) {
			...CurrentUserToken
		}
	}
	${CurrentUserTokenFragmentDoc}
`;
export const UpdateSettingsDocument = gql`
	mutation updateSettings(
		$displayName: String
		$watermarkLocation: OverlayLocation!
		$copyrightLocation: OverlayLocation
		$description: String!
		$photoId: UUID
	) {
		updateSettings(
			displayName: $displayName
			watermarkLocation: $watermarkLocation
			copyrightLocation: $copyrightLocation
			description: $description
			photoId: $photoId
		) {
			...CurrentUser
		}
	}
	${CurrentUserFragmentDoc}
`;
export const MergeDiveSitesDocument = gql`
	mutation mergeDiveSites($fromId: UUID!, $toId: UUID!) {
		mergeDiveSites(fromId: $fromId, toId: $toId)
	}
`;
export const RequestResetTokenDocument = gql`
	mutation requestResetToken($email: String!) {
		requestResetToken(email: $email)
	}
`;
export const ResetPasswordDocument = gql`
	mutation resetPassword($email: String!, $newPassword: String!, $token: UUID!) {
		resetPassword(email: $email, newPassword: $newPassword, token: $token) {
			...CurrentUserToken
		}
	}
	${CurrentUserTokenFragmentDoc}
`;
export const ChangePasswordDocument = gql`
	mutation changePassword($oldPassword: String!, $newPassword: String!) {
		changePassword(oldPassword: $oldPassword, newPassword: $newPassword)
	}
`;
export const NewRegionDocument = gql`
	mutation newRegion($region: CreateRegion!) {
		newRegion(region: $region) {
			...RegionNode
		}
	}
	${RegionNodeFragmentDoc}
`;
export const RemoveRegionDocument = gql`
	mutation removeRegion($id: UUID!) {
		removeRegion(id: $id)
	}
`;
export const RegisterUserDocument = gql`
	mutation registerUser($username: String!, $email: String!, $password: String!) {
		registerUser(username: $username, email: $email, password: $password)
	}
`;
export const RemoveDiveDocument = gql`
	mutation removeDive($id: UUID!) {
		removeDive(id: $id)
	}
`;
export const RemoveDiveSiteDocument = gql`
	mutation removeDiveSite($id: UUID!) {
		removeDiveSite(id: $id)
	}
`;
export const RemovePhotoDocument = gql`
	mutation removePhoto($id: UUID!) {
		removePhoto(id: $id)
	}
`;
export const RemoveReferenceDocument = gql`
	mutation removeReference($id: UUID!) {
		removeReference(id: $id)
	}
`;
export const RemoveSealifeDocument = gql`
	mutation removeSealife($id: UUID!) {
		removeSealife(id: $id)
	}
`;
export const SyncSubsurfaceDocument = gql`
	mutation syncSubsurface($email: String!, $password: String!) {
		syncSubsurface(email: $email, password: $password)
	}
`;
export const UpdatePhotoDocument = gql`
	mutation updatePhoto($photo: CreatePhoto!) {
		updatePhoto(photo: $photo) {
			...PhotoSummary
		}
	}
	${PhotoSummaryFragmentDoc}
	${DiveSummaryFragmentDoc}
	${UserSummaryFragmentDoc}
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
`;
export const VerifyEmailDocument = gql`
	mutation verifyEmail($email: String!, $token: UUID!) {
		verifyEmail(email: $email, token: $token) {
			...CurrentUserToken
		}
	}
	${CurrentUserTokenFragmentDoc}
`;
export const GetCategoriesDocument = gql`
	query getCategories {
		categories {
			...CategoryNode
		}
	}
	${CategoryNodeFragmentDoc}
	${CategoryValueNodeFragmentDoc}
`;
export const GetCategoryValuesDocument = gql`
	query getCategoryValues {
		categoryValues {
			...CategoryValueNode
		}
	}
	${CategoryValueNodeFragmentDoc}
`;
export const GetCurrentUserDocument = gql`
	query getCurrentUser {
		currentUser {
			...CurrentUser
		}
	}
	${CurrentUserFragmentDoc}
`;
export const FbAppIdDocument = gql`
	query fbAppId {
		fbAppId
	}
`;
export const GetFeedbackDocument = gql`
	query getFeedback($id: UUID) {
		feedback(id: $id) {
			...FeedbackNode
		}
	}
	${FeedbackNodeFragmentDoc}
`;
export const FrontPageDocument = gql`
	query frontPage {
		popularDiveSites {
			...SiteSummaryMetrics
		}
		recentDives {
			...DiveWithMetrics
		}
	}
	${SiteSummaryMetricsFragmentDoc}
	${SiteMetricNodeFragmentDoc}
	${DiveWithMetricsFragmentDoc}
	${SiteSummaryFragmentDoc}
	${PhotoSummaryFragmentDoc}
	${DiveSummaryFragmentDoc}
	${UserSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
`;
export const GetDiveDocument = gql`
	query getDive($id: UUID!) {
		dives(id: $id) {
			...DiveNode
		}
		siteUrl
	}
	${DiveNodeFragmentDoc}
	${CommentFragmentDoc}
	${UserSummaryFragmentDoc}
	${PhotoSummaryFragmentDoc}
	${DiveSummaryFragmentDoc}
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
`;
export const GetDiveSitesDocument = gql`
	query getDiveSites($id: UUID, $name: String, $maxDepth: Float, $slug: String) {
		diveSites(id: $id, name: $name, maxDepth: $maxDepth, slug: $slug) {
			...Site
		}
		siteUrl
		regions {
			...RegionNode
		}
	}
	${SiteFragmentDoc}
	${SiteMetricNodeFragmentDoc}
	${PhotoSummaryFragmentDoc}
	${DiveSummaryFragmentDoc}
	${UserSummaryFragmentDoc}
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
	${DiveWithMetricsFragmentDoc}
	${ReferenceFragmentDoc}
	${RegionNodeFragmentDoc}
`;
export const GetDiveSitesSummaryMetricsDocument = gql`
	query getDiveSitesSummaryMetrics($id: UUID, $name: String, $maxDepth: Float, $slug: String) {
		diveSites(id: $id, name: $name, maxDepth: $maxDepth, slug: $slug) {
			...SiteSummaryMetrics
		}
	}
	${SiteSummaryMetricsFragmentDoc}
	${SiteMetricNodeFragmentDoc}
`;
export const GetDivesDocument = gql`
	query getDives($diveSite: UUID, $username: String, $offset: Int) {
		dives(diveSite: $diveSite, username: $username, offset: $offset) {
			...DiveWithMetrics
		}
	}
	${DiveWithMetricsFragmentDoc}
	${SiteSummaryFragmentDoc}
	${PhotoSummaryFragmentDoc}
	${DiveSummaryFragmentDoc}
	${UserSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
`;
export const GetPhotosDocument = gql`
	query getPhotos(
		$id: UUID
		$userId: UUID
		$username: String
		$diveSite: UUID
		$dive: UUID
		$sealifeId: UUID
		$offset: Int
		$orderByUpload: Boolean
	) {
		photos(
			id: $id
			userId: $userId
			username: $username
			diveSite: $diveSite
			dive: $dive
			sealifeId: $sealifeId
			offset: $offset
			orderByUpload: $orderByUpload
		) {
			...PhotoSummary
		}
		siteUrl
	}
	${PhotoSummaryFragmentDoc}
	${DiveSummaryFragmentDoc}
	${UserSummaryFragmentDoc}
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
`;
export const GetSealifeSummaryDocument = gql`
	query getSealifeSummary(
		$id: UUID
		$name: String
		$scientificName: String
		$slug: String
		$categoryValues: [UUID!]
	) {
		sealife(
			id: $id
			name: $name
			scientificName: $scientificName
			slug: $slug
			categoryValues: $categoryValues
		) {
			...SealifeSummary
		}
	}
	${SealifeSummaryFragmentDoc}
`;
export const GetSealifeDocument = gql`
	query getSealife($id: UUID, $name: String, $scientificName: String, $slug: String) {
		sealife(id: $id, name: $name, scientificName: $scientificName, slug: $slug) {
			...SealifeNode
		}
		siteUrl
	}
	${SealifeNodeFragmentDoc}
	${PhotoSummaryFragmentDoc}
	${DiveSummaryFragmentDoc}
	${UserSummaryFragmentDoc}
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
	${ReferenceFragmentDoc}
`;
export const GetUserDocument = gql`
	query getUser($username: String!) {
		user(username: $username) {
			...UserInfo
		}
		siteUrl
	}
	${UserInfoFragmentDoc}
	${PhotoSummaryFragmentDoc}
	${DiveSummaryFragmentDoc}
	${UserSummaryFragmentDoc}
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
	${DiveWithMetricsFragmentDoc}
`;
export const GetRegionsDocument = gql`
	query getRegions {
		regions {
			...RegionNode
		}
	}
	${RegionNodeFragmentDoc}
`;
export const SearchDocument = gql`
	query search($query: String!, $offset: Int) {
		search(query: $query, offset: $offset) {
			...SearchResultNode
		}
	}
	${SearchResultNodeFragmentDoc}
`;

export type SdkFunctionWrapper = <T>(
	action: (requestHeaders?: Record<string, string>) => Promise<T>,
	operationName: string,
	operationType?: string
) => Promise<T>;

const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
	return {
		addDive(
			variables: AddDiveMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<AddDiveMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<AddDiveMutation>(AddDiveDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'addDive',
				'mutation'
			);
		},
		addDiveSite(
			variables: AddDiveSiteMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<AddDiveSiteMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<AddDiveSiteMutation>(AddDiveSiteDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'addDiveSite',
				'mutation'
			);
		},
		addFeedback(
			variables: AddFeedbackMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<AddFeedbackMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<AddFeedbackMutation>(AddFeedbackDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'addFeedback',
				'mutation'
			);
		},
		newReference(
			variables: NewReferenceMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<NewReferenceMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<NewReferenceMutation>(NewReferenceDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'newReference',
				'mutation'
			);
		},
		addSealife(
			variables: AddSealifeMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<AddSealifeMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<AddSealifeMutation>(AddSealifeDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'addSealife',
				'mutation'
			);
		},
		addComment(
			variables: AddCommentMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<AddCommentMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<AddCommentMutation>(AddCommentDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'addComment',
				'mutation'
			);
		},
		removeComment(
			variables: RemoveCommentMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<RemoveCommentMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<RemoveCommentMutation>(RemoveCommentDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'removeComment',
				'mutation'
			);
		},
		deleteUser(
			variables: DeleteUserMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<DeleteUserMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<DeleteUserMutation>(DeleteUserDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'deleteUser',
				'mutation'
			);
		},
		fbLoginUser(
			variables: FbLoginUserMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<FbLoginUserMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<FbLoginUserMutation>(FbLoginUserDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'fbLoginUser',
				'mutation'
			);
		},
		fbRegisterUser(
			variables: FbRegisterUserMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<FbRegisterUserMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<FbRegisterUserMutation>(FbRegisterUserDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'fbRegisterUser',
				'mutation'
			);
		},
		likeDive(
			variables: LikeDiveMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<LikeDiveMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<LikeDiveMutation>(LikeDiveDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'likeDive',
				'mutation'
			);
		},
		unlikeDive(
			variables: UnlikeDiveMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<UnlikeDiveMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<UnlikeDiveMutation>(UnlikeDiveDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'unlikeDive',
				'mutation'
			);
		},
		likePhoto(
			variables: LikePhotoMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<LikePhotoMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<LikePhotoMutation>(LikePhotoDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'likePhoto',
				'mutation'
			);
		},
		unlikePhoto(
			variables: UnlikePhotoMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<UnlikePhotoMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<UnlikePhotoMutation>(UnlikePhotoDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'unlikePhoto',
				'mutation'
			);
		},
		loginUser(
			variables: LoginUserMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<LoginUserMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<LoginUserMutation>(LoginUserDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'loginUser',
				'mutation'
			);
		},
		updateSettings(
			variables: UpdateSettingsMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<UpdateSettingsMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<UpdateSettingsMutation>(UpdateSettingsDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'updateSettings',
				'mutation'
			);
		},
		mergeDiveSites(
			variables: MergeDiveSitesMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<MergeDiveSitesMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<MergeDiveSitesMutation>(MergeDiveSitesDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'mergeDiveSites',
				'mutation'
			);
		},
		requestResetToken(
			variables: RequestResetTokenMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<RequestResetTokenMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<RequestResetTokenMutation>(RequestResetTokenDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'requestResetToken',
				'mutation'
			);
		},
		resetPassword(
			variables: ResetPasswordMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<ResetPasswordMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<ResetPasswordMutation>(ResetPasswordDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'resetPassword',
				'mutation'
			);
		},
		changePassword(
			variables: ChangePasswordMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<ChangePasswordMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<ChangePasswordMutation>(ChangePasswordDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'changePassword',
				'mutation'
			);
		},
		newRegion(
			variables: NewRegionMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<NewRegionMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<NewRegionMutation>(NewRegionDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'newRegion',
				'mutation'
			);
		},
		removeRegion(
			variables: RemoveRegionMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<RemoveRegionMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<RemoveRegionMutation>(RemoveRegionDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'removeRegion',
				'mutation'
			);
		},
		registerUser(
			variables: RegisterUserMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<RegisterUserMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<RegisterUserMutation>(RegisterUserDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'registerUser',
				'mutation'
			);
		},
		removeDive(
			variables: RemoveDiveMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<RemoveDiveMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<RemoveDiveMutation>(RemoveDiveDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'removeDive',
				'mutation'
			);
		},
		removeDiveSite(
			variables: RemoveDiveSiteMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<RemoveDiveSiteMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<RemoveDiveSiteMutation>(RemoveDiveSiteDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'removeDiveSite',
				'mutation'
			);
		},
		removePhoto(
			variables: RemovePhotoMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<RemovePhotoMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<RemovePhotoMutation>(RemovePhotoDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'removePhoto',
				'mutation'
			);
		},
		removeReference(
			variables: RemoveReferenceMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<RemoveReferenceMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<RemoveReferenceMutation>(RemoveReferenceDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'removeReference',
				'mutation'
			);
		},
		removeSealife(
			variables: RemoveSealifeMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<RemoveSealifeMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<RemoveSealifeMutation>(RemoveSealifeDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'removeSealife',
				'mutation'
			);
		},
		syncSubsurface(
			variables: SyncSubsurfaceMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<SyncSubsurfaceMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<SyncSubsurfaceMutation>(SyncSubsurfaceDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'syncSubsurface',
				'mutation'
			);
		},
		updatePhoto(
			variables: UpdatePhotoMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<UpdatePhotoMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<UpdatePhotoMutation>(UpdatePhotoDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'updatePhoto',
				'mutation'
			);
		},
		verifyEmail(
			variables: VerifyEmailMutationVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<VerifyEmailMutation> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<VerifyEmailMutation>(VerifyEmailDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'verifyEmail',
				'mutation'
			);
		},
		getCategories(
			variables?: GetCategoriesQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetCategoriesQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetCategoriesQuery>(GetCategoriesDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'getCategories',
				'query'
			);
		},
		getCategoryValues(
			variables?: GetCategoryValuesQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetCategoryValuesQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetCategoryValuesQuery>(GetCategoryValuesDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'getCategoryValues',
				'query'
			);
		},
		getCurrentUser(
			variables?: GetCurrentUserQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetCurrentUserQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetCurrentUserQuery>(GetCurrentUserDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'getCurrentUser',
				'query'
			);
		},
		fbAppId(
			variables?: FbAppIdQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<FbAppIdQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<FbAppIdQuery>(FbAppIdDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'fbAppId',
				'query'
			);
		},
		getFeedback(
			variables?: GetFeedbackQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetFeedbackQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetFeedbackQuery>(GetFeedbackDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'getFeedback',
				'query'
			);
		},
		frontPage(
			variables?: FrontPageQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<FrontPageQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<FrontPageQuery>(FrontPageDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'frontPage',
				'query'
			);
		},
		getDive(
			variables: GetDiveQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetDiveQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetDiveQuery>(GetDiveDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'getDive',
				'query'
			);
		},
		getDiveSites(
			variables?: GetDiveSitesQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetDiveSitesQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetDiveSitesQuery>(GetDiveSitesDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'getDiveSites',
				'query'
			);
		},
		getDiveSitesSummaryMetrics(
			variables?: GetDiveSitesSummaryMetricsQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetDiveSitesSummaryMetricsQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetDiveSitesSummaryMetricsQuery>(
						GetDiveSitesSummaryMetricsDocument,
						variables,
						{ ...requestHeaders, ...wrappedRequestHeaders }
					),
				'getDiveSitesSummaryMetrics',
				'query'
			);
		},
		getDives(
			variables?: GetDivesQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetDivesQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetDivesQuery>(GetDivesDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'getDives',
				'query'
			);
		},
		getPhotos(
			variables?: GetPhotosQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetPhotosQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetPhotosQuery>(GetPhotosDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'getPhotos',
				'query'
			);
		},
		getSealifeSummary(
			variables?: GetSealifeSummaryQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetSealifeSummaryQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetSealifeSummaryQuery>(GetSealifeSummaryDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'getSealifeSummary',
				'query'
			);
		},
		getSealife(
			variables?: GetSealifeQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetSealifeQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetSealifeQuery>(GetSealifeDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'getSealife',
				'query'
			);
		},
		getUser(
			variables: GetUserQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetUserQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetUserQuery>(GetUserDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'getUser',
				'query'
			);
		},
		getRegions(
			variables?: GetRegionsQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<GetRegionsQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<GetRegionsQuery>(GetRegionsDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'getRegions',
				'query'
			);
		},
		search(
			variables: SearchQueryVariables,
			requestHeaders?: GraphQLClientRequestHeaders
		): Promise<SearchQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<SearchQuery>(SearchDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'search',
				'query'
			);
		}
	};
}
export type Sdk = ReturnType<typeof getSdk>;
