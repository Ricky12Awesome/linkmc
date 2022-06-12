<script lang="ts">
  import type Project from "../types/project"

  export let project: Project
  export let directory: string
</script>

<div class="card">
  <img class="logo" src={project.logo.thumbnailUrl} alt="" />
  <div class="body">
    <p class="title">{project.name}</p>
    <p class="description">{project.summary}</p>
  </div>
  <img class="logo" src="" alt="" />
</div>

<style>
  .card {
    background-color: hsl(0, 0%, 10%);
    display: flex;
  }

  .logo {
    width: 8rem;
    height: 8rem;
    margin: 5px;
  }

  .body {
    display: flexbox;
    flex-grow: 1;
  }

  .title {
    font-size: 32px;
    margin: 0px 5px;
  }

  .description {
    font-size: 16px;
    margin: 5px 5px;
  }
</style>
