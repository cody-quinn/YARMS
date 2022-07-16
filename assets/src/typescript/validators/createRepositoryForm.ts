export const nameValidator = (value: string) => {
  if (value.length == 0) {
    return "";
  }

  if (value.length > 64) {
    return "Name may not exceed 64 characters in length";
  }

  if (value.includes(" ")) {
    return "Name can not contain any spaces";
  }

  const re = /^[a-zA-Z0-9_-]+$/;
  if (value.match(re) == null) {
    return "Name must contain only alphanumeric, '-' and '_' characters";
  }

  return "";
};
