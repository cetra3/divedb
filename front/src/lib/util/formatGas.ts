import type { GasOutput  } from "$lib/graphql/generated";

export default (gas: GasOutput) => {
  if (gas.o2 == 21. && gas.he == 0.) {
    return "Air"
  } else if (gas.he == 0.) {
    return `EAN${Math.trunc(gas.o2)}`
  } else {
    return `${Math.trunc(gas.o2)}%/${Math.trunc(gas.he)}%`
  }
};
