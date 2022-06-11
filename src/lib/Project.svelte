<script lang="ts">
  import type Project from "src/models/project"

  export let project: Project
</script>

<div class="card">
  <img class="logo" src={project.logo.thumbnailUrl} alt="" />
  <p class="title">{project.name}</p>
</div>

<style>
  .card {
    background-color: rgb(28, 28, 28);
    display: grid;
    grid-auto-columns: minmax(min-content, max-content);

    padding: 10px;
  }

  .logo {
    width: 8em;
    height: 8em;
    grid-row: 0;
    grid-column: 0;
    grid-column-end: 1;
    padding: 5px;
  }

  .title {
    grid-row: 0;
    grid-column: 1;
    font-size: 28px;
    margin: 0 10px;
  }
</style>
