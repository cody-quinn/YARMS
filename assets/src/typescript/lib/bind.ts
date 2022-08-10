import { queryFormRadiosByName } from "./query";

export type bindedValue<T, V> = [() => V, (value: V) => void, T];

export const bindFormElement = <T extends HTMLInputElement | HTMLSelectElement>(element: T): bindedValue<T, string> => {
  return [
    () => {
      return element.value;
    },
    (value: string) => {
      element.value = value;
    },
    element,
  ];
};

export const bindRadioGroup = (name: string): bindedValue<Array<HTMLInputElement>, string> => {
  const radios = queryFormRadiosByName(name);

  return [
    () => {
      return radios.filter((el) => el.checked)[0].value;
    },
    (value: string) => {
      radios.filter((el) => el.checked)[0].checked = false;
      radios.filter((el) => el.value == value)[0].checked = true;
    },
    radios,
  ];
};

export const bindFormButtonElement = <T extends HTMLInputElement>(element: T): bindedValue<T, boolean> => {
  return [
    () => {
      return element.checked;
    },
    (value: boolean) => {
      element.checked = value;
    },
    element,
  ];
};

export const bindInnerHTML = <T extends HTMLElement>(element: T): bindedValue<T, string> => {
  return [
    () => {
      return element.innerHTML;
    },
    (value: string) => {
      element.innerHTML = value;
    },
    element,
  ];
};

export const bindInnerText = <T extends HTMLElement>(element: T): bindedValue<T, string> => {
  return [
    () => {
      return element.innerText;
    },
    (value: string) => {
      element.innerText = value;
    },
    element,
  ];
};

export const bindAttribute = <T extends Element>(element: T, attributeName: string): bindedValue<T, string | null> => {
  return [
    () => {
      return element.getAttribute(attributeName);
    },
    (value: string | null) => {
      if (value === null) {
        element.removeAttribute(attributeName);
        return;
      }
      element.setAttribute(attributeName, value);
    },
    element,
  ];
};
