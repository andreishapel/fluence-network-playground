<script>
  import { getFormData } from '@helpers/form';

  export let onSubmit;

  const submit = (event) => {
    const { target } = event;

    const formData = getFormData(target);
    onSubmit(formData);
  };
</script>

<form class={$$props.class} target="_self" on:submit|preventDefault={submit}>
  <slot></slot>
</form>
