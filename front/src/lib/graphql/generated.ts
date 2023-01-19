import type { GraphQLClient } from 'graphql-request';
import type * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
	ID: string;
	String: string;
	Boolean: boolean;
	Int: number;
	Float: number;
	/**
	 * Implement the DateTime<Utc> scalar
	 *
	 * The input/output is a string in RFC3339 format.
	 */
	DateTime: any;
	/** A scalar that can represent any JSON Object value. */
	JSONObject: any;
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
	UUID: string;
};

export type Category = {
	__typename?: 'Category';
	id: Scalars['UUID'];
	name: Scalars['String'];
	values: Array<CategoryValue>;
};

export type CategoryValue = {
	__typename?: 'CategoryValue';
	categoryId: Scalars['UUID'];
	id: Scalars['UUID'];
	value: Scalars['String'];
};

export type CreateCategory = {
	id?: InputMaybe<Scalars['UUID']>;
	name: Scalars['String'];
};

export type CreateCategoryValue = {
	categoryId: Scalars['UUID'];
	id?: InputMaybe<Scalars['UUID']>;
	value: Scalars['String'];
};

export type CreateDive = {
	date: Scalars['DateTime'];
	depth: Scalars['Float'];
	diveSiteId?: InputMaybe<Scalars['UUID']>;
	duration: Scalars['Int'];
	id?: InputMaybe<Scalars['UUID']>;
};

export type CreateDiveSite = {
	access: Scalars['String'];
	depth: Scalars['Float'];
	description: Scalars['String'];
	difficulty: Difficulty;
	id?: InputMaybe<Scalars['UUID']>;
	lat: Scalars['Float'];
	lon: Scalars['Float'];
	name: Scalars['String'];
	photoId?: InputMaybe<Scalars['UUID']>;
	published: Scalars['Boolean'];
};

export type CreatePhoto = {
	date: Scalars['DateTime'];
	diveId?: InputMaybe<Scalars['UUID']>;
	diveSiteId?: InputMaybe<Scalars['UUID']>;
	filename: Scalars['String'];
	id?: InputMaybe<Scalars['UUID']>;
	sealifeId?: InputMaybe<Scalars['UUID']>;
	size: Scalars['Int'];
	userId: Scalars['UUID'];
};

export type CreateRegion = {
	id?: InputMaybe<Scalars['UUID']>;
	latMax: Scalars['Float'];
	latMin: Scalars['Float'];
	lonMax: Scalars['Float'];
	lonMin: Scalars['Float'];
	name: Scalars['String'];
};

export type CreateSealife = {
	categoryMap?: InputMaybe<Scalars['JSONObject']>;
	description: Scalars['String'];
	hideLocation: Scalars['Boolean'];
	id?: InputMaybe<Scalars['UUID']>;
	name: Scalars['String'];
	photoId?: InputMaybe<Scalars['UUID']>;
	scientificName?: InputMaybe<Scalars['String']>;
};

export enum Difficulty {
	Aow = 'AOW',
	Ow = 'OW',
	Tech = 'TECH'
}

export type Dive = {
	__typename?: 'Dive';
	date: Scalars['DateTime'];
	depth: Scalars['Float'];
	diveSite?: Maybe<DiveSite>;
	diveSiteId?: Maybe<Scalars['UUID']>;
	duration: Scalars['Int'];
	hasMetrics: Scalars['Boolean'];
	id: Scalars['UUID'];
	latestPhotos: Array<Photo>;
	number: Scalars['Int'];
	userId: Scalars['UUID'];
};

