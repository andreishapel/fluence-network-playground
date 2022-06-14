<script>
  export let networkStatus;
  export let handleConnect;
  export let handleDisconnect;

  $: isConnected = networkStatus.isConnected;
  $: title = (isConnected) ? 'Disconnect' : 'Connect';

  const handleClick = () => {
    if (isConnected) {
      handleDisconnect();
    } else {
      handleConnect();
    }
  };
</script>

<button class="button is-primary" class:is-active={!isConnected} class:is-danger={isConnected} on:click={handleClick}>
  {title}
</button>

<style lang="scss">
  .button {
    width: 100px;
  }
</style>
