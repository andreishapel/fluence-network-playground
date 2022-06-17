export const getFormData = (target) => {
  const formData = new FormData(target);

  const data = {};
  for (const formEntity of formData) {
    const [key, value] = formEntity;
    data[key] = value;
  }

  return data;
};