export type DiveSite = {
	__typename?: 'DiveSite';
	access: Scalars['String'];
	date: Scalars['DateTime'];
	depth: Scalars['Float'];
	description: Scalars['String'];
	difficulty: Difficulty;
	id: Scalars['UUID'];
	lat: Scalars['Float'];
	latestPhotos: Array<Photo>;
	lon: Scalars['Float'];
	name: Scalars['String'];
	photo?: Maybe<Photo>;
	photoId?: Maybe<Scalars['UUID']>;
	published: Scalars['Boolean'];
	references: Array<OgReference>;
	siteMetrics: SiteMetric;
	slug?: Maybe<Scalars['String']>;
	summary: Scalars['String'];
	userId?: Maybe<Scalars['UUID']>;
};

export type Feedback = {
	__typename?: 'Feedback';
	date: Scalars['DateTime'];
	feedback: Scalars['String'];
	id: Scalars['UUID'];
	userId: Scalars['UUID'];
};

export type LoginResponse = {
	__typename?: 'LoginResponse';
	copyrightLocation?: Maybe<OverlayLocation>;
	email: Scalars['String'];
	id: Scalars['UUID'];
	level: UserLevel;
	token: Scalars['String'];
	username?: Maybe<Scalars['String']>;
	watermarkLocation: OverlayLocation;
};

export type Mutation = {
	__typename?: 'Mutation';
	addFeedback: Feedback;
	changePassword: Scalars['Boolean'];
	checkReference: OgReference;
	deleteUser: Scalars['Boolean'];
	fbLogin: LoginResponse;
	fbRegisterUser: LoginResponse;
	login: LoginResponse;
	mergeDiveSites: Scalars['Boolean'];
	newCategory: Category;
	newCategoryValue: CategoryValue;
	newDive: Dive;
	newDiveSite: DiveSite;
	newReference: OgReference;
	newRegion: Region;
	newSealife: Sealife;
	registerUser: LoginResponse;
	removeCategory: Scalars['Boolean'];
	removeCategoryValue: Scalars['Boolean'];
	removeDive: Scalars['Boolean'];
	removeDiveSite: Scalars['Boolean'];
	removePhoto: Scalars['Boolean'];
	removeReference: Scalars['Boolean'];
	removeRegion: Scalars['Boolean'];
	removeSealife: Scalars['Boolean'];
	requestResetToken: Scalars['Boolean'];
	resetPassword: LoginResponse;
	syncSubsurface: Scalars['Boolean'];
	updatePhoto: Photo;
	updateSettings?: Maybe<LoginResponse>;
};

export type MutationAddFeedbackArgs = {
	feedback: Scalars['String'];
};

export type MutationChangePasswordArgs = {
	newPassword: Scalars['String'];
	oldPassword: Scalars['String'];
};

export type MutationCheckReferenceArgs = {
	url: Scalars['String'];
};

export type MutationDeleteUserArgs = {
	password: Scalars['String'];
};

export type MutationFbLoginArgs = {
	code: Scalars['String'];
	redirectUri: Scalars['String'];
};

export type MutationFbRegisterUserArgs = {
	code: Scalars['String'];
	redirectUri: Scalars['String'];
};

export type MutationLoginArgs = {
	email: Scalars['String'];
	password: Scalars['String'];
};

export type MutationMergeDiveSitesArgs = {
	fromId: Scalars['UUID'];
	toId: Scalars['UUID'];
};

export type MutationNewCategoryArgs = {
	category: CreateCategory;
};

export type MutationNewCategoryValueArgs = {
	categoryValue: CreateCategoryValue;
};

export type MutationNewDiveArgs = {
	dive: CreateDive;
};

export type MutationNewDiveSiteArgs = {
	site: CreateDiveSite;
};

export type MutationNewReferenceArgs = {
	diveSiteId?: InputMaybe<Scalars['UUID']>;
	sealifeId?: InputMaybe<Scalars['UUID']>;
	url: Scalars['String'];
};

export type MutationNewRegionArgs = {
	region: CreateRegion;
};

export type MutationNewSealifeArgs = {
	sealife: CreateSealife;
};

export type MutationRegisterUserArgs = {
	email: Scalars['String'];
	password: Scalars['String'];
};

export type MutationRemoveCategoryArgs = {
	categoryId: Scalars['UUID'];
};

