/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
// renderWithRouter.js
import { render as rts } from "@testing-library/svelte";
import WrapRouter from "./wrapRouter.svelte";

/**
 * Test-render a component, that relies on some of svelte-navigator's
 * features, inside a Router.
 *
 * @param component The component you want to wrap in a Router
 * @param componentProps The props you want to pass to it
 * @param routerOptions Futher configuration (`onNavigate`,
 * `withRoute`, `initialPathname`)
 * @param options Options for testing library's `render` function
 */
export const render = (
  component,
  componentProps: any,
  routerOptions: any,
  options: any
) => rts(WrapRouter, { component, componentProps, ...routerOptions }, options);
