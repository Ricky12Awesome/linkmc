<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import Project from "./lib/Project.svelte"

  const proj = () => invoke("test").then(it => it as Project)
</script>

<main>
  {#await proj() then project}
    <Project {project} />
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
