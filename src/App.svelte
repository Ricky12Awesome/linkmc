<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import ProjectComponent from "./lib/Project.svelte"
  import type Project from "./types/project"

  const proj = () => invoke("test").then((it) => it as Project)
</script>

<main>
  {#await proj() then project}
    <ProjectComponent {project} />
  {/await}
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;

    background-color: black;
    color: white;
  }
</style>