export type MutationRemoveCategoryValueArgs = {
	categoryValueId: Scalars['UUID'];
};

export type MutationRemoveDiveArgs = {
	id: Scalars['UUID'];
};

export type MutationRemoveDiveSiteArgs = {
	id: Scalars['UUID'];
};

export type MutationRemovePhotoArgs = {
	id: Scalars['UUID'];
};

export type MutationRemoveReferenceArgs = {
	id: Scalars['UUID'];
};

export type MutationRemoveRegionArgs = {
	id: Scalars['UUID'];
};

export type MutationRemoveSealifeArgs = {
	id: Scalars['UUID'];
};

export type MutationRequestResetTokenArgs = {
	email: Scalars['String'];
};

export type MutationResetPasswordArgs = {
	email: Scalars['String'];
	newPassword: Scalars['String'];
	token: Scalars['UUID'];
};

export type MutationSyncSubsurfaceArgs = {
	email: Scalars['String'];
	password: Scalars['String'];
};

export type MutationUpdatePhotoArgs = {
	photo: CreatePhoto;
};

export type MutationUpdateSettingsArgs = {
	copyrightLocation?: InputMaybe<OverlayLocation>;
	username?: InputMaybe<Scalars['String']>;
	watermarkLocation: OverlayLocation;
};

export type OgReference = {
	__typename?: 'OgReference';
	description: Scalars['String'];
	id: Scalars['UUID'];
	imageUrl?: Maybe<Scalars['String']>;
	lastFetched: Scalars['DateTime'];
	title: Scalars['String'];
	url: Scalars['String'];
};

export enum OverlayLocation {
	BottomLeft = 'BOTTOM_LEFT',
	BottomRight = 'BOTTOM_RIGHT',
	TopLeft = 'TOP_LEFT',
	TopRight = 'TOP_RIGHT'
}

export type Photo = {
	__typename?: 'Photo';
	date?: Maybe<Scalars['DateTime']>;
	dive?: Maybe<Dive>;
	diveSite?: Maybe<DiveSite>;
	filename: Scalars['String'];
	height: Scalars['Float'];
	id: Scalars['UUID'];
	sealife?: Maybe<Sealife>;
	size: Scalars['Float'];
	userId: Scalars['UUID'];
	width: Scalars['Float'];
};

export type Query = {
	__typename?: 'Query';
	categories: Array<Category>;
	categoryValues: Array<CategoryValue>;
	currentUser?: Maybe<LoginResponse>;
	diveSites: Array<DiveSite>;
	dives: Array<Dive>;
	fbAppId: Scalars['String'];
	photos: Array<Photo>;
	popularDiveSites: Array<DiveSite>;
	regions: Array<Region>;
	sealife: Array<Sealife>;
	search: Array<SearchResult>;
	siteUrl: Scalars['String'];
};

export type QueryDiveSitesArgs = {
	id?: InputMaybe<Scalars['UUID']>;
	maxDepth?: InputMaybe<Scalars['Float']>;
	name?: InputMaybe<Scalars['String']>;
	slug?: InputMaybe<Scalars['String']>;
};

export type QueryDivesArgs = {
	id?: InputMaybe<Scalars['UUID']>;
	maxDepth?: InputMaybe<Scalars['Float']>;
};

export type QueryPhotosArgs = {
	dive?: InputMaybe<Scalars['UUID']>;
	diveSite?: InputMaybe<Scalars['UUID']>;
	id?: InputMaybe<Scalars['UUID']>;
	offset?: InputMaybe<Scalars['Int']>;
	orderByUpload?: InputMaybe<Scalars['Boolean']>;
	sealifeId?: InputMaybe<Scalars['UUID']>;
	userId?: InputMaybe<Scalars['UUID']>;
};

