<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import Navbar from "./lib/Navbar.svelte"
  import ProjectComponent from "./lib/Project.svelte"
  import type Project from "./types/project"

  const proj = () =>
    invoke("test").then((it) => {
      console.log(it)
      return it as Project
    })
</script>

<main>
  <Navbar />
  {#await proj() then project}
    <ProjectComponent {project} directory="/this/is/a/test/directory" />
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
