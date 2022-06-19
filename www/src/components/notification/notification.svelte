<script>
  export let title;
  export let isSuccess = true;
  export let isInfo = false;
  export let isError = false;
  export let isOpen = false;

  const close = () => isOpen = false;
</script>

<article class={`message is-primary ${$$props.class}`} class:is-success={isSuccess} class:is-info={isInfo} class:is-danger={isError} class:is-hidden={!isOpen}>
  <div class="message-header">
    <p>{title}</p>
    <button class="delete" on:click={close}></button>
  </div>
  <div class="message-body">
    <slot></slot>
  </div>
</article>

<style lang="scss">
  .message {
    margin: 10px 0;
  }

  .message-body {
    padding: 15px;
  }
</style>