export type QuerySealifeArgs = {
	categoryValues?: InputMaybe<Array<Scalars['UUID']>>;
	id?: InputMaybe<Scalars['UUID']>;
	name?: InputMaybe<Scalars['String']>;
	scientificName?: InputMaybe<Scalars['String']>;
	slug?: InputMaybe<Scalars['String']>;
};

export type QuerySearchArgs = {
	query: Scalars['String'];
};

export type Region = {
	__typename?: 'Region';
	id: Scalars['UUID'];
	latMax: Scalars['Float'];
	latMin: Scalars['Float'];
	lonMax: Scalars['Float'];
	lonMin: Scalars['Float'];
	name: Scalars['String'];
	slug: Scalars['String'];
};

export type Sealife = {
	__typename?: 'Sealife';
	categoryMap: Scalars['JSONObject'];
	date: Scalars['DateTime'];
	description: Scalars['String'];
	hideLocation: Scalars['Boolean'];
	id: Scalars['UUID'];
	latestPhotos: Array<Photo>;
	name: Scalars['String'];
	photo?: Maybe<Photo>;
	photoId?: Maybe<Scalars['UUID']>;
	references: Array<OgReference>;
	scientificName?: Maybe<Scalars['String']>;
	slug?: Maybe<Scalars['String']>;
	summary: Scalars['String'];
};

export type SearchResult = {
	__typename?: 'SearchResult';
	id: Scalars['String'];
	kind: SearchResultKind;
	name: Scalars['String'];
	photoId?: Maybe<Scalars['String']>;
	scientificName?: Maybe<Scalars['String']>;
	slug: Scalars['String'];
	summary: Scalars['String'];
};

export enum SearchResultKind {
	DiveSite = 'DIVE_SITE',
	Sealife = 'SEALIFE'
}

