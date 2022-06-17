<script>
  export let title;
  export let size = 'large';
</script>

<section class={`block ${$$props.class} ${size}`}>
  <header class="header is-size-4">
    {title}
  </header>
  <slot></slot>
</section>

<style lang="scss">
  .block {
    margin: 20px auto 40px auto;

    &.large {
      width: 675px;
    }

    &.medium {
      width: 450px;
    }
  }

  .header {
    text-align: center;
  }
</style>
