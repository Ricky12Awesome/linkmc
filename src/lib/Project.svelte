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
    <button class="button"> Curseforge </button>
  </div>
</div>

<style>
  .card {
    background-color: hsl(0, 0%, 10%);
    display: flex;
  }

  .logo {
    width: 8em;
    height: 8em;
    margin: 5px;
  }

  .body {
    display: flexbox;
  }

  .title {
    font-size: 32px;
    margin: 0px 5px;
  }

  .description {
    font-size: 16px;
    margin: 5px 5px;
  }

  .button {
    border: none;
    border-radius: 5px;
    font-size: 24px;
    margin: 5px 5px;
  }

  .button:enabled {
    background-color: hsl(0, 0%, 90%);
  }

  .button:hover {
    background-color: hsl(0, 0%, 75%);
  }

  .button:active {
    background-color: hsl(0, 0%, 25%);
  }
</style>
