<script>
  let columns = [];
  let rows = [[]];
</script>

<table class="table is-fullwidth">
  <thead>
    <tr>
      {#each columns as column}
        <th>{column}</th>
      {/each}
    </tr>
  </thead>
  <tbody>
    {#each rows as row}
      <tr>
        {#each row as item}
          <td>{item}</td>
        {/each}
      </tr>
    {/each}
  </tbody>
</table>