export type SiteMetric = {
	__typename?: 'SiteMetric';
	diveCount: Scalars['Int'];
	photoCount: Scalars['Int'];
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

export type DiveSummaryFragment = {
	__typename?: 'Dive';
	id: string;
	date: any;
	duration: number;
	depth: number;
	diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
};

export type DiveWithMetricsFragment = {
	__typename?: 'Dive';
	id: string;
	userId: string;
	date: any;
	depth: number;
	duration: number;
	hasMetrics: boolean;
	diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
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
	diveSiteId?: string | null;
	latestPhotos: Array<{
		__typename?: 'Photo';
		id: string;
		userId: string;
		filename: string;
		date?: any | null;
		size: number;
		width: number;
		height: number;
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			duration: number;
			depth: number;
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
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			duration: number;
			depth: number;
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
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			duration: number;
			depth: number;
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

export type PhotoSummaryFragment = {
	__typename?: 'Photo';
	id: string;
	userId: string;
	filename: string;
	date?: any | null;
	size: number;
	width: number;
	height: number;
	dive?: {
		__typename?: 'Dive';
		id: string;
		date: any;
		duration: number;
		depth: number;
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
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			duration: number;
			depth: number;
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
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			duration: number;
			depth: number;
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
	username?: string | null;
	watermarkLocation: OverlayLocation;
	copyrightLocation?: OverlayLocation | null;
};

export type CurrentUserTokenFragment = {
	__typename?: 'LoginResponse';
	id: string;
	email: string;
	level: UserLevel;
	token: string;
};

export type AddDiveMutationVariables = Exact<{
	dive: CreateDive;
}>;

export type AddDiveMutation = {
	__typename?: 'Mutation';
	newDive: {
		__typename?: 'Dive';
		id: string;
		date: any;
		duration: number;
		depth: number;
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
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				duration: number;
				depth: number;
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
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				duration: number;
				depth: number;
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
	feedback: Scalars['String'];
}>;

export type AddFeedbackMutation = {
	__typename?: 'Mutation';
	addFeedback: { __typename?: 'Feedback'; id: string };
};

export type NewReferenceMutationVariables = Exact<{
	url: Scalars['String'];
	sealifeId?: InputMaybe<Scalars['UUID']>;
	diveSiteId?: InputMaybe<Scalars['UUID']>;
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
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				duration: number;
				depth: number;
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
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				duration: number;
				depth: number;
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

export type DeleteUserMutationVariables = Exact<{
	password: Scalars['String'];
}>;

export type DeleteUserMutation = { __typename?: 'Mutation'; deleteUser: boolean };

export type FbLoginUserMutationVariables = Exact<{
	redirectUri: Scalars['String'];
	code: Scalars['String'];
}>;

export type FbLoginUserMutation = {
	__typename?: 'Mutation';
	fbLogin: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		level: UserLevel;
		token: string;
	};
};

export type FbRegisterUserMutationVariables = Exact<{
	redirectUri: Scalars['String'];
	code: Scalars['String'];
}>;

export type FbRegisterUserMutation = {
	__typename?: 'Mutation';
	fbRegisterUser: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		level: UserLevel;
		token: string;
	};
};

export type LoginUserMutationVariables = Exact<{
	email: Scalars['String'];
	password: Scalars['String'];
}>;

export type LoginUserMutation = {
	__typename?: 'Mutation';
	login: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		level: UserLevel;
		token: string;
	};
};

export type UpdateSettingsMutationVariables = Exact<{
	username?: InputMaybe<Scalars['String']>;
	watermarkLocation: OverlayLocation;
	copyrightLocation?: InputMaybe<OverlayLocation>;
}>;

export type UpdateSettingsMutation = {
	__typename?: 'Mutation';
	updateSettings?: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		level: UserLevel;
		username?: string | null;
		watermarkLocation: OverlayLocation;
		copyrightLocation?: OverlayLocation | null;
	} | null;
};

export type MergeDiveSitesMutationVariables = Exact<{
	fromId: Scalars['UUID'];
	toId: Scalars['UUID'];
}>;

export type MergeDiveSitesMutation = { __typename?: 'Mutation'; mergeDiveSites: boolean };

export type RequestResetTokenMutationVariables = Exact<{
	email: Scalars['String'];
}>;

export type RequestResetTokenMutation = { __typename?: 'Mutation'; requestResetToken: boolean };

export type ResetPasswordMutationVariables = Exact<{
	email: Scalars['String'];
	newPassword: Scalars['String'];
	token: Scalars['UUID'];
}>;

export type ResetPasswordMutation = {
	__typename?: 'Mutation';
	resetPassword: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		level: UserLevel;
		token: string;
	};
};

export type ChangePasswordMutationVariables = Exact<{
	oldPassword: Scalars['String'];
	newPassword: Scalars['String'];
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
	id: Scalars['UUID'];
}>;

export type RemoveRegionMutation = { __typename?: 'Mutation'; removeRegion: boolean };

export type RegisterUserMutationVariables = Exact<{
	email: Scalars['String'];
	password: Scalars['String'];
}>;

export type RegisterUserMutation = {
	__typename?: 'Mutation';
	registerUser: {
		__typename?: 'LoginResponse';
		id: string;
		email: string;
		level: UserLevel;
		token: string;
	};
};

export type RemoveDiveMutationVariables = Exact<{
	id: Scalars['UUID'];
}>;

export type RemoveDiveMutation = { __typename?: 'Mutation'; removeDive: boolean };

export type RemoveDiveSiteMutationVariables = Exact<{
	id: Scalars['UUID'];
}>;

export type RemoveDiveSiteMutation = { __typename?: 'Mutation'; removeDiveSite: boolean };

export type RemovePhotoMutationVariables = Exact<{
	id: Scalars['UUID'];
}>;

export type RemovePhotoMutation = { __typename?: 'Mutation'; removePhoto: boolean };

export type RemoveReferenceMutationVariables = Exact<{
	id: Scalars['UUID'];
}>;

export type RemoveReferenceMutation = { __typename?: 'Mutation'; removeReference: boolean };

export type RemoveSealifeMutationVariables = Exact<{
	id: Scalars['UUID'];
}>;

export type RemoveSealifeMutation = { __typename?: 'Mutation'; removeSealife: boolean };

export type SyncSubsurfaceMutationVariables = Exact<{
	email: Scalars['String'];
	password: Scalars['String'];
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
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			duration: number;
			depth: number;
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
		username?: string | null;
		watermarkLocation: OverlayLocation;
		copyrightLocation?: OverlayLocation | null;
	} | null;
};

