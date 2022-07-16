import { bindFormElement, bindInnerText } from "./bind";

export type validator = (input: string) => string;

export const addInputValidator = <T extends HTMLInputElement, E extends HTMLElement>(
  element: T,
  errorElement: E,
  validator: validator
) => {
  let [getValue] = bindFormElement(element);
  let [_, setError] = bindInnerText(errorElement);

  element.addEventListener("input", (ev) => setError(validator(getValue())));
  setError(validator(getValue()));
};
