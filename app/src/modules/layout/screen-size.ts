import { derived, writable } from 'svelte/store';

export enum ScreenSize {
  Phone = 480,
  Mobile = 768,
  Laptop = 1240,
  DesktopLg = 1800,
  DesktopXl = 100_000
}

export const screenSize$ = writable<ScreenSize>(ScreenSize.Laptop);

export const widthAtMost$ = (width: ScreenSize) =>
  derived(screenSize$, screenSize => {
    return width <= screenSize;
  });

export const widthAtLeast$ = (width: ScreenSize) =>
  derived(screenSize$, screenSize => {
    return width >= screenSize;
  });
