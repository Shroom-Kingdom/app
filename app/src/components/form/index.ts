export type HtmlInputEvent = Event & {
  currentTarget: EventTarget & HTMLInputElement;
};
