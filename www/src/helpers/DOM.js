export const clickOutside = (node) => {
  const handleClick = () => {
    const { target, defaultPrevented } = event;

    if (node && !node.contains(target) && !defaultPrevented) {
      const clickOutsideEvent = new CustomEvent('clickOutside', node);
      node.dispatchEvent(clickOutsideEvent);
    }
  }

  document.addEventListener('click', handleClick, true);

  return {
    destroy() {
      document.removeEventListener('click', handleClick, true);
    },
  };
};
