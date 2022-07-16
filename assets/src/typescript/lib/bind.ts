export type bindedValue<T, V> = [() => V, (value: V) => void, T];

export const bindFormElement = <T extends HTMLInputElement>(element: T): bindedValue<T, string> => {
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
