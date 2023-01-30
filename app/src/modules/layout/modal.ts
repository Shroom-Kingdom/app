import type { SvelteComponentTyped } from 'svelte/internal';
import { get, writable } from 'svelte/store';

import { ScreenSize, screenSize$ } from './screen-size';

export enum ModalSize {
  Small,
  Medium,
  Large
}

export interface ModalDimension {
  width: string;
  maxHeight: string;
}

export const MODAL_DIMENSIONS = {
  [ModalSize.Small]: () => {
    switch (get(screenSize$)) {
      case ScreenSize.Phone:
        return {
          width: '400px',
          maxHeight: '100vh'
        };
      case ScreenSize.Mobile:
        return {
          width: '400px',
          maxHeight: '80vh'
        };
      default:
        return {
          width: '400px',
          maxHeight: '60vh'
        };
    }
  },
  [ModalSize.Medium]: () => {
    switch (get(screenSize$)) {
      case ScreenSize.Phone:
        return {
          width: '600px',
          maxHeight: '100vh'
        };
      case ScreenSize.Mobile:
        return {
          width: '600px',
          maxHeight: '85vh'
        };
      default:
        return {
          width: '600px',
          maxHeight: '70vh'
        };
    }
  },
  [ModalSize.Large]: () => {
    switch (get(screenSize$)) {
      case ScreenSize.Phone:
        return {
          width: '800px',
          maxHeight: '100vh'
        };
      case ScreenSize.Mobile:
        return {
          width: '800px',
          maxHeight: '90vh'
        };
      default:
        return {
          width: '800px',
          maxHeight: '80vh'
        };
    }
  }
} satisfies Record<ModalSize, () => ModalDimension>;

export const modal$ = writable<SvelteComponentTyped | null>(null);
export const modalSize$ = writable<ModalSize>(ModalSize.Medium);

modalSize$.subscribe(modalSize => {
  setModalDimension(modalSize);
});

screenSize$.subscribe(() => {
  setModalDimension(get(modalSize$));
});

function setModalDimension(modalSize: ModalSize) {
  const root = document.querySelector(':root') as HTMLElement;
  root.style.setProperty('--modal-width', MODAL_DIMENSIONS[modalSize]().width);
  root.style.setProperty(
    '--modal-max-height',
    MODAL_DIMENSIONS[modalSize]().maxHeight
  );
}
