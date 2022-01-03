export function isEmpty(val) {
  return val.trim().length === 0;
}

export function isValidEmail(email) {
  const regex = /^((?!\.)[\w\-_.]*[^.])(@\w+)(\.\w+(\.\w+)?[^.\W])$/;
  return !regex.test(email);
}