export type FbAppIdQueryVariables = Exact<{ [key: string]: never }>;

export type FbAppIdQuery = { __typename?: 'Query'; fbAppId: string };

export type GetDiveQueryVariables = Exact<{
	id: Scalars['UUID'];
}>;

export type GetDiveQuery = {
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
		diveSiteId?: string | null;
		latestPhotos: Array<{
			__typename?: 'Photo';
			id: string;
			userId: string;
			filename: string;
			date?: any | null;
			size: number;
			width: number;
			height: number;
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				duration: number;
				depth: number;
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
		}>;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
	}>;
};

export type GetDiveSitesQueryVariables = Exact<{
	id?: InputMaybe<Scalars['UUID']>;
	name?: InputMaybe<Scalars['String']>;
	maxDepth?: InputMaybe<Scalars['Float']>;
	slug?: InputMaybe<Scalars['String']>;
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
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				duration: number;
				depth: number;
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
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				duration: number;
				depth: number;
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
	id?: InputMaybe<Scalars['UUID']>;
	name?: InputMaybe<Scalars['String']>;
	maxDepth?: InputMaybe<Scalars['Float']>;
	slug?: InputMaybe<Scalars['String']>;
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

export type PopularDiveSitesQueryVariables = Exact<{ [key: string]: never }>;

export type PopularDiveSitesQuery = {
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
};

export type GetDivesQueryVariables = Exact<{ [key: string]: never }>;

export type GetDivesQuery = {
	__typename?: 'Query';
	dives: Array<{
		__typename?: 'Dive';
		id: string;
		userId: string;
		date: any;
		depth: number;
		duration: number;
		hasMetrics: boolean;
		diveSite?: { __typename?: 'DiveSite'; name: string; id: string; slug?: string | null } | null;
	}>;
};

export type GetPhotosQueryVariables = Exact<{
	id?: InputMaybe<Scalars['UUID']>;
	userId?: InputMaybe<Scalars['UUID']>;
	diveSite?: InputMaybe<Scalars['UUID']>;
	dive?: InputMaybe<Scalars['UUID']>;
	sealifeId?: InputMaybe<Scalars['UUID']>;
	offset?: InputMaybe<Scalars['Int']>;
	orderByUpload?: InputMaybe<Scalars['Boolean']>;
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
		dive?: {
			__typename?: 'Dive';
			id: string;
			date: any;
			duration: number;
			depth: number;
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
	}>;
};

export type GetSealifeSummaryQueryVariables = Exact<{
	id?: InputMaybe<Scalars['UUID']>;
	name?: InputMaybe<Scalars['String']>;
	scientificName?: InputMaybe<Scalars['String']>;
	slug?: InputMaybe<Scalars['String']>;
	categoryValues?: InputMaybe<Array<Scalars['UUID']> | Scalars['UUID']>;
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
	id?: InputMaybe<Scalars['UUID']>;
	name?: InputMaybe<Scalars['String']>;
	scientificName?: InputMaybe<Scalars['String']>;
	slug?: InputMaybe<Scalars['String']>;
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
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				duration: number;
				depth: number;
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
			dive?: {
				__typename?: 'Dive';
				id: string;
				date: any;
				duration: number;
				depth: number;
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
	query: Scalars['String'];
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
export const SiteSummaryFragmentDoc = gql`
	fragment SiteSummary on DiveSite {
		name
		id
		slug
	}
`;
export const DiveWithMetricsFragmentDoc = gql`
	fragment DiveWithMetrics on Dive {
		id
		userId
		date
		depth
		duration
		hasMetrics
		diveSite {
			...SiteSummary
		}
	}
`;
export const DiveSummaryFragmentDoc = gql`
	fragment DiveSummary on Dive {
		id
		date
		duration
		depth
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
		dive {
			...DiveSummary
		}
		diveSite {
			...SiteSummary
		}
		sealife {
			...SealifeSummary
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
		watermarkLocation
		copyrightLocation
	}
`;
export const CurrentUserTokenFragmentDoc = gql`
	fragment CurrentUserToken on LoginResponse {
		id
		email
		level
		token
	}
`;
export const AddDiveDocument = gql`
	mutation addDive($dive: CreateDive!) {
		newDive(dive: $dive) {
			...DiveSummary
		}
	}
	${DiveSummaryFragmentDoc}
	${SiteSummaryFragmentDoc}
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
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
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
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
	${ReferenceFragmentDoc}
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
	mutation fbRegisterUser($redirectUri: String!, $code: String!) {
		fbRegisterUser(redirectUri: $redirectUri, code: $code) {
			...CurrentUserToken
		}
	}
	${CurrentUserTokenFragmentDoc}
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
		$username: String
		$watermarkLocation: OverlayLocation!
		$copyrightLocation: OverlayLocation
	) {
		updateSettings(
			username: $username
			watermarkLocation: $watermarkLocation
			copyrightLocation: $copyrightLocation
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
	mutation registerUser($email: String!, $password: String!) {
		registerUser(email: $email, password: $password) {
			...CurrentUserToken
		}
	}
	${CurrentUserTokenFragmentDoc}
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
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
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
export const GetDiveDocument = gql`
	query getDive($id: UUID!) {
		dives(id: $id) {
			...DiveNode
		}
	}
	${DiveNodeFragmentDoc}
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
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
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
export const PopularDiveSitesDocument = gql`
	query popularDiveSites {
		popularDiveSites {
			...SiteSummaryMetrics
		}
	}
	${SiteSummaryMetricsFragmentDoc}
	${SiteMetricNodeFragmentDoc}
`;
export const GetDivesDocument = gql`
	query getDives {
		dives {
			...DiveWithMetrics
		}
	}
	${DiveWithMetricsFragmentDoc}
	${SiteSummaryFragmentDoc}
`;
export const GetPhotosDocument = gql`
	query getPhotos(
		$id: UUID
		$userId: UUID
		$diveSite: UUID
		$dive: UUID
		$sealifeId: UUID
		$offset: Int
		$orderByUpload: Boolean
	) {
		photos(
			id: $id
			userId: $userId
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
	${SiteSummaryFragmentDoc}
	${SealifeSummaryFragmentDoc}
	${ReferenceFragmentDoc}
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
	query search($query: String!) {
		search(query: $query) {
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
		deleteUser(
			variables: DeleteUserMutationVariables,
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
		loginUser(
			variables: LoginUserMutationVariables,
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
		getCategories(
			variables?: GetCategoriesQueryVariables,
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
		getDive(
			variables: GetDiveQueryVariables,
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
		popularDiveSites(
			variables?: PopularDiveSitesQueryVariables,
			requestHeaders?: Dom.RequestInit['headers']
		): Promise<PopularDiveSitesQuery> {
			return withWrapper(
				(wrappedRequestHeaders) =>
					client.request<PopularDiveSitesQuery>(PopularDiveSitesDocument, variables, {
						...requestHeaders,
						...wrappedRequestHeaders
					}),
				'popularDiveSites',
				'query'
			);
		},
		getDives(
			variables?: GetDivesQueryVariables,
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
		getRegions(
			variables?: GetRegionsQueryVariables,
			requestHeaders?: Dom.RequestInit['headers']
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
			requestHeaders?: Dom.RequestInit['headers']
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
