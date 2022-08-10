export const queryChildren = <P extends Element, T extends Element>(parent: P, selector: string): T =>
  parent.querySelector(selector);

export const query = <T extends Element>(selector: string): T => document.querySelector(selector);

export const queryFormRadiosByName = (name: string): HTMLInputElement[] =>
  Array.from(document.getElementsByTagName("input"))
    .filter((el) => el.type == "radio")
    .filter((el) => el.name == name);
