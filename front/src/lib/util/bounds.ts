import type { LatLngBoundsExpression, LatLngLiteral } from 'leaflet';

export interface Bounds {
	latMin: number;
	lonMin: number;
	latMax: number;
	lonMax: number;
}

export const centerVals = (bounds: LatLngBoundsExpression): number[] => {
	let { latMin, lonMin, latMax, lonMax } = fromLeaflet(bounds);

	return [latMin + (latMax - latMin) / 2, lonMin + (lonMax - lonMin) / 2];
};

export const fromLeaflet = (bounds: LatLngBoundsExpression): Bounds => {
	let latMin;
	let lonMin;
	let latMax;
	let lonMax;

	if (bounds instanceof Array) {
		latMin = bounds[0][0];
		lonMin = bounds[0][1];
		latMax = bounds[1][0];
		lonMax = bounds[1][1];
	} else {
		let ne: LatLngLiteral = (bounds as any)._northEast;
		let sw: LatLngLiteral = (bounds as any)._southWest;

		latMin = sw.lat;
		lonMin = sw.lng;
		latMax = ne.lat;
		lonMax = ne.lng;
	}

	return {
		latMin,
		lonMin,
		latMax,
		lonMax
	};
};
